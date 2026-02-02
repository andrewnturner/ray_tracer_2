use crate::geometry::{
    Point2, Ray, Transform,
    space::{CameraSpace, RasterSpace, WorldSpace},
};

use super::perspective_camera::PerspectiveCamera;

pub enum Camera {
    PerspectiveCamera(PerspectiveCamera),
}

impl Camera {
    pub fn generate_ray(&self, target_point: Point2<RasterSpace, f64>) -> Ray<CameraSpace, f64> {
        match self {
            Self::PerspectiveCamera(c) => c.generate_ray(target_point),
        }
    }
}

pub struct CameraInstance {
    pub camera: Camera,
    pub world_to_camera: Transform<WorldSpace, CameraSpace>,
}

impl CameraInstance {
    pub fn new(camera: Camera, world_to_camera: Transform<WorldSpace, CameraSpace>) -> Self {
        Self {
            camera,
            world_to_camera,
        }
    }
}
