use crate::{
    geometry::{Ray, space::ObjectSpace},
    hit_record::HitRecord,
    scene::element::ElementList,
};

use super::sphere::Sphere;

pub enum Element {
    ElementList(ElementList),
    Sphere(Sphere),
}

impl Element {
    pub fn intersect(&self, ray: &Ray<ObjectSpace, f64>) -> Option<HitRecord> {
        match self {
            Self::Sphere(sphere) => sphere.intersect(ray),
            Self::ElementList(element_list) => element_list.intersect(ray),
        }
    }
}
