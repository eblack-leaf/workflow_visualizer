use workflow_visualizer::{Attach, Visualizer};

pub struct EntryAttachment;
impl Attach for EntryAttachment {
    fn attach(visualizer: &mut Visualizer) {}
}
