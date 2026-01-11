use std::marker::PhantomData;

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
}

impl<S> Vector3<S, f64> {
    pub fn len(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
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
}
