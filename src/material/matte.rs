use crate::{
    geometry::{Ray, space::WorldSpace},
    hit_record::HitRecord,
    random::random_in_unit_sphere,
};

#[derive(Clone)]
pub struct Matte {
    diffuse_reflection: f64,
}

impl Matte {
    pub fn new(diffuse_reflection: f64) -> Self {
        Self { diffuse_reflection }
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<Ray<WorldSpace, f64>> {
        let scatter_direction = hit_record.normal + random_in_unit_sphere();

        Some(Ray::new(
            hit_record.p.relabel(),
            scatter_direction.normalise().relabel(),
        )) // TODO coordinate system
    }

    pub fn is_close(&self, other: &Self) -> bool {
        let tolerance = 1e-6;

        (self.diffuse_reflection - other.diffuse_reflection).abs() < tolerance
    }
}
