use crate::{
    geometry::{Ray, space::WorldSpace},
    hit_record::HitRecord,
    material::Matte,
    texture::Texture,
};

#[derive(Clone)]
pub enum Material {
    Matte(Matte),
}
impl Material {
    pub fn scatter(&self, hit_record: &HitRecord) -> Option<Ray<WorldSpace, f64>> {
        match &self {
            Self::Matte(m) => m.scatter(hit_record),
        }
    }

    pub fn texture(&self) -> &Texture {
        match &self {
            Self::Matte(m) => &m.texture,
        }
    }
}

impl Material {
    pub fn is_close(&self, other: &Self) -> bool {
        match self {
            Self::Matte(m) => {
                if let Self::Matte(m2) = other {
                    m.is_close(m2)
                } else {
                    false
                }
            }
        }
    }
}
