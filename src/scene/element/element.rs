use crate::{
    geometry::{Ray, space::ObjectSpace},
    hit_record::HitRecord,
};

use super::sphere::Sphere;

pub enum Element {
    Sphere(Sphere),
}

impl Element {
    pub fn intersect(&self, ray: &Ray<ObjectSpace, f64>) -> Option<HitRecord> {
        match self {
            Self::Sphere(sphere) => sphere.intersect(ray),
        }
    }
}
