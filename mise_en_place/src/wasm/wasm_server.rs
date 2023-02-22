#[cfg(not(target_arch = "wasm32"))]
use std::net::SocketAddr;

#[cfg(not(target_arch = "wasm32"))]
use warp::Filter;
#[cfg(not(target_arch = "wasm32"))]
use warp::hyper::header::HeaderName;

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
    #[allow(dead_code)]
    src: String,
}

impl WasmServer {
    pub fn new(wasm_compiler: &WasmCompiler) -> Self {
        Self {
            src: wasm_compiler.destination.clone(),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn serve_at<Addr: Into<SocketAddr>>(self, addr: Addr) {
        let cors = warp::cors().build();
        let post = warp::post()
            .and(warp::path("save"))
            .and(warp::body::bytes())
            .and(warp::body::content_length_limit(1024 * 5)).map(|e| {
            println!("post received {:?}", e);
            warp::reply()
        });
        let site = warp::fs::dir(self.src)
            .with(warp::compression::gzip())
            .map(cross_origin_embedder_policy)
            .map(cross_origin_opener_policy)
            .with(cors);
        let rt = tokio::runtime::Runtime::new().expect("no tokio runtime");
        rt.block_on(
            warp::serve(site.or(post))
                .tls()
                .key(include_bytes!("key.pem"))
                .cert(include_bytes!("cert.pem"))
                .bind(addr),
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn post_test() {
    let route = warp::post().map(warp::reply);
    let res = warp::test::request().method(warp::hyper::Method::POST.as_str()).reply(&route).await;
    assert_eq!(res.status(), 200);
}

pub fn post_server(string: &mut String) {
    *string = "Ok then".to_string();
    #[cfg(target_arch = "wasm32")] {
        let window = web_sys::window().expect("no web window");
        let location = window.location();
        let request = web_sys::XmlHttpRequest::new().unwrap();
        let request_location = location.origin().unwrap() + "/save";
        let rloc = format!("posting to {}", request_location);
        *string = rloc.clone();
        web_sys::console::info_1(&wasm_bindgen::JsValue::from_str(rloc.as_str()));
        let _ = request.open("POST", request_location.as_str());
        let _ = request.send_with_opt_str(Some("hello mhats the post"));
    }
}