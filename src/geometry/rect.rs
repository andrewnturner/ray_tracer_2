use super::point::Point2;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rect<S, T> {
    pub bottom_left: Point2<S, T>,
    pub top_right: Point2<S, T>,
}

impl<S, T> Rect<S, T> {
    pub fn new(bottom_left: Point2<S, T>, top_right: Point2<S, T>) -> Self {
        Self {
            bottom_left,
            top_right,
        }
    }
}

impl<S> Rect<S, f64> {
    pub fn width(&self) -> f64 {
        self.top_right.x - self.bottom_left.x
    }

    pub fn height(&self) -> f64 {
        self.top_right.y - self.bottom_left.y
    }
}
