use crate::camera::CameraInstance;
use crate::raster::Raster;
use crate::scene::Scene;

pub trait Renderer {
    fn render(&self, scene: &Scene, camera_instance: &CameraInstance, target: &mut Raster);
}
