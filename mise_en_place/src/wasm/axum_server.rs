use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::{Extension, Router, ServiceExt};
use axum::routing::get;
use axum_server::tls_rustls::RustlsConfig;
use axum_sessions::{SameSite, SessionLayer};
use axum_sessions::async_session::MemoryStore;
use rand::{Rng, thread_rng};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use url::Url;
use uuid::Uuid;
use warp::post;
use webauthn_rs::{Webauthn, WebauthnBuilder};
use webauthn_rs::prelude::Passkey;

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

pub struct AxumWasmServer {}

pub(crate) struct ServerData {
    pub(crate) webauthn: Arc<Webauthn>,
    pub(crate) user_data: Arc<Mutex<UserData>>,
}

impl ServerData {
    pub(crate) fn new(rp_id: IpAddr, rp_origin: &Url) -> Self {
        let builder = WebauthnBuilder::new(rp_id.to_string().as_str(), rp_origin).expect("Invalid configuration");
        let webauthn = Arc::new(builder.build().expect("could no build webauthn"));
        let user_data = Arc::new(Mutex::new(UserData::new()));
        Self {
            webauthn,
            user_data,
        }
    }
}

impl AxumWasmServer {
    pub async fn serve_at<Addr: Into<SocketAddr>>(src: PathBuf, addr: Addr) {
        let address = addr.into();
        let rp_id = address.ip();
        let url = "https://" + address.to_string();
        let rp_origin = Url::parse(url).expect("Invalid URL");
        let server_data = ServerData::new(rp_id, &rp_origin);
        let tls_config = RustlsConfig::from_pem_file(
            "mise_en_place/src/wasm/cert.pem",
            "mise_en_place/src/wasm/cert.pem",
        ).await.unwrap();
        let store = MemoryStore::new();
        let secret = thread_rng().gen::<[u8; 128]>(); // MUST be at least 64 bytes!
        let session_layer = SessionLayer::new(store, &secret)
            .with_cookie_name("webauthnrs")
            .with_same_site_policy(SameSite::Strict)
            .with_secure(true);
        let router = Router::new()
            .route("/", get(site_fetch))
            .route("/register_start/:username", post(start_register))
            .route("/register_finish", post(finish_register))
            .route("/login_start/:username", post(start_authentication))
            .route("/login_finish", post(finish_authentication))
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::new()
                        .allow_origin())
                    .layer(CompressionLayer::new())
                    .layer(Extension(server_data))
                    .layer(session_layer)
            );
        axum_server::bind_rustls(addr.into(), tls_config)
            .serve(router.into_make_service())
            .await
            .unwrap();
    }
}

fn site_fetch() {}

fn start_register() {}

fn finish_register() {}

fn start_authentication() {}

fn finish_authentication() {}