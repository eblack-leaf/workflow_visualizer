mod svg_glue;

use crate::svg_icon::svg_glue::{convert_path, convert_stroke, FALLBACK_COLOR};
use crate::RawPosition;
use lyon::lyon_tessellation::{FillVertex, StrokeVertex, VertexBuffers};
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertexConstructor, StrokeTessellator,
    StrokeVertexConstructor,
};
use serde::{Deserialize, Serialize};
use usvg::{NodeExt, TreeParsing};

pub type SvgData = Vec<u8>;
pub(crate) struct ParsedSvg {
    pub(crate) tree: usvg::Tree,
}
impl ParsedSvg {
    pub(crate) fn new(tree: usvg::Tree) -> Self {
        Self { tree }
    }
}
#[derive(Serialize, Deserialize)]
pub(crate) struct TessellatedSvg {
    pub(crate) vertices: Vec<RawPosition>,
    pub(crate) indices: Vec<u32>,
}

impl TessellatedSvg {
    fn new(geometry: VertexBuffers<RawPosition, u32>) -> Self {
        Self {
            vertices: geometry.vertices,
            indices: geometry.indices,
        }
    }
}

pub(crate) fn tessellate_svg(svg: SvgData) -> Option<TessellatedSvg> {
    let parsed_svg = usvg::Tree::from_data(svg.as_slice(), &usvg::Options::default());
    if let Ok(svg) = parsed_svg {
        let size = svg.size;
        let view_box = svg.view_box;
        let mut geometry = lyon::tessellation::VertexBuffers::<RawPosition, u32>::new();
        let mut fill_tess = FillTessellator::new();
        let mut stroke_tess = StrokeTessellator::new();
        for node in svg.root.descendants() {
            if let usvg::NodeKind::Path(ref p) = *node.borrow() {
                let t = node.transform();
                if let Some(ref fill) = p.fill {
                    let color = match fill.paint {
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
                    let (stroke_color, stroke_opts) = convert_stroke(stroke);
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
        for mut vertex in geometry.vertices.iter_mut() {
            vertex.x /= size.width();
            vertex.y /= size.height();
        }
        return Some(TessellatedSvg::new(geometry));
    }
    None
}
pub(crate) struct Ctor(pub(crate) i32);
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
pub enum BundledSvg {
    Activity,
}
pub struct SvgRequest {}
pub(crate) struct TessellatedSvgBuffer {
    pub(crate) vertices: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
}
// render buffer with color + scale to desired area at pos (what context is tessellation in?)
