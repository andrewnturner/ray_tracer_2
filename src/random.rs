use rand::{Rng, rng};

use crate::geometry::Vector3;

pub fn random_in_unit_sphere<S>() -> Vector3<S, f64> {
    let mut rng = rng();

    loop {
        let p = Vector3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        );

        if p.squared_len() < 1.0 {
            return p;
        }
    }
}
