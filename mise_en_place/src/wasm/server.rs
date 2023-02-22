#[cfg(not(target_arch = "wasm32"))]
use std::net::SocketAddr;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use warp::hyper::body::Bytes;
#[cfg(not(target_arch = "wasm32"))]
use warp::hyper::header::HeaderName;
#[cfg(not(target_arch = "wasm32"))]
use warp::hyper::StatusCode;
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
    #[allow(dead_code)]
    src: String,
}

pub trait MessageHandler {
    #[cfg(not(target_arch = "wasm32"))]
    fn handle(&mut self, user: String, pass: String, message: Bytes) -> (StatusCode, String) {
        println!(
            "post received user: {:?}, pass: {:?}, message: {:?}",
            user, pass, message
        );
        (StatusCode::OK, "body text".to_string())
    }
    fn content_length_max(&self) -> u64 {
        1024 * 16
    }
}

impl WasmServer {
    pub fn new(wasm_compiler: &WasmCompiler) -> Self {
        Self {
            src: wasm_compiler.destination.clone(),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn serve_at<Addr: Into<SocketAddr>, MH: MessageHandler + Send + Sync + 'static>(
        self,
        addr: Addr,
        message_handler: MH,
    ) {
        let cors = warp::cors().build();
        let content_max = message_handler.content_length_max();
        let m_handler = Arc::new(Mutex::new(message_handler));
        let m_handle = m_handler.clone();
        let post = warp::post()
            .and(warp::path("message"))
            .and(warp::header::header::<String>("username"))
            .and(warp::header::header::<String>("password"))
            .and(warp::body::bytes())
            .and(warp::body::content_length_limit(content_max))
            .map(move |user, pass, message| {
                let (status, body) = m_handle
                    .lock()
                    .expect("could not lock")
                    .handle(user, pass, message);
                warp::reply::with_status(
                    warp::reply::with_header(
                        warp::reply::Response::new(warp::hyper::Body::from(body)),
                        "content-type",
                        "text/plain",
                    ),
                    status,
                )
            });
        let site = warp::fs::dir(self.src);
        let rt = tokio::runtime::Runtime::new().expect("no tokio runtime");
        rt.block_on(
            warp::serve(
                site.or(post)
                    .with(warp::compression::gzip())
                    .map(cross_origin_embedder_policy)
                    .map(cross_origin_opener_policy)
                    .with(cors),
            )
            .tls()
            .key(include_bytes!("key.pem"))
            .cert(include_bytes!("cert.pem"))
            .bind(addr),
        );
    }
}

pub fn post_server(
    _message: String,
    _user: String,
    _password: String, /* clone handle of message receiver */
) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::{prelude::*, JsCast};
        let window = web_sys::window().expect("no web window");
        let location = window.location();
        let request = web_sys::XmlHttpRequest::new().unwrap();
        let request_location = location.origin().unwrap() + "/message";
        let _ = request.open("POST", request_location.as_str());
        let closure = Closure::wrap(Box::new(move |xhr: web_sys::XmlHttpRequest| {
            if xhr.ready_state() == web_sys::XmlHttpRequest::DONE && xhr.status().unwrap() == 200 {}
        }) as Box<dyn FnMut(_)>);
        let _ = request.set_onreadystatechange(Some(closure.as_ref().unchecked_ref()));
        let _ = request.set_request_header("username", _user.as_str());
        let _ = request.set_request_header("password", _password.as_str());
        let _ = request.send_with_opt_str(Some(_message.as_str()));
        closure.forget();
    }
}
