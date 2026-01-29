use crate::{
    geometry::{Point3, Vector3, space::ObjectSpace},
    material::Material,
};

pub struct HitRecord {
    pub p: Point3<ObjectSpace, f64>,
    pub normal: Vector3<ObjectSpace, f64>,
    pub material: Material, // TODO take ref
}

impl HitRecord {
    pub fn new(
        p: Point3<ObjectSpace, f64>,
        normal: Vector3<ObjectSpace, f64>,
        material: Material,
    ) -> Self {
        Self {
            p,
            normal,
            material,
        }
    }
}

impl HitRecord {
    pub fn is_close(self, other: &HitRecord) -> bool {
        self.p.is_close(&other.p)
            & self.normal.is_close(&other.normal)
            & self.material.is_close(&other.material)
    }
}
