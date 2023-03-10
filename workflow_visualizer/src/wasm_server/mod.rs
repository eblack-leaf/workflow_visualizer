use axum::headers::{HeaderName, HeaderValue};
use axum::http::Method;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct WasmServer;

impl WasmServer {
    pub fn serve_at<P: Into<PathBuf>, Addr: Into<SocketAddr>>(src: P, addr: Addr) {
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
                    ),
            );
            axum_server::bind_rustls(address, tls_config)
                .serve(site_router.into_make_service())
                .await
                .unwrap();
        });
    }
}
