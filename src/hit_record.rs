use crate::{
    geometry::{
        Point2, Point3, Vector3,
        space::{ObjectSpace, TextureSpace},
    },
    material::Material,
};

pub struct HitRecord {
    pub p: Point3<ObjectSpace, f64>,
    pub normal: Vector3<ObjectSpace, f64>,
    pub material: Material, // TODO take ref
    pub texture_point: Point2<TextureSpace, f64>,
}

impl HitRecord {
    pub fn new(
        p: Point3<ObjectSpace, f64>,
        normal: Vector3<ObjectSpace, f64>,
        material: Material,
        texture_point: Point2<TextureSpace, f64>,
    ) -> Self {
        Self {
            p,
            normal,
            material,
            texture_point,
        }
    }
}

impl HitRecord {
    pub fn is_close(self, other: &HitRecord) -> bool {
        self.p.is_close(&other.p)
            & self.normal.is_close(&other.normal)
            & self.material.is_close(&other.material)
            & self.texture_point.is_close(&other.texture_point)
    }
}
