use crate::geometry::{Point3, space::ObjectSpace};

pub struct HitRecord {
    p: Point3<ObjectSpace, f64>,
}

impl HitRecord {
    pub fn new(p: Point3<ObjectSpace, f64>) -> Self {
        Self { p }
    }
}

impl HitRecord {
    pub fn is_close(self, other: &HitRecord) -> bool {
        self.p.is_close(&other.p)
    }
}
