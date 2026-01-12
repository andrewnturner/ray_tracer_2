use crate::{
    geometry::{Ray, space::ObjectSpace},
    hit_record::HitRecord,
};

pub struct Sphere {
    radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    pub fn intersect(&self, ray: &Ray<ObjectSpace, f64>) -> Option<HitRecord> {
        // We have a ray R(t) = A + tB and sphere of radius r centred at O. We have an intersection
        // if there is t such that
        //     R(t).R(t) = r^2.
        // Expanding gives
        //     A.A + 2t(B.A) + (t^2)(B.B) - r^2 = 0,
        // a quadratic in t. We can then look at the discriminant to see whether there are
        // any solutions.
        let a = ray.direction.squared_len();
        let b = ray.direction.dot(ray.origin.into_vector()) * 2.0;
        let c = ray.origin.into_vector().squared_len() - (self.radius * self.radius);

        let d = (b * b) - (4.0 * a * c);

        if d < 0.0 {
            return None;
        }

        let sqrt_d = d.sqrt();
        let t = {
            let t_minus = (-b - sqrt_d) / (2.0 * a);
            if t_minus < 0.0 {
                (-b + sqrt_d) / (2.0 * a)
            } else {
                t_minus
            }
        };

        let p = ray.at(t);

        let normal = p.into_vector().normalise();

        Some(HitRecord::new(p, normal))
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Point3, Vector3};

    use super::*;

    #[test]
    fn test_sphere_intersect_outside() {
        let ray = Ray::new(Point3::new(-5.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let sphere = Sphere::new(1.0);

        let hit = sphere.intersect(&ray).unwrap();

        let expected = HitRecord::new(Point3::new(-1.0, 0.0, 0.0), Vector3::new(-1.0, 0.0, 0.0));
        assert!(hit.is_close(&expected));
    }

    #[test]
    fn test_sphere_intersect_inside() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let sphere = Sphere::new(2.0);

        let hit = sphere.intersect(&ray).unwrap();

        let expected = HitRecord::new(Point3::new(2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        assert!(hit.is_close(&expected));
    }

    #[test]
    fn test_sphere_intersect_miss() {
        let ray = Ray::new(Point3::new(-5.0, 2.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let sphere = Sphere::new(1.0);

        let hit = sphere.intersect(&ray);

        assert!(hit.is_none());
    }
}
