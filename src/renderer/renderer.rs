use crate::camera::CameraInstance;
use crate::scene::Scene;
use crate::target::Target;

pub trait Renderer {
    fn render(&self, scene: &Scene, camera_instance: &CameraInstance, target: &mut Target);
}
