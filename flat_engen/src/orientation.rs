pub enum Orientation {
    Landscape,
    Portrait,
}
// handle orientation as Portrait default
// landscape adds extra stack column if scaled gpu_area can contain it else just stack scroll/page down to other focus
// no scroll just focus idea, flat layout of views
