use std::{marker::PhantomData, ops::Add};

use num_traits::Num;

use crate::geometry::Vector3;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point2<S, T> {
    pub x: T,
    pub y: T,
    _phantom: PhantomData<S>,
}

impl<S, T> Point2<S, T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
            _phantom: PhantomData,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point3<S, T> {
    pub x: T,
    pub y: T,
    pub z: T,
    _phantom: PhantomData<S>,
}

impl<S, T> Point3<S, T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            _phantom: PhantomData,
        }
    }

    pub fn into_vector(self) -> Vector3<S, T> {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn relabel<S2>(self) -> Point3<S2, T> {
        Point3 {
            x: self.x,
            y: self.y,
            z: self.z,
            _phantom: PhantomData,
        }
    }
}

impl<S> Point3<S, f64> {
    pub fn is_close(&self, other: &Point3<S, f64>) -> bool {
        let tolerance = 1e-6;

        return ((self.x - other.x).abs() < tolerance)
            && (((self.y - other.y).abs() < tolerance) && ((self.z - other.z).abs() < tolerance));
    }
}

impl<S> Add<Vector3<S, f64>> for Point3<S, f64> {
    type Output = Point3<S, f64>;

    fn add(self, rhs: Vector3<S, f64>) -> Point3<S, f64> {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            _phantom: PhantomData,
        }
    }
}
