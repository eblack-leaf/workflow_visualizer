use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;
#[cfg(not(target_arch = "wasm32"))]
use warp::hyper::header::HeaderName;
#[cfg(not(target_arch = "wasm32"))]
use warp::hyper::StatusCode;
#[cfg(not(target_arch = "wasm32"))]
use warp::Filter;

use crate::wasm::WasmCompiler;
use crate::{Attach, Engen};

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
    fn handle(&mut self, user: String, pass: String, message: String) -> (StatusCode, String) {
        println!(
            "post received user: {}, pass: {}, message: {}",
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
            .map(move |user, pass, message: warp::hyper::body::Bytes| {
                let (status, body) = m_handle.lock().expect("could not lock").handle(
                    user,
                    pass,
                    std::str::from_utf8(message.as_ref()).unwrap().to_string(),
                );
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

pub type Username = String;
pub type Password = String;
pub type Message = String;

pub struct MessageReceiver {
    pub messages: HashMap<Username, Message>,
}

impl MessageReceiver {
    pub(crate) fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn receive(&mut self, user: &Username, message: Message) {
        self.messages.insert(user.clone(), message);
    }
}

#[derive(Resource)]
pub struct MessageReceiverHandler {
    pub(crate) handle: Arc<Mutex<MessageReceiver>>,
}

impl MessageReceiverHandler {
    pub(crate) fn new() -> Self {
        Self {
            handle: Arc::new(Mutex::new(MessageReceiver::new())),
        }
    }
    pub fn generate_handle(&self) -> Arc<Mutex<MessageReceiver>> {
        self.handle.clone()
    }
}

impl Attach for MessageReceiverHandler {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(MessageReceiverHandler::new());
    }
}

pub fn post_server(
    _message: Message,
    _user: Username,
    _password: Password,
    _message_receiver_handle: &MessageReceiverHandler,
) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::{prelude::*, JsCast};
        let handle = _message_receiver_handle.generate_handle();
        let window = web_sys::window().expect("no web window");
        let location = window.location();
        let request = web_sys::XmlHttpRequest::new().unwrap();
        let request_location = location.origin().unwrap() + "/message";
        let _ = request.open("POST", request_location.as_str());
        let user_cloned = _user.clone();
        let closure = Closure::wrap(Box::new(move |xhr: web_sys::XmlHttpRequest| {
            if xhr.ready_state() == web_sys::XmlHttpRequest::DONE && xhr.status().unwrap() == 200 {
                handle
                    .lock()
                    .unwrap()
                    .receive(&user_cloned, xhr.response_text().unwrap().unwrap());
            }
        }) as Box<dyn Fn(_)>);
        let _ = request.set_onreadystatechange(Some(closure.as_ref().unchecked_ref()));
        let _ = request.set_request_header("username", _user.as_str());
        let _ = request.set_request_header("password", _password.as_str());
        let _ = request.send_with_opt_str(Some(_message.as_str()));
        closure.forget();
    }
}
