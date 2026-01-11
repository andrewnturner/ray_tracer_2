use super::point::Point3;
use super::vector::Vector3;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ray<S, T> {
    pub origin: Point3<S, T>,
    pub direction: Vector3<S, T>,
}

impl<S, T> Ray<S, T> {
    pub fn new(origin: Point3<S, T>, direction: Vector3<S, T>) -> Self {
        Self { origin, direction }
    }

    pub fn relabel<S2>(self) -> Ray<S2, T> {
        Ray {
            origin: self.origin.relabel(),
            direction: self.direction.relabel(),
        }
    }
}

impl<S: Clone> Ray<S, f64> {
    pub fn at(&self, t: f64) -> Point3<S, f64> {
        self.origin.clone() + (t * self.direction.clone())
    }
}
