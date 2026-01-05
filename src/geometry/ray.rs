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
}
