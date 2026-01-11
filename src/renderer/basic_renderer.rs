use crate::{
    camera::Camera,
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
            sampler: Sampler::new(300),
        }
    }

    fn light_intensity(&self, ray: Ray<WorldSpace, f64>, scene: &Scene) -> Colour {
        Colour::new(1.0, 0.0, 0.0)
    }
}

impl Renderer for BasicRenderer {
    fn render(&self, scene: &Scene, camera: &Camera, target: &mut Target) {
        let target_rect = target.rect();

        for x in target_rect.top_left.x..target_rect.bottom_right.x {
            for y in target_rect.top_left.y..target_rect.bottom_right.y {
                let current_pixel = Point2::new(x, y);
                let current_pixel_f64 = Point2::new(x as f64, y as f64);

                let ray = camera.generate_ray(current_pixel_f64);
                println!("x={:?}, y={:?}, ray={:?}", x, y, ray);

                let mut pixel_colour = Colour::zero();
                let mut sample_count = 0;
                for sample in self.sampler.sample_pixel(current_pixel, camera) {
                    let ray = sample.ray;
                    let intensity = self.light_intensity(ray, scene);

                    pixel_colour += intensity;
                    sample_count += 1;
                }

                pixel_colour /= sample_count as f64;

                target.set_pixel(current_pixel, pixel_colour);
            }
        }
    }
}
