use lyon::math::Point;
use lyon::path::PathEvent;
use lyon::tessellation;
use lyon::tessellation::StrokeOptions;
use usvg::tiny_skia_path::{PathSegment, PathSegmentsIter};

fn create_point(x: &f64, y: &f64) -> Point {
    Point::new((*x) as f32, (*y) as f32)
}

pub(crate) struct PathConvIter<'a> {
    iter: PathSegmentsIter<'a>,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
}

impl<'l> Iterator for PathConvIter<'l> {
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }

        let next = self.iter.next();
        match next {
            None => {
                if self.needs_end {
                    self.needs_end = false;
                    let last = self.prev;
                    let first = self.first;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    None
                }
            }
            Some(segment) => {
                match segment {
                    PathSegment::MoveTo(point) => {
                        if self.needs_end {
                            let last = self.prev;
                            let first = self.first;
                            self.needs_end = false;
                            self.prev = create_point(&(point.x as f64), &(point.y as f64));
                            self.deferred = Some(PathEvent::Begin { at: self.prev });
                            self.first = self.prev;
                            Some(PathEvent::End {
                                last,
                                first,
                                close: false,
                            })
                        } else {
                            self.first = create_point(&(point.x as f64), &(point.y as f64));
                            self.needs_end = true;
                            Some(PathEvent::Begin { at: self.first })
                        }
                    }
                    PathSegment::LineTo(point) => {
                        self.needs_end = true;
                        let from = self.prev;
                        self.prev = create_point(&(point.x as f64), &(point.y as f64));
                        Some(PathEvent::Line {
                            from,
                            to: self.prev,
                        })
                    }
                    PathSegment::QuadTo(a, b) => {
                        // PathEvent::Quad?
                        None
                    }
                    PathSegment::CubicTo(a, b, c) => {
                        self.needs_end = true;
                        let from = self.prev;
                        self.prev = create_point(&(c.x as f64), &(c.y as f64));
                        Some(PathEvent::Cubic {
                            from,
                            ctrl1: create_point(&(a.x as f64), &(a.y as f64)),
                            ctrl2: create_point(&(b.x as f64), &(b.y as f64)),
                            to: self.prev,
                        })
                    }
                    PathSegment::Close => {
                        self.needs_end = false;
                        self.prev = self.first;
                        Some(PathEvent::End {
                            last: self.prev,
                            first: self.first,
                            close: true,
                        })
                    }
                }
            }
        }
    }
}
#[allow(unused)]
pub(crate) fn convert_path(p: &usvg::Path) -> PathConvIter {
    PathConvIter {
        iter: p.data.segments(),
        first: Point::new(0.0, 0.0),
        prev: Point::new(0.0, 0.0),
        deferred: None,
        needs_end: false,
    }
}
#[allow(unused)]
pub(crate) const FALLBACK_COLOR: usvg::Color = usvg::Color {
    red: 0,
    green: 0,
    blue: 0,
};
#[allow(unused)]
pub(crate) fn convert_stroke(s: &usvg::Stroke) -> (usvg::Color, StrokeOptions) {
    let color = match s.paint {
        usvg::Paint::Color(c) => c,
        _ => FALLBACK_COLOR,
    };
    let linecap = match s.linecap {
        usvg::LineCap::Butt => tessellation::LineCap::Butt,
        usvg::LineCap::Square => tessellation::LineCap::Square,
        usvg::LineCap::Round => tessellation::LineCap::Round,
    };
    let linejoin = match s.linejoin {
        usvg::LineJoin::Miter => tessellation::LineJoin::Miter,
        usvg::LineJoin::Bevel => tessellation::LineJoin::Bevel,
        usvg::LineJoin::Round => tessellation::LineJoin::Round,
    };

    let opt = StrokeOptions::tolerance(0.01)
        .with_line_width(s.width.get())
        .with_line_cap(linecap)
        .with_line_join(linejoin);

    (color, opt)
}
