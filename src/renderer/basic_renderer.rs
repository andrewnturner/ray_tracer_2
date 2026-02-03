use crate::{
    camera::CameraInstance,
    colour::Colour,
    geometry::{Point2, Ray, Vector3, space::WorldSpace},
    raster::Raster,
    renderer::Renderer,
    sampler::Sampler,
    scene::Scene,
};

pub struct BasicRenderer {
    sampler: Sampler,
}

impl BasicRenderer {
    pub fn new() -> Self {
        Self {
            sampler: Sampler::new(1),
        }
    }

    fn trace(&self, ray: &Ray<WorldSpace, f64>, scene: &Scene, depth: u32) -> Colour {
        if depth <= 0 {
            return Colour::zero();
        }

        if let Some(hit_record) = scene.intersect(ray) {
            if let Some(scattered_ray) = hit_record.material.scatter(&hit_record) {
                let attentuation = hit_record
                    .material
                    .texture()
                    .query(hit_record.texture_point);
                let scattered = self.trace(&scattered_ray, &scene, depth - 1);

                let result = Colour::new(
                    scattered.r * attentuation.r,
                    scattered.g * attentuation.g,
                    scattered.b * attentuation.b,
                );

                result
            } else {
                Colour::zero()
            }
        } else {
            let ambient_direction = Vector3::new(1.0, 1.0, 1.0).normalise();
            let power = ray.direction.dot(ambient_direction);
            if power > 0.0 {
                power * Colour::new(0.5, 0.5, 0.5)
            } else {
                Colour::zero()
            }
        }
    }

    fn light_intensity(&self, ray: &Ray<WorldSpace, f64>, scene: &Scene) -> Colour {
        self.trace(ray, scene, 5)
    }
}

impl Renderer for BasicRenderer {
    fn render(&self, scene: &Scene, camera_instance: &CameraInstance, target: &mut Raster) {
        let target_rect = target.rect();

        for x in target_rect.bottom_left.x..target_rect.top_right.x {
            for y in target_rect.bottom_left.y..target_rect.top_right.y {
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
