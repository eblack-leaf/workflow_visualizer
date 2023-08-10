use crate::{Attach, Visualizer};

pub(crate) struct TouchAttachment;
impl Attach for TouchAttachment {
    fn attach(visualizer: &mut Visualizer) {}
}
