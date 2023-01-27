use std::net::SocketAddr;

pub struct Server {
    pub src: String,
}

impl Server {
    pub fn new<T: Into<String>>(src: T) -> Self {
        Self { src: src.into() }
    }
    pub fn serve_at<Addr: Into<SocketAddr>>(mut self, addr: Addr) {}
}
