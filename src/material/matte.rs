use crate::{
    geometry::{Ray, space::WorldSpace},
    hit_record::HitRecord,
    random::random_in_unit_sphere,
    texture::Texture,
};

#[derive(Clone)]
pub struct Matte {
    pub texture: Texture,
}

impl Matte {
    pub fn new(texture: Texture) -> Self {
        Self { texture }
    }

    pub fn scatter(&self, hit_record: &HitRecord) -> Option<Ray<WorldSpace, f64>> {
        let scatter_direction = hit_record.normal + random_in_unit_sphere();

        Some(Ray::new(
            hit_record.p.relabel(),
            scatter_direction.normalise().relabel(),
        )) // TODO coordinate system
    }
}

#[cfg(test)]
impl Matte {
    pub fn is_close(&self, _other: &Self) -> bool {
        // TODO should take into accoutn texture
        true
    }
}
