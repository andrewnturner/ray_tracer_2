use crate::{
    camera::Camera,
    geometry::{
        Point2, Point3, Ray, Vector3,
        space::{TargetSpace, WorldSpace},
    },
};

pub struct Sampler {
    samples_per_pixel: u32,
}

impl Sampler {
    pub fn new(samples_per_pixel: u32) -> Self {
        Self { samples_per_pixel }
    }

    pub fn sample_pixel(
        &self,
        pixel: Point2<TargetSpace, u32>,
        camera: &Camera,
    ) -> impl Iterator<Item = Sample> {
        SampleIterator::new(self.samples_per_pixel, pixel, camera)
    }
}

struct SampleIterator<'a> {
    num_samples: u32,
    pixel: Point2<TargetSpace, u32>,
    current_sample: u32,
    camera: &'a Camera,
}

impl<'a> SampleIterator<'a> {
    fn new(num_samples: u32, pixel: Point2<TargetSpace, u32>, camera: &'a Camera) -> Self {
        Self {
            num_samples,
            pixel,
            current_sample: 0,
            camera,
        }
    }
}

impl<'a> Iterator for SampleIterator<'a> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample >= 1 {
            return None;
        }

        self.current_sample += 1;

        let target_point = Point2::new(self.pixel.x as f64, self.pixel.y as f64);
        let ray = self.camera.generate_ray(target_point);

        Some(Sample { target_point, ray })
    }
}

pub struct Sample {
    pub target_point: Point2<TargetSpace, f64>,
    pub ray: Ray<WorldSpace, f64>,
}
