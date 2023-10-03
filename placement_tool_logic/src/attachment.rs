use workflow_visualizer::{Attach, Visualizer};

pub struct LogicAttachment;
impl Attach for LogicAttachment {
    fn attach(visualizer: &mut Visualizer) {}
}
