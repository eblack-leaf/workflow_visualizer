use std::net::SocketAddr;

#[cfg(not(target_arch = "wasm32"))]
use warp::hyper::header::HeaderName;
#[cfg(not(target_arch = "wasm32"))]
use warp::Filter;

use crate::wasm::WasmCompiler;

#[cfg(not(target_arch = "wasm32"))]
fn cross_origin_embedder_policy(reply: impl warp::Reply) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        HeaderName::from_static("cross-origin-embedder-policy"),
        "require-corp",
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn cross_origin_opener_policy(reply: impl warp::Reply) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        HeaderName::from_static("cross-origin-opener-policy"),
        "same-origin",
    )
}

pub struct WasmServer {
    src: String,
}

impl WasmServer {
    pub fn new(wasm_compile_descriptor: &WasmCompiler) -> Self {
        Self {
            src: wasm_compile_descriptor.destination.clone(),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn serve_at<Addr: Into<SocketAddr>>(self, addr: Addr) {
        let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET"]);
        let routes = warp::fs::dir(self.src)
            .with(warp::compression::gzip())
            .map(cross_origin_embedder_policy)
            .map(cross_origin_opener_policy)
            .with(cors);
        let rt = tokio::runtime::Runtime::new().expect("no tokio runtime");
        rt.block_on(
            warp::serve(routes)
                .tls()
                .key(include_bytes!("key.pem"))
                .cert(include_bytes!("cert.pem"))
                .run(addr),
        );
    }
}
