use crate::{
    camera::{Camera, CameraInstance},
    colour::Colour,
    geometry::{Point2, Ray, space::WorldSpace},
    renderer::Renderer,
    sampler::Sampler,
    scene::Scene,
    target::Target,
};

pub struct BasicRenderer {
    sampler: Sampler,
}

impl BasicRenderer {
    pub fn new() -> Self {
        Self {
            sampler: Sampler::new(10),
        }
    }

    fn light_intensity(&self, ray: &Ray<WorldSpace, f64>, scene: &Scene) -> Colour {
        if let Some(hit_record) = scene.intersect(ray) {
            let f = ray.direction.dot(hit_record.normal.relabel()); // TODO convert to world space
            Colour::new(f, 0.0, 0.0)
        } else {
            Colour::new(0.0, 1.0, 0.0)
        }
    }
}

impl Renderer for BasicRenderer {
    fn render(&self, scene: &Scene, camera_instance: &CameraInstance, target: &mut Target) {
        let target_rect = target.rect();

        for x in target_rect.top_left.x..target_rect.bottom_right.x {
            for y in target_rect.top_left.y..target_rect.bottom_right.y {
                let current_pixel = Point2::new(x, y);

                let mut pixel_colour = Colour::zero();
                let mut sample_count = 0;
                for sample in self
                    .sampler
                    .sample_pixel(current_pixel, &camera_instance.camera)
                {
                    let ray_camera = sample.ray;
                    let ray_world = camera_instance.world_to_camera.clone().inverse() * ray_camera;
                    let intensity = self.light_intensity(&ray_world, scene);

                    pixel_colour += intensity;
                    sample_count += 1;
                }

                pixel_colour /= sample_count as f64;

                target.set_pixel(current_pixel, pixel_colour);
            }
        }
    }
}
