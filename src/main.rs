mod camera;
mod colour;
mod geometry;
mod grid;
mod hit_record;
mod material;
mod matrix;
mod random;
mod renderer;
mod sampler;
mod scene;
mod target;

use camera::Camera;
use scene::Scene;
use scene::element::{Element, Sphere};
use target::Target;

use crate::camera::{CameraInstance, PerspectiveCamera};
use crate::geometry::{Point2, Rect, Transform};
use crate::material::{Material, Matte};
use crate::renderer::BasicRenderer;
use crate::renderer::Renderer;
use crate::scene::ElementInstance;

fn build_demo_scene_and_camera(target: &Target) -> (Scene, CameraInstance) {
    let sphere = Element::Sphere(Sphere::new(4.0, Material::Matte(Matte::new(1.0))));
    let sphere_instance = ElementInstance::new(sphere, Transform::translate(0.0, 0.0, 10.0));
    let scene = Scene::new(sphere_instance);

    let aspect_ratio = (target.width as f64) / (target.height as f64);
    let screen = Rect::new(
        Point2::new(-aspect_ratio, -1.0),
        Point2::new(aspect_ratio, 1.0),
    );

    let camera = Camera::PerspectiveCamera(PerspectiveCamera::new(target, screen));
    let camera_instance = CameraInstance::new(camera, Transform::translate(1.0, 0.0, 0.0));

    (scene, camera_instance)
}

fn main() {
    let mut target = Target::new(160, 120);

    let (scene, camera_instance) = build_demo_scene_and_camera(&target);

    let renderer = BasicRenderer::new();
    renderer.render(&scene, &camera_instance, &mut target);

    target.write_png("out/demo.png");
}
