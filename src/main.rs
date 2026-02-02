mod camera;
mod colour;
mod geometry;
mod grid;
mod hit_record;
mod material;
mod matrix;
mod random;
mod raster;
mod renderer;
mod sampler;
mod scene;
mod texture;

use camera::Camera;
use raster::Raster;
use scene::Scene;
use scene::element::{Element, Sphere};

use crate::camera::{CameraInstance, PerspectiveCamera};
use crate::colour::Colour;
use crate::geometry::{Point2, Rect, Transform};
use crate::material::{Material, Matte};
use crate::renderer::BasicRenderer;
use crate::renderer::Renderer;
use crate::scene::ElementInstance;
use crate::scene::element::ElementList;
use crate::texture::{ConstantTexture, Texture};

fn build_demo_scene_and_camera(target: &Raster) -> (Scene, CameraInstance) {
    let blue_sphere = Element::Sphere(Sphere::new(
        4.0,
        Material::Matte(Matte::new(
            1.0,
            Texture::Constant(ConstantTexture::new(Colour::new(0.0, 0.0, 1.0))),
        )),
    ));
    let blue_sphere_instance =
        ElementInstance::new(blue_sphere, Transform::translate(0.0, 4.0, 10.0));

    let red_sphere = Element::Sphere(Sphere::new(
        3.0,
        Material::Matte(Matte::new(
            1.0,
            Texture::Constant(ConstantTexture::new(Colour::new(1.0, 0.0, 0.0))),
        )),
    ));
    let red_sphere_instance =
        ElementInstance::new(red_sphere, Transform::translate(7.0, 3.0, 10.0));

    let ground_sphere = Element::Sphere(Sphere::new(
        10.0,
        Material::Matte(Matte::new(
            1.0,
            Texture::Constant(ConstantTexture::new(Colour::new(0.0, 1.0, 0.0))),
        )),
    ));
    let ground_sphere_instance =
        ElementInstance::new(ground_sphere, Transform::translate(0.0, -10.0, 10.0));

    let element_list = Element::ElementList(ElementList::new(vec![
        blue_sphere_instance,
        red_sphere_instance,
        ground_sphere_instance,
    ]));
    let element_list_instance = ElementInstance::new(element_list, Transform::identity());

    let scene = Scene::new(element_list_instance);

    let aspect_ratio = (target.width as f64) / (target.height as f64);
    let screen = Rect::new(
        Point2::new(-aspect_ratio, -1.0),
        Point2::new(aspect_ratio, 1.0),
    );
    println!("screen: {:?}", screen);

    let camera = Camera::PerspectiveCamera(PerspectiveCamera::new(target, screen));
    let camera_instance = CameraInstance::new(camera, Transform::translate(1.0, 0.0, 0.0));

    (scene, camera_instance)
}

fn main() {
    let mut target = Raster::new(80, 60);

    let (scene, camera_instance) = build_demo_scene_and_camera(&target);

    let renderer = BasicRenderer::new();
    renderer.render(&scene, &camera_instance, &mut target);

    target.write_png("out/demo.png");
}
