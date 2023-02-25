use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{Extension, Json, Router, ServiceExt};
use axum::debug_handler;
use axum::extract::Path;
use axum::headers::HeaderName;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum_server::service::SendService;
use axum_server::tls_rustls::RustlsConfig;
use axum_sessions::{SameSite, SessionLayer};
use axum_sessions::async_session::MemoryStore;
use axum_sessions::extractors::WritableSession;
use rand::{Rng, thread_rng};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;
use uuid::Uuid;
use webauthn_rs::{Webauthn, WebauthnBuilder};
use webauthn_rs::prelude::{
    CreationChallengeResponse, Passkey, PasskeyAuthentication, PasskeyRegistration,
    PublicKeyCredential, RegisterPublicKeyCredential,
};

mod webauth_glue;

pub(crate) struct UserData {
    pub(crate) name_to_id: HashMap<String, Uuid>,
    pub(crate) keys: HashMap<Uuid, Vec<Passkey>>,
}

impl UserData {
    pub(crate) fn new() -> Self {
        Self {
            name_to_id: HashMap::new(),
            keys: HashMap::new(),
        }
    }
}

pub struct WasmServer;

#[derive(Clone)]
pub(crate) struct ServerData {
    pub(crate) webauthn: Arc<Webauthn>,
    pub(crate) user_data: Arc<Mutex<UserData>>,
}

impl ServerData {
    pub(crate) fn new(rp_id: String, rp_origin: &Url) -> Self {
        let builder =
            WebauthnBuilder::new(rp_id.as_str(), rp_origin).expect("Invalid configuration");
        let webauthn = Arc::new(builder.build().expect("could no build webauthn"));
        let user_data = Arc::new(Mutex::new(UserData::new()));
        Self {
            webauthn,
            user_data,
        }
    }
}

impl WasmServer {
    pub fn serve_at<P: Into<PathBuf>, Addr: Into<SocketAddr>>(src: P, rp_id: String, addr: Addr) {
        let src = src.into();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let html_content = std::fs::read_to_string(src.join("index.html")).unwrap();
        let js_content = std::fs::read_to_string(src.join("app.js")).unwrap();
        let wasm_content = std::fs::read(src.join("app_bg.wasm")).unwrap();
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "mise_en_place=debug,tower_http=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        rt.block_on(async {
            let address = addr.into();
            let url = "https://".to_string() + rp_id.as_str();
            let rp_origin = Url::parse(url.as_str()).expect("Invalid URL");
            let server_data = ServerData::new("localhost".to_string(), &rp_origin);
            let tls_config = RustlsConfig::from_pem_file(
                "mise_en_place/src/wasm_server/cert.pem",
                "mise_en_place/src/wasm_server/key.pem",
            )
                .await
                .unwrap();
            let store = MemoryStore::new();
            let secret = thread_rng().gen::<[u8; 128]>(); // MUST be at least 64 bytes!
            let session_layer = SessionLayer::new(store, &secret)
                .with_cookie_name("webauthnrs")
                .with_same_site_policy(SameSite::Strict)
                .with_secure(true);
            let site_router = Router::new()
                .route("/app", get(move || async {
                    tracing::debug!("serving html");
                    Html(html_content)
                }))
                .route(
                    "/app.js",
                    get(|| async {
                        let mut response = js_content.into_response();
                        response.headers_mut().insert(
                            "content-type",
                            HeaderValue::from_static("application/javascript"),
                        );
                        tracing::debug!("serving js");
                        response
                    }),
                )
                .route(
                    "/app_bg.wasm",
                    get(|| async move {
                        let mut response = wasm_content.into_response();
                        response
                            .headers_mut()
                            .insert("content-type", HeaderValue::from_static("application/wasm"));
                        tracing::debug!("serving wasm");
                        response
                    }),
                )
                .fallback_service(ServeDir::new(src));
            let router = Router::new()
                .route("/register_start/:username", post(start_register))
                .route("/register_finish", post(finish_register))
                .route("/login_start/:username", post(start_authentication))
                .route("/login_finish", post(finish_authentication))
                .merge(site_router)
                .layer(
                    ServiceBuilder::new()
                        .layer(CompressionLayer::new().gzip(true))
                        .layer(TraceLayer::new_for_http())
                        .layer(SetResponseHeaderLayer::if_not_present(
                            HeaderName::from_static("cross-origin-opener-policy"),
                            HeaderValue::from_static("same-origin"),
                        ))
                        .layer(SetResponseHeaderLayer::if_not_present(
                            HeaderName::from_static("cross-origin-embedder-policy"),
                            HeaderValue::from_static("require-corp"),
                        ))
                        .layer(
                            CorsLayer::new()
                                .allow_methods([Method::GET, Method::POST])
                                .allow_origin(address.to_string().parse::<HeaderValue>().unwrap()),
                        )
                        .layer(Extension(server_data))
                        .layer(session_layer),
                );
            axum_server::bind_rustls(address, tls_config)
                .serve(router.into_make_service())
                .await
                .unwrap();
        });
    }
}

#[debug_handler]
async fn internal_server_error(e: std::io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("unhandled internal error: {}", e),
    )
}

async fn start_register(
    Extension(server_data): Extension<ServerData>,
    mut session: WritableSession,
    Path(username): Path<String>,
) -> Result<Json<CreationChallengeResponse>, &'static str> {
    let unique_user_id = {
        let guard = server_data.user_data.lock().await;
        guard
            .name_to_id
            .get(&username)
            .copied()
            .unwrap_or_else(Uuid::new_v4)
    };
    session.remove("reg_state");
    let exclude_credentials = {
        let users_guard = server_data.user_data.lock().await;
        users_guard
            .keys
            .get(&unique_user_id)
            .map(|keys| keys.iter().map(|sk| sk.cred_id().clone()).collect())
    };
    let res = match server_data.webauthn.start_passkey_registration(
        unique_user_id,
        &username,
        &username,
        exclude_credentials,
    ) {
        Ok((ccr, reg_state)) => {
            // Note that due to the session store in use being a server side memory store, this is
            // safe to store the reg_state into the session since it is not client controlled and
            // not open to replay attacks. If this was a cookie store, this would be UNSAFE.
            session
                .insert("reg_state", (username, unique_user_id, reg_state))
                .expect("Failed to insert");
            tracing::info!("Registration Successful!");
            Json(ccr)
        }
        Err(e) => {
            tracing::debug!("challenge_register -> {:?}", e);
            return Err("Unknown");
        }
    };
    Ok(res)
}

async fn finish_register(
    Extension(app_state): Extension<ServerData>,
    mut session: WritableSession,
    Json(reg): Json<RegisterPublicKeyCredential>,
) -> Result<impl IntoResponse, &'static str> {
    let (username, user_unique_id, reg_state): (String, Uuid, PasskeyRegistration) =
        session.get("reg_state").ok_or("Corrupt Session")?; //Corrupt Session

    session.remove("reg_state");

    let res = match app_state
        .webauthn
        .finish_passkey_registration(&reg, &reg_state)
    {
        Ok(sk) => {
            let mut users_guard = app_state.user_data.lock().await;

            //TODO: This is where we would store the credential in a db, or persist them in some other way.
            users_guard
                .keys
                .entry(user_unique_id)
                .and_modify(|keys| keys.push(sk.clone()))
                .or_insert_with(|| vec![sk.clone()]);

            users_guard.name_to_id.insert(username, user_unique_id);

            StatusCode::OK
        }
        Err(e) => {
            tracing::debug!("challenge_register -> {:?}", e);
            StatusCode::BAD_REQUEST
        }
    };
    Ok(res)
}

async fn start_authentication(
    Extension(app_state): Extension<ServerData>,
    mut session: WritableSession,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, &'static str> {
    tracing::info!("Start Authentication");
    // We get the username from the URL, but you could get this via form submission or
    // some other process.

    // Remove any previous authentication that may have occured from the session.
    session.remove("auth_state");

    // Get the set of keys that the user possesses
    let users_guard = app_state.user_data.lock().await;

    // Look up their unique id from the username
    let user_unique_id = users_guard
        .name_to_id
        .get(&username)
        .copied()
        .ok_or("User Not Found")?;

    let allow_credentials = users_guard
        .keys
        .get(&user_unique_id)
        .ok_or("User Has No Credentials")?;

    let res = match app_state
        .webauthn
        .start_passkey_authentication(allow_credentials)
    {
        Ok((rcr, auth_state)) => {
            // Drop the mutex to allow the mut borrows below to proceed
            drop(users_guard);

            // Note that due to the session store in use being a server side memory store, this is
            // safe to store the auth_state into the session since it is not client controlled and
            // not open to replay attacks. If this was a cookie store, this would be UNSAFE.
            session
                .insert("auth_state", (user_unique_id, auth_state))
                .expect("Failed to insert");
            Json(rcr)
        }
        Err(e) => {
            tracing::debug!("challenge_authenticate -> {:?}", e);
            return Err("Unknown");
        }
    };
    Ok(res)
}

async fn finish_authentication(
    Extension(app_state): Extension<ServerData>,
    mut session: WritableSession,
    Json(auth): Json<PublicKeyCredential>,
) -> Result<impl IntoResponse, &'static str> {
    let (user_unique_id, auth_state): (Uuid, PasskeyAuthentication) =
        session.get("auth_state").ok_or("CorruptSession")?;

    session.remove("auth_state");

    let res = match app_state
        .webauthn
        .finish_passkey_authentication(&auth, &auth_state)
    {
        Ok(auth_result) => {
            let mut users_guard = app_state.user_data.lock().await;

            // Update the credential counter, if possible.
            users_guard
                .keys
                .get_mut(&user_unique_id)
                .map(|keys| {
                    keys.iter_mut().for_each(|sk| {
                        // This will update the credential if it's the matching
                        // one. Otherwise it's ignored. That is why it is safe to
                        // iterate this over the full list.
                        sk.update_credential(&auth_result);
                    })
                })
                .ok_or("User Has No Credentials")?;
            StatusCode::OK
        }
        Err(e) => {
            tracing::debug!("challenge_register -> {:?}", e);
            StatusCode::BAD_REQUEST
        }
    };
    tracing::info!("Authentication Successful!");
    Ok(res)
}
