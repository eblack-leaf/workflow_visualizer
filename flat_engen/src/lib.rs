use std::net::SocketAddr;
pub struct CanvasOptions {}
impl Default for CanvasOptions {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            CanvasOptions {}
        }
        #[cfg(target_arch = "wasm32")]
        {
            CanvasOptions {}
        }
    }
}
pub struct Theme {}
impl Default for Theme {
    fn default() -> Self {
        Theme {}
    }
}
pub struct EngenDescriptor {
    pub canvas_options: Option<CanvasOptions>,
    pub theme: Option<Theme>,
}
impl EngenDescriptor {
    pub fn new() -> Self {
        Self {
            canvas_options: None,
            theme: None,
        }
    }
    pub fn with_canvas_options(mut self, canvas_options: CanvasOptions) -> Self {
        self.canvas_options.replace(canvas_options);
        self
    }
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme.replace(theme);
        self
    }
}
pub struct Engen {
    pub engen_descriptor: EngenDescriptor,
}
impl Engen {
    pub fn new(engen_descriptor: EngenDescriptor) -> Self {
        Self { engen_descriptor }
    }
    pub fn attach_renderer<Renderer: Attach + Render + Extract>(&mut self) {}
    pub fn launch<FrontEndImpl: FrontEnd>(mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {}
        #[cfg(target_arch = "wasm32")]
        {}
    }
    pub fn compile_wasm_to(compile_descriptor: CompileDescriptor) -> Server {
        Server::new(compile_descriptor.destination)
    }
}
pub struct Server {
    pub src: String,
}
impl Server {
    pub fn new<T: Into<String>>(src: T) -> Self {
        Self { src: src.into() }
    }
    pub fn serve_at<Addr: Into<SocketAddr>>(mut self, addr: Addr) {}
}
pub struct Task {}
pub trait FrontEnd {
    fn setup(task: &mut Task);
}
pub struct CompileDescriptor {
    pub package: String,
    pub package_options: String,
    pub destination: String,
}
impl CompileDescriptor {
    pub fn new<T: Into<String>>(package: T, package_options: T, destination: T) -> Self {
        Self {
            package: package.into(),
            package_options: package_options.into(),
            destination: destination.into(),
        }
    }
}
