mod camera;
mod colour;
mod geometry;
mod grid;
mod hit_record;
mod matrix;
mod renderer;
mod sampler;
mod scene;
mod target;

use camera::Camera;
use scene::Scene;
use scene::element::{Element, Sphere};
use target::Target;

use crate::camera::PerspectiveCamera;
use crate::geometry::{Point2, Rect, Transform};
use crate::renderer::BasicRenderer;
use crate::renderer::Renderer;
use crate::scene::ElementInstance;

fn build_demo_scene_and_camera(target: &Target) -> (Scene, Camera) {
    let sphere = Element::Sphere(Sphere::new(4.95));
    let sphere_instance = ElementInstance::new(sphere, Transform::translate(0.0, 0.0, 5.0));
    let scene = Scene::new(sphere_instance);

    let screen = Rect::new(Point2::new(0, 0), Point2::new(120, 100));
    let camera = Camera::PerspectiveCamera(PerspectiveCamera::new(target, screen));

    (scene, camera)
}

fn main() {
    let mut target = Target::new(80, 60);

    let (scene, camera) = build_demo_scene_and_camera(&target);

    let renderer = BasicRenderer::new();
    renderer.render(&scene, &camera, &mut target);

    target.write_png("out/demo.png");
}
