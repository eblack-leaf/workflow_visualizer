use std::net::SocketAddr;

use warp::hyper::header::HeaderName;
use warp::Filter;

use crate::wasm_compiler::WasmCompiler;

fn cross_origin_embedder_policy(reply: impl warp::Reply) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        HeaderName::from_static("cross-origin-embedder-policy"),
        "require-corp",
    )
}

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
    pub(crate) fn new(wasm_compiler: WasmCompiler) -> Self {
        Self {
            src: wasm_compiler.destination,
        }
    }
    pub fn serve_at<Addr: Into<SocketAddr>>(mut self, addr: Addr) {
        let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET"]);
        let routes = warp::fs::dir(self.src)
            .map(cross_origin_embedder_policy)
            .map(cross_origin_opener_policy)
            .with(cors);
        let mut rt = tokio::runtime::Runtime::new().expect("no tokio runtime");
        rt.block_on(
            warp::serve(routes)
                .tls()
                .key(include_bytes!("key.pem"))
                .cert(include_bytes!("cert.pem"))
                .run(addr),
        );
    }
}
