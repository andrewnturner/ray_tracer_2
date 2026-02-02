use crate::{
    colour::Colour,
    geometry::{Point2, space::TextureSpace},
    texture::constant_texture::ConstantTexture,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Texture {
    Constant(ConstantTexture),
}

impl Texture {
    pub fn query(&self, _texture_point: Point2<TextureSpace, f64>) -> Colour {
        match self {
            Self::Constant(t) => t.colour,
        }
    }
}
