use crate::{
    renderer::Renderer,
    scene::{Camera, Scene},
    target::Target,
};

pub struct BasicRenderer {}

impl BasicRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Renderer for BasicRenderer {
    fn render(&self, scene: &Scene, camera: &Camera, target: &mut Target) {}
}
