use super::point::Point2;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rect<S, T> {
    pub top_left: Point2<S, T>,
    pub bottom_right: Point2<S, T>,
}

impl<S, T> Rect<S, T> {
    pub fn new(top_left: Point2<S, T>, bottom_right: Point2<S, T>) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl<S> Rect<S, u32> {
    pub fn width(&self) -> u32 {
        self.bottom_right.x - self.top_left.x
    }

    pub fn height(&self) -> u32 {
        self.bottom_right.y - self.top_left.y
    }
}
