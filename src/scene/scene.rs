use super::element::Element;

pub struct Scene {
    element: Element,
}

impl Scene {
    pub fn new(element: Element) -> Self {
        Self { element }
    }
}
