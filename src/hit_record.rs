use crate::geometry::{Point3, Vector3, space::ObjectSpace};

pub struct HitRecord {
    pub p: Point3<ObjectSpace, f64>,
    pub normal: Vector3<ObjectSpace, f64>,
}

impl HitRecord {
    pub fn new(p: Point3<ObjectSpace, f64>, normal: Vector3<ObjectSpace, f64>) -> Self {
        Self { p, normal }
    }
}

impl HitRecord {
    pub fn is_close(self, other: &HitRecord) -> bool {
        self.p.is_close(&other.p) & self.normal.is_close(&other.normal)
    }
}
