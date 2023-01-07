use r_engen::{
    Canvas, Engen, Extract, Id, Render, RenderAttachment, RenderPassHandle, Task, Viewport,
};

#[derive(Default)]
pub(crate) struct CustomRenderAttachment {}
impl RenderAttachment for CustomRenderAttachment {
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
pub(crate) struct Extractor {}
impl Extract for Extractor {
    fn extract(&mut self, compute: &Task, render: &mut Task) {
        todo!()
    }
}
pub(crate) struct Renderer {}
impl Render for Renderer {
    fn id(&self) -> Id {
        todo!()
    }

    fn phase(&self) -> r_engen::RenderPhase {
        todo!()
    }

    fn render<'a>(&'a self, render_pass: &mut RenderPassHandle<'a>, viewport: &'a Viewport) {
        todo!()
    }
}
