mod camera;
mod colour;
mod geometry;
mod grid;
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
use crate::geometry::{Point2, Rect};
use crate::renderer::BasicRenderer;
use crate::renderer::Renderer;

fn build_demo_scene_and_camera(target: &Target) -> (Scene, Camera) {
    let element = Element::Sphere(Sphere::new());
    let scene = Scene::new(element);

    let screen = Rect::new(Point2::new(0, 0), Point2::new(120, 100));
    let camera = Camera::PerspectiveCamera(PerspectiveCamera::new(target, screen));

    (scene, camera)
}

fn main() {
    let mut target = Target::new(4, 3);

    let (scene, camera) = build_demo_scene_and_camera(&target);

    let renderer = BasicRenderer::new();
    renderer.render(&scene, &camera, &mut target);

    target.write_png("out/demo.png");
}
