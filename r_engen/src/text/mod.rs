mod font;
mod rasterization;
mod scale;
mod vertex;

use crate::canvas::{Canvas, Viewport};
use crate::color::Color;
use crate::coord::{Area, Depth, Position};
use crate::instance::EntityKey;
use crate::text::scale::Scale;
use crate::{
    instance, Engen, Extract, Id, Render, RenderAttachment, RenderPassHandle, RenderPhase, Task,
};

#[derive(Default)]
pub struct TextAttachment {}
impl RenderAttachment for TextAttachment {
    fn attach(&self, engen: &mut Engen) {
        todo!()
    }
    fn extractor(&self) -> Box<dyn Extract> {
        todo!()
    }
    fn renderer(&self, canvas: &Canvas) -> Box<dyn Render> {
        todo!()
    }
}
pub struct Extractor {}
impl Extract for Extractor {
    fn extract(&mut self, compute: &Task, render: &mut Task) {
        todo!()
    }
}
pub(crate) type GlyphHash = fontdue::layout::GlyphRasterConfig;
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct GlyphOffset(pub usize);
pub struct Request {
    pub character: char,
    pub scale: Scale,
    pub hash: GlyphHash,
    pub position: Position,
    pub area: Area,
    pub depth: Depth,
    pub color: Color,
    pub descriptor: Option<rasterization::Descriptor>,
}
pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    instance_coordinator: instance::Coordinator<EntityKey<GlyphOffset>, Request>,
}
impl Render for Renderer {
    fn id(&self) -> Id {
        Id("text")
    }
    fn phase(&self) -> RenderPhase {
        RenderPhase::Alpha
    }
    fn render<'a>(&'a self, render_pass_handle: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        todo!()
    }
}
