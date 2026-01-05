use crate::{
    geometry::{
        Point2, Point3, Ray, Vector3,
        space::{SceneSpace, TargetSpace},
    },
    scene::Camera,
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
        SampleIterator::new(self.samples_per_pixel, pixel)
    }
}

struct SampleIterator {
    num_samples: u32,
    pixel: Point2<TargetSpace, u32>,
    current_sample: u32,
}

impl SampleIterator {
    fn new(num_samples: u32, pixel: Point2<TargetSpace, u32>) -> Self {
        Self {
            num_samples,
            pixel,
            current_sample: 0,
        }
    }
}

impl Iterator for SampleIterator {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample >= 1 {
            return None;
        }

        self.current_sample += 1;

        let target_point = Point2::new(self.pixel.x as f64, self.pixel.y as f64);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));

        Some(Sample { target_point, ray })
    }
}

pub struct Sample {
    pub target_point: Point2<TargetSpace, f64>,
    pub ray: Ray<SceneSpace, f64>,
}
