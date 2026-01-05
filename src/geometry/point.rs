use std::marker::PhantomData;

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
}
