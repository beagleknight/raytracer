use crate::shapes::Shape;
use rays::Ray;
use std::any::Any;
use tuples::{vector, Tuple};

pub struct Plane {}

impl Default for Plane {
    fn default() -> Plane {
        Plane {}
    }
}

impl Shape for Plane {
    fn intersects_at(&self, ray: &Ray) -> Option<[f64; 2]> {
        if ray.direction.y.abs() < 0.0001 {
            return None;
        }
        let t = -ray.origin.y / ray.direction.y;
        Some([t, t])
    }

    fn local_normal_at(&self, _local_point: &Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::shapes::{planes::Plane, Shape};
    use rays::Ray;
    use tuples::{point, vector};

    #[test]
    fn normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();
        let n1 = p.local_normal_at(&point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(&point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(&point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::default();
        let r = Ray {
            origin: point(0.0, 10.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs = p.intersects_at(&r);
        assert!(xs.is_none());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::default();
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs = p.intersects_at(&r);
        assert!(xs.is_none());
    }

    #[test]
    fn ray_intersecting_a_plane_from_above() {
        let p = Plane::default();
        let r = Ray {
            origin: point(0.0, 1.0, 0.0),
            direction: vector(0.0, -1.0, 0.0),
        };
        let xs = p.intersects_at(&r).unwrap();
        assert_eq!(xs[0], 1.0);
    }

    #[test]
    fn ray_intersecting_a_plane_from_below() {
        let p = Plane::default();
        let r = Ray {
            origin: point(0.0, -1.0, 0.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let xs = p.intersects_at(&r).unwrap();
        assert_eq!(xs[0], 1.0);
    }
}
