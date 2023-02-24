use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::net::SocketAddr;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use warp::Filter;
#[cfg(not(target_arch = "wasm32"))]
use warp::hyper::header::HeaderName;
#[cfg(not(target_arch = "wasm32"))]
use warp::hyper::StatusCode;

use crate::{Attach, Engen};
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

#[cfg(not(target_arch = "wasm32"))]
pub type StatusCodeExpt = StatusCode;

pub trait MessageHandler {
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unused)]
    fn handle(
        &mut self,
        user: String,
        pass: String,
        ty: MessageType,
        message: Message,
    ) -> (StatusCodeExpt, (MessageType, Message));
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
        let post = warp::post()
            .and(warp::path("message"))
            .and(warp::header::header::<String>("username"))
            .and(warp::header::header::<String>("password"))
            .and(warp::body::bytes())
            .and(warp::body::content_length_limit(content_max))
            .map(move |user, pass, data: warp::hyper::body::Bytes| {
                let ty = data[0];
                let message = data[1..].to_vec();
                let (status, (reply_ty, reply_message)) =
                    m_handler.lock().unwrap().handle(user, pass, ty, message);
                let mut reply_data = Vec::from(reply_message);
                reply_data.insert(0, reply_ty);
                warp::reply::with_status(
                    warp::reply::with_header(
                        warp::reply::Response::new(warp::hyper::Body::from(reply_data)),
                        "content-type",
                        "application/octet-stream",
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
pub type Message = Vec<u8>;

pub fn to_message<T: Serialize>(t: &T) -> Option<Message> {
    match rmp_serde::to_vec(t) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

pub fn resolve_message<T: for<'a> Deserialize<'a>>(message: Message) -> Option<T> {
    match rmp_serde::from_slice::<T>(&*message) {
        Ok(t) => Some(t),
        _ => None,
    }
}

#[derive(Resource)]
pub struct MessageReceiver {
    pub(crate) messages: Arc<Mutex<Messages>>,
}

pub type MessageType = u8;

pub struct Messages {
    pub data: HashMap<Username, Vec<(MessageType, Message)>>,
}

impl Messages {
    pub(crate) fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn receive(&mut self, user: Username, response: (MessageType, Message)) {
        if let Some(message_buffer) = self.data.get_mut(&user) {
            message_buffer.push(response);
        } else {
            self.data.insert(user, vec![response]);
        }
    }
}

pub trait MessageRepr
    where
        Self: Serialize + Sized,
{
    fn message_type() -> MessageType;
    fn to_message(&self) -> Option<Message> {
        to_message(self)
    }
}

impl MessageReceiver {
    pub(crate) fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Messages::new())),
        }
    }
    pub fn post_message<M: MessageRepr>(&self, _message: M, _user: Username, _password: Password) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, prelude::*};
            let window = web_sys::window().expect("no web window");
            let location = window.location();
            let request = Rc::new(web_sys::XmlHttpRequest::new().unwrap());
            request.set_response_type(web_sys::XmlHttpRequestResponseType::Arraybuffer);
            let request_location = location.origin().unwrap() + "/message";
            let _ = request.open("POST", request_location.as_str());
            let request_handle = request.clone();
            let messages_handle = self.messages.clone();
            let user_clone = _user.clone();
            let closure = Closure::wrap(Box::new(move |_e: web_sys::Event| {
                if request_handle.ready_state() == web_sys::XmlHttpRequest::DONE
                    && request_handle.status().unwrap() == 200
                {
                    if let Ok(response) = request_handle.response() {
                        let array = js_sys::Uint8Array::new(&response);
                        let data = array.to_vec();
                        let response_type = data[0];
                        let response_body = data[1..].to_vec();
                        messages_handle
                            .lock()
                            .unwrap()
                            .receive(user_clone.clone(), (response_type, response_body));
                    }
                }
            }) as Box<dyn Fn(_)>);
            let _ = request.set_onreadystatechange(Some(closure.as_ref().unchecked_ref()));
            let _ = request.set_request_header("username", _user.as_str());
            let _ = request.set_request_header("password", _password.as_str());
            let mut data = rmp_serde::to_vec(&_message).unwrap();
            data.insert(0, M::message_type());
            let _ = request.send_with_opt_u8_array(Some(data.as_slice()));
            closure.forget();
        }
    }
    pub fn messages(&self) -> Vec<(Username, Vec<(MessageType, Message)>)> {
        self.messages
            .lock()
            .unwrap()
            .data
            .drain()
            .collect::<Vec<(Username, Vec<(MessageType, Message)>)>>()
    }
}

impl Attach for MessageReceiver {
    fn attach(engen: &mut Engen) {
        engen
            .frontend
            .container
            .insert_resource(MessageReceiver::new());
    }
}
