use std::{marker::PhantomData, ops::Mul};

use crate::{
    geometry::{Point3, Ray, Vector3},
    matrix::Matrix4x4,
};

#[derive(Clone)]
pub struct Transform<S1, S2> {
    matrix: Matrix4x4,
    inverse_matrix: Matrix4x4,
    _phantom1: PhantomData<S1>,
    _phantom2: PhantomData<S2>,
}

impl<S1, S2> Transform<S1, S2> {
    pub fn identity() -> Self {
        let matrix = Matrix4x4::identity();
        let inverse_matrix = Matrix4x4::identity();

        Self {
            matrix,
            inverse_matrix,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn translate(translate_x: f64, translate_y: f64, translate_z: f64) -> Self {
        let mut matrix = Matrix4x4::identity();
        matrix.m[0][3] = translate_x;
        matrix.m[1][3] = translate_y;
        matrix.m[2][3] = translate_z;

        let mut inverse_matrix = Matrix4x4::identity();
        inverse_matrix.m[0][3] = -translate_x;
        inverse_matrix.m[1][3] = -translate_y;
        inverse_matrix.m[2][3] = -translate_z;

        Self {
            matrix,
            inverse_matrix,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn scale(scale_x: f64, scale_y: f64, scale_z: f64) -> Self {
        let mut matrix = Matrix4x4::identity();
        matrix.m[0][0] = scale_x;
        matrix.m[1][1] = scale_y;
        matrix.m[2][2] = scale_z;
        matrix.m[3][3] = 1.0;

        let mut inverse_matrix = Matrix4x4::identity();
        inverse_matrix.m[0][0] = 1.0 / scale_x;
        inverse_matrix.m[1][1] = 1.0 / scale_y;
        inverse_matrix.m[2][2] = 1.0 / scale_z;

        Self {
            matrix,
            inverse_matrix,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn inverse(self) -> Transform<S2, S1> {
        Transform {
            matrix: self.inverse_matrix,
            inverse_matrix: self.matrix,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn perspective(fov: f64, near: f64, far: f64) -> Self {
        let mut matrix = Matrix4x4::identity();
        matrix.m[2][2] = far / (far - near);
        matrix.m[2][3] = -(far * near) / (far - near);
        matrix.m[3][2] = 1.0;
        matrix.m[3][3] = 0.0;

        let tan_angle = (fov / 2.0).tan();
        matrix.m[0][0] /= tan_angle;
        matrix.m[1][1] /= tan_angle;

        Self {
            matrix,
            inverse_matrix: matrix.inverse(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<S1, S2> Mul<Point3<S1, f64>> for Transform<S1, S2> {
    type Output = Point3<S2, f64>;

    fn mul(self, rhs: Point3<S1, f64>) -> Point3<S2, f64> {
        let p_homog = [rhs.x, rhs.y, rhs.z, 1.0];
        let out_homog = self.matrix * p_homog;

        Point3::new(out_homog[0], out_homog[1], out_homog[2])
    }
}

impl<S1, S2> Mul<Vector3<S1, f64>> for Transform<S1, S2> {
    type Output = Vector3<S2, f64>;

    fn mul(self, rhs: Vector3<S1, f64>) -> Vector3<S2, f64> {
        let p_homog = [rhs.x, rhs.y, rhs.z, 0.0];
        let out_homog = self.matrix * p_homog;

        Vector3::new(out_homog[0], out_homog[1], out_homog[2])
    }
}

impl<S1: Clone, S2: Clone> Mul<Ray<S1, f64>> for Transform<S1, S2> {
    type Output = Ray<S2, f64>;

    fn mul(self, rhs: Ray<S1, f64>) -> Ray<S2, f64> {
        let new_origin = self.clone() * rhs.origin;
        let new_direction = self.clone() * rhs.direction;

        Ray::new(new_origin, new_direction)
    }
}

impl<S1, S2, S3> Mul<Transform<S1, S2>> for Transform<S2, S3> {
    type Output = Transform<S1, S3>;

    fn mul(self, rhs: Transform<S1, S2>) -> Transform<S1, S3> {
        Transform {
            matrix: self.matrix * rhs.matrix,
            inverse_matrix: rhs.inverse_matrix * self.inverse_matrix,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::space::WorldSpace;

    use super::*;

    #[test]
    fn test_identity_on_point() {
        let p: Point3<WorldSpace, f64> = Point3::new(1.0, 2.0, 3.0);
        let t: Transform<WorldSpace, WorldSpace> = Transform::identity();

        let out = t * p;

        let expected = Point3::new(1.0, 2.0, 3.0);
        assert!(out.is_close(&expected));
    }

    #[test]
    fn test_identity_inverse() {
        let t: Transform<WorldSpace, WorldSpace> = Transform::identity();
        let t_inv = t.clone().inverse();

        let out = t_inv * t;

        let id = Matrix4x4::identity();
        assert!(out.matrix.is_close(&id));
        assert!(out.inverse_matrix.is_close(&id));
    }

    #[test]
    fn test_translate_on_point() {
        let p: Point3<WorldSpace, f64> = Point3::new(1.0, 2.0, 3.0);
        let t: Transform<WorldSpace, WorldSpace> = Transform::translate(2.0, 4.0, 6.0);

        let out = t * p;

        let expected = Point3::new(3.0, 6.0, 9.0);
        assert!(out.is_close(&expected));
    }

    #[test]
    fn test_translate_inverse() {
        let t: Transform<WorldSpace, WorldSpace> = Transform::translate(1.0, 2.0, 3.0);
        let t_inv = t.clone().inverse();

        let out = t_inv * t;

        let id = Matrix4x4::identity();
        assert!(out.matrix.is_close(&id));
        assert!(out.inverse_matrix.is_close(&id));
    }

    #[test]
    fn test_scale_on_point() {
        let p: Point3<WorldSpace, f64> = Point3::new(1.0, 2.0, 3.0);
        let t: Transform<WorldSpace, WorldSpace> = Transform::scale(2.0, 4.0, 6.0);

        let out = t * p;

        let expected = Point3::new(2.0, 8.0, 18.0);
        assert!(out.is_close(&expected));
    }

    #[test]
    fn test_scale_inverse() {
        let t: Transform<WorldSpace, WorldSpace> = Transform::scale(1.0, 2.0, 3.0);
        let t_inv = t.clone().inverse();

        let out = t_inv * t;

        let id = Matrix4x4::identity();
        assert!(out.matrix.is_close(&id));
        assert!(out.inverse_matrix.is_close(&id));
    }
}
