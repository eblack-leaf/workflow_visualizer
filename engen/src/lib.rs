#![allow(unused, dead_code)]
mod canvas;
mod color;
mod coord;
mod instance;
mod job;
mod launcher;
mod options;
mod r_instance;
mod render;
mod run;
mod task;
mod text;
mod theme;
mod uniform;

pub struct Engen {}
impl Engen {
    pub fn new() -> Self {
        Self {}
    }
}
