use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{Extension, Router, ServiceExt};
use axum::headers::HeaderName;
use axum::http::{HeaderValue, Method};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum_extra::middleware::option_layer;
use axum_server::service::SendService;
use axum_server::tls_rustls::RustlsConfig;
use axum_sessions::{SameSite, SessionLayer};
use axum_sessions::async_session::MemoryStore;
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
use webauthn_rs::prelude::Passkey;

mod client_side_webauthn;
mod server_side_webauthn;

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
    pub fn serve_at<P: Into<PathBuf>, Addr: Into<SocketAddr>>(
        src: P,
        rp_id: String,
        addr: Addr,
        authentication: bool,
    ) {
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
            let (server_data, session_layer, web_authn_router) = match authentication {
                true => {
                    let url = "https://".to_string() + rp_id.as_str();
                    let rp_origin = Url::parse(url.as_str()).expect("Invalid URL");
                    let server_data = ServerData::new("localhost".to_string(), &rp_origin);
                    let store = MemoryStore::new();
                    let secret = thread_rng().gen::<[u8; 128]>(); // MUST be at least 64 bytes!
                    let session_layer = SessionLayer::new(store, &secret)
                        .with_cookie_name("webauthnrs")
                        .with_same_site_policy(SameSite::Strict)
                        .with_secure(true);
                    let web_authn_router = Router::new()
                        .route(
                            "/register_start/:username",
                            post(server_side_webauthn::start_register),
                        )
                        .route(
                            "/register_finish",
                            post(server_side_webauthn::finish_register),
                        )
                        .route(
                            "/login_start/:username",
                            post(server_side_webauthn::start_authentication),
                        )
                        .route(
                            "/login_finish",
                            post(server_side_webauthn::finish_authentication),
                        );
                    (
                        Some(Extension(server_data)),
                        Some(session_layer),
                        Some(web_authn_router),
                    )
                }
                false => (None, None, None),
            };
            let tls_config = RustlsConfig::from_pem(
                include_bytes!("self_signed_certs/cert.pem").to_vec(),
                include_bytes!("self_signed_certs/key.pem").to_vec(),
            )
                .await
                .unwrap();
            let mut site_router = Router::new()
                .route(
                    "/",
                    get(move || async {
                        tracing::debug!("serving html");
                        Html(html_content)
                    }),
                )
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
            if authentication {
                site_router = site_router.merge(web_authn_router.unwrap());
            }
            site_router = site_router.layer(
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
                    .layer(option_layer(server_data))
                    .layer(option_layer(session_layer)),
            );
            axum_server::bind_rustls(address, tls_config)
                .serve(site_router.into_make_service())
                .await
                .unwrap();
        });
    }
}
