use crate::{
    geometry::{Ray, space::ObjectSpace},
    hit_record::HitRecord,
    scene::{ElementInstance, element::Element},
};

pub struct ElementList {
    elements: Vec<ElementInstance>,
}

impl ElementList {
    pub fn new(elements: Vec<ElementInstance>) -> Self {
        Self { elements }
    }

    pub fn intersect(&self, ray: &Ray<ObjectSpace, f64>) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;

        for element_instance in &self.elements {
            if let Some(hit_record) = element_instance.intersect(&ray.relabel()) {
                if let Some(closest) = &closest_hit {
                    if closest.t < hit_record.t {
                        closest_hit = Some(hit_record);
                    }
                } else {
                    closest_hit = Some(hit_record);
                }
            }
        }

        closest_hit
    }
}
