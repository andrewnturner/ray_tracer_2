use crate::{
    geometry::{
        Ray, Transform,
        space::{ObjectSpace, ParentSpace, WorldSpace},
    },
    hit_record::HitRecord,
};

use super::element::Element;

pub struct Scene {
    element_instance: ElementInstance,
}

impl Scene {
    pub fn new(element_instance: ElementInstance) -> Self {
        Self { element_instance }
    }

    pub fn intersect(&self, ray: &Ray<WorldSpace, f64>) -> Option<HitRecord> {
        self.element_instance.intersect(&ray.relabel())
    }
}

pub struct ElementInstance {
    element: Element,
    parent_to_object: Transform<ParentSpace, ObjectSpace>,
}

impl ElementInstance {
    pub fn new(element: Element, parent_to_object: Transform<ParentSpace, ObjectSpace>) -> Self {
        Self {
            element,
            parent_to_object,
        }
    }

    pub fn intersect(&self, ray: &Ray<ParentSpace, f64>) -> Option<HitRecord> {
        let object_ray = self.parent_to_object.clone() * ray.clone();

        self.element.intersect(&object_ray)
    }
}
