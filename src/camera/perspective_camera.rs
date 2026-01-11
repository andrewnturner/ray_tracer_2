use std::f64::consts::PI;

use crate::{
    geometry::{
        Point2, Point3, Ray, Rect, Transform, Vector3,
        space::{
            CameraSpace, IntermediateSpace, NDCSpace, RasterSpace, ScreenSpace, TargetSpace,
            WorldSpace,
        },
    },
    target::Target,
};

pub struct PerspectiveCamera {
    world_to_camera: Transform<WorldSpace, CameraSpace>,
    camera_to_screen: Transform<CameraSpace, ScreenSpace>,
    screen_to_raster: Transform<ScreenSpace, RasterSpace>,
}

impl PerspectiveCamera {
    pub fn new(target: &Target, window: Rect<ScreenSpace, u32>) -> Self {
        let world_to_camera = Transform::identity();

        let camera_to_screen = Transform::perspective(PI / 3.0, 0.1, 1_000.0);

        let screen_to_raster = Transform::<IntermediateSpace, RasterSpace>::scale(
            target.width as f64,
            target.height as f64,
            1.0,
        ) * Transform::scale(
            1.0 / window.width() as f64,
            1.0 / window.height() as f64,
            1.0,
        ) * Transform::<ScreenSpace, IntermediateSpace>::translate(
            -(window.top_left.x as f64),
            -(window.top_left.y as f64),
            0.0,
        );

        Self {
            world_to_camera,
            camera_to_screen,
            screen_to_raster,
        }
    }

    pub fn generate_ray(&self, target_point: Point2<TargetSpace, f64>) -> Ray<WorldSpace, f64> {
        let p_raster = Point3::new(target_point.x, target_point.y, 0.0);
        let p_screen = self.screen_to_raster.clone().inverse() * p_raster;
        let p_camera = self.camera_to_screen.clone().inverse() * p_screen;

        let ray_camera = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            p_camera.into_vector().normalise(),
        );

        self.world_to_camera.clone().inverse() * ray_camera
    }
}
