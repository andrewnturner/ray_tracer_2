use crate::geometry::{
    Point2, Ray,
    space::{TargetSpace, WorldSpace},
};

use super::perspective_camera::PerspectiveCamera;

pub enum Camera {
    PerspectiveCamera(PerspectiveCamera),
}

impl Camera {
    pub fn generate_ray(&self, target_point: Point2<TargetSpace, f64>) -> Ray<WorldSpace, f64> {
        match self {
            Self::PerspectiveCamera(c) => c.generate_ray(target_point),
        }
    }
}
