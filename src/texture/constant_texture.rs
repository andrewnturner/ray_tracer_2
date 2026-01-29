use crate::colour::Colour;

#[derive(Clone, Copy)]
pub struct ConstantTexture {
    pub colour: Colour,
}

impl ConstantTexture {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}
