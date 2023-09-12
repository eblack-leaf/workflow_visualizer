use crate::svg_icon::svg_glue::{convert_path, convert_stroke, FALLBACK_COLOR};
use crate::svg_icon::SvgData;
use crate::{RawPosition, ResourceHandle};
use bevy_ecs::component::Component;
use lyon::lyon_tessellation::{BuffersBuilder, StrokeVertex, StrokeVertexConstructor};
use lyon::tessellation::{
    FillOptions, FillTessellator, FillVertex, FillVertexConstructor, StrokeTessellator,
    VertexBuffers,
};
use serde::{Deserialize, Serialize};
use usvg::TreeParsing;
#[allow(unused)]
pub struct ParsedSvg {
    pub tree: usvg::Tree,
}

impl ParsedSvg {
    #[allow(unused)]
    pub fn new(tree: usvg::Tree) -> Self {
        Self { tree }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TessellatedSvg {
    pub vertices: Vec<RawPosition>,
    pub indices: Vec<u32>,
}
#[allow(unused)]
impl TessellatedSvg {
    fn new(geometry: VertexBuffers<RawPosition, u32>) -> Self {
        Self {
            vertices: geometry.vertices,
            indices: geometry.indices,
        }
    }
}
#[allow(unused)]
pub fn tessellate_svg(svg: SvgData) -> Option<TessellatedSvg> {
    let parsed_svg = usvg::Tree::from_data(svg.as_slice(), &usvg::Options::default());
    if let Ok(svg) = parsed_svg {
        let size = svg.size;
        let _view_box = svg.view_box;
        let mut geometry = lyon::tessellation::VertexBuffers::<RawPosition, u32>::new();
        let mut fill_tess = FillTessellator::new();
        let mut stroke_tess = StrokeTessellator::new();
        for node in svg.root.descendants() {
            if let usvg::NodeKind::Path(ref p) = *node.borrow() {
                if let Some(ref fill) = p.fill {
                    let _color = match fill.paint {
                        usvg::Paint::Color(c) => c,
                        _ => FALLBACK_COLOR,
                    };
                    fill_tess
                        .tessellate(
                            convert_path(p),
                            &FillOptions::tolerance(0.01),
                            &mut BuffersBuilder::<RawPosition, u32, Ctor>::new(
                                &mut geometry,
                                Ctor(1),
                            ),
                        )
                        .expect("Error during tessellation!");
                }
                if let Some(ref stroke) = p.stroke {
                    let (_stroke_color, stroke_opts) = convert_stroke(stroke);
                    let _ = stroke_tess.tessellate(
                        convert_path(p),
                        &stroke_opts.with_tolerance(0.01),
                        &mut BuffersBuilder::<RawPosition, u32, Ctor>::new(&mut geometry, Ctor(1)),
                    );
                }
            }
        }
        println!("{:?}", geometry.vertices.len());
        println!("{:?}", geometry.indices.len());
        for vertex in geometry.vertices.iter_mut() {
            vertex.x /= size.width();
            vertex.y /= size.height();
        }
        return Some(TessellatedSvg::new(geometry));
    }
    None
}

pub struct Ctor(pub i32);

impl FillVertexConstructor<RawPosition> for Ctor {
    fn new_vertex(&mut self, vertex: FillVertex) -> RawPosition {
        RawPosition::new(vertex.position().x, vertex.position().y)
    }
}

impl StrokeVertexConstructor<RawPosition> for Ctor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> RawPosition {
        RawPosition::new(vertex.position().x, vertex.position().y)
    }
}

#[cfg(test)]
#[test]
fn tester() {
    let tessellated = tessellate_svg(include_bytes!("svg/activity.svg").to_vec());
    if let Some(tess) = tessellated {
        std::fs::write(
            "/home/omi-voshuli/Desktop/dev/workflow_visualizer/workflow_visualizer/src/svg_icon/tessellated_svg/activity.tess",
                       serde_json::to_string(&tess).expect("serialize")).expect("write");
    }
}
#[allow(unused)]
pub enum BundledSvg {
    Activity,
}
impl BundledSvg {
    #[allow(unused)]
    pub fn tessellation(&self) -> TessellatedSvg {
        match &self {
            BundledSvg::Activity => {
                serde_json::from_slice(include_bytes!("tessellated_svg/activity.tess"))
                    .expect("svg load")
            }
        }
    }
}
#[derive(Component)]
pub struct SvgRequest {
    pub handle: ResourceHandle,
    pub tessellated_svg: Option<TessellatedSvg>,
}

impl SvgRequest {
    #[allow(unused)]
    pub fn new<RH: Into<ResourceHandle>>(handle: RH, svg: TessellatedSvg) -> Self {
        Self {
            handle: handle.into(),
            tessellated_svg: Some(svg),
        }
    }
}
