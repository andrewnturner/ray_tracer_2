use crate::camera::Camera;
use crate::scene::Scene;
use crate::target::Target;

pub trait Renderer {
    fn render(&self, scene: &Scene, camera: &Camera, target: &mut Target);
}
