mod colour;
mod geometry;
mod grid;
mod renderer;
mod sampler;
mod scene;
mod target;

use scene::Camera;
use scene::Scene;
use scene::element::{Element, Sphere};
use target::Target;

use crate::renderer::BasicRenderer;
use crate::renderer::Renderer;

fn build_demo_scene_and_camera() -> (Scene, Camera) {
    let element = Element::Sphere(Sphere::new());
    let scene = Scene::new(element);

    let camera = Camera::new();

    (scene, camera)
}

fn main() {
    let (scene, camera) = build_demo_scene_and_camera();

    let mut target = Target::new(80, 60);

    let renderer = BasicRenderer::new();
    renderer.render(&scene, &camera, &mut target);

    target.write_png("out/demo.png");
}
