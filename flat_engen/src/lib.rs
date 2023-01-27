use std::net::SocketAddr;

use bevy_ecs::prelude::Resource;

pub use canvas::CanvasOptions;
pub use compile_wasm::CompileDescriptor;
use render::RenderFns;
pub use server::Server;
pub use task::Task;
pub use theme::Theme;

pub use crate::coord::Area;
pub use crate::coord::Position;
use crate::extract::{Extract, ExtractFns, invoke_extract};
use crate::launcher::Launcher;
use crate::render::{invoke_render, Render, RenderPhase};

mod canvas;
mod color;
mod compile_wasm;
mod coord;
mod extract;
mod launcher;
mod orientation;
mod render;
mod server;
mod task;
mod theme;
mod uniform;
mod viewport;

pub struct EngenDescriptor {
    pub canvas_options: Option<CanvasOptions>,
    pub theme: Option<Theme>,
    pub native_dimensions: Option<Area>,
    pub min_canvas_dimensions: Option<Area>,
}

impl EngenDescriptor {
    pub fn new() -> Self {
        Self {
            canvas_options: None,
            theme: None,
            native_dimensions: None,
            min_canvas_dimensions: None,
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
    pub fn with_native_dimensions(mut self, dimensions: Area) -> Self {
        self.native_dimensions.replace(dimensions);
        self
    }
    pub fn with_min_canvas_dimensions(mut self, dimensions: Area) -> Self {
        self.min_canvas_dimensions.replace(dimensions);
        self
    }
}

pub struct EngenOptions {
    pub(crate) canvas_options: CanvasOptions,
    pub(crate) theme: Theme,
    pub(crate) native_dimensions: Option<Area>,
    pub(crate) min_canvas_dimensions: Option<Area>,
}

impl EngenOptions {
    pub(crate) fn new(engen_descriptor: EngenDescriptor) -> Self {
        Self {
            canvas_options: engen_descriptor.canvas_options.unwrap_or_default(),
            theme: engen_descriptor.theme.unwrap_or_default(),
            native_dimensions: engen_descriptor.native_dimensions,
            min_canvas_dimensions: engen_descriptor.min_canvas_dimensions,
        }
    }
}

pub struct Engen {
    pub engen_options: EngenOptions,
    pub(crate) front_end: Task,
    pub(crate) backend: Task,
    pub(crate) render_fns: (RenderFns, RenderFns),
    pub(crate) extract_fns: ExtractFns,
}

impl Engen {
    pub fn new(engen_descriptor: EngenDescriptor) -> Self {
        Self {
            engen_options: EngenOptions::new(engen_descriptor),
            front_end: Task::new(),
            backend: Task::new(),
            render_fns: (RenderFns::new(), RenderFns::new()),
            extract_fns: ExtractFns::new(),
        }
    }
    pub(crate) fn attach<Attachment: Attach>(&mut self) {
        Attachment::attach(self);
    }
    pub fn add_render_attachment<RenderAttachment: Attach + Render + Extract + Resource>(
        &mut self,
    ) {
        RenderAttachment::attach(self);
        match RenderAttachment::phase() {
            RenderPhase::Opaque => self
                .render_fns
                .0
                .push(Box::new(invoke_render::<RenderAttachment>)),
            RenderPhase::Alpha => self
                .render_fns
                .1
                .push(Box::new(invoke_render::<RenderAttachment>)),
        }
        self.extract_fns
            .push(Box::new(invoke_extract::<RenderAttachment>));
    }
    pub fn launch<FrontEndImpl: FrontEnd>(mut self) {
        FrontEndImpl::setup(&mut self.front_end);
        #[cfg(not(target_arch = "wasm32"))]
        {
            Launcher::native(self);
        }
        #[cfg(target_arch = "wasm32")]
        {
            Launcher::web(self);
        }
    }
    pub fn compile_wasm_to(&self, compile_descriptor: CompileDescriptor) -> Option<Server> {
        match compile_descriptor.compile(&self.engen_options.theme) {
            Ok(_) => {}
            Err(_) => {
                return None;
            }
        }
        Some(Server::new(compile_descriptor.destination))
    }
}

pub trait Attach {
    fn attach(engen: &mut Engen);
}

pub trait FrontEnd {
    fn setup(task: &mut Task);
}
