use std::{marker::PhantomData, ops::Mul};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vector3<S, T> {
    pub x: T,
    pub y: T,
    pub z: T,
    _phantom: PhantomData<S>,
}

impl<S, T> Vector3<S, T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            _phantom: PhantomData,
        }
    }

    pub fn relabel<S2>(self) -> Vector3<S2, T> {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
            _phantom: PhantomData,
        }
    }
}

impl<S> Vector3<S, f64> {
    pub fn squared_len(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn len(&self) -> f64 {
        self.squared_len().sqrt()
    }

    pub fn normalise(&self) -> Self {
        let len = self.len();

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            _phantom: PhantomData,
        }
    }

    pub fn dot(&self, rhs: Vector3<S, f64>) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn is_close(&self, other: &Vector3<S, f64>) -> bool {
        let tolerance = 1e-6;

        return ((self.x - other.x).abs() < tolerance)
            && (((self.y - other.y).abs() < tolerance) && ((self.z - other.z).abs() < tolerance));
    }
}

impl<S> Mul<Vector3<S, f64>> for f64 {
    type Output = Vector3<S, f64>;

    fn mul(self, rhs: Vector3<S, f64>) -> Vector3<S, f64> {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            _phantom: PhantomData,
        }
    }
}
