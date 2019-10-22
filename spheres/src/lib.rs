use uuid::Uuid;

use intersections::{Intersection, Object};
use materials::Material;
use matrices::{inverse, matrix_tuple_multiply, transpose, IDENTITY};
use rays::Ray;
use tuples::{dot, normalize, point, Tuple};

#[derive(Debug, Clone)]
pub struct Sphere {
    id: Uuid,
    pub transform: [[f64; 4]; 4],
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            id: Uuid::new_v4(),
            transform: IDENTITY,
            material: Material::default(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<[Intersection<Sphere>; 2]> {
        let ray = ray.transform(&inverse(&self.transform));
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);
        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Some([
            Intersection {
                t: t1,
                object: self,
            },
            Intersection {
                t: t2,
                object: self,
            },
        ])
    }
}

impl Object for Sphere {
    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let inverse_transform = inverse(&self.transform);
        let transpose_inverse_transform = transpose(&inverse_transform);
        let object_point = matrix_tuple_multiply(&inverse_transform, world_point);
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = matrix_tuple_multiply(&transpose_inverse_transform, &object_normal);
        world_normal.w = 0.0;
        normalize(&world_normal)
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::*;
    use matrices::IDENTITY;
    use rays::Ray;
    use transformations::MatrixTransformations;
    use tuples::{point, vector};

    #[test]
    fn ray_intersects_a_sphere_at_two_points() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert!(xs.is_none());
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, IDENTITY);
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::new();
        let t = IDENTITY.translate(2.0, 3.0, 4.0);
        s.transform = t;
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s = Sphere::new();
        s.transform = IDENTITY.scale(2.0, 2.0, 2.0);
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s = Sphere::new();
        s.transform = IDENTITY.translate(5.0, 0.0, 0.0);
        let xs = s.intersect(&r);
        assert!(xs.is_none())
    }

    #[test]
    fn normal_on_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(&point(
            (3.0 as f64).sqrt() / 3.0,
            (3.0 as f64).sqrt() / 3.0,
            (3.0 as f64).sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            vector(
                (3.0 as f64).sqrt() / 3.0,
                (3.0 as f64).sqrt() / 3.0,
                (3.0 as f64).sqrt() / 3.0
            )
        );
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(&point(
            (3.0 as f64).sqrt() / 3.0,
            (3.0 as f64).sqrt() / 3.0,
            (3.0 as f64).sqrt() / 3.0,
        ));
        assert_eq!(n, normalize(&n));
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.transform = IDENTITY.translate(0.0, 1.0, 0.0);
        let n = s.normal_at(&point(0.0, 1.70711, -0.70711));
        assert_eq!(n, vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        s.transform = IDENTITY.rotate_z(PI / 5.0).scale(1.0, 0.5, 1.0);
        let n = s.normal_at(&point(
            0.0,
            (2.0 as f64).sqrt() / 2.0,
            -((2.0 as f64).sqrt() / 2.0),
        ));
        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_a_default_material() {
        let s = Sphere::new();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::new();
        let i = Intersection {
            t: 4.0,
            object: &shape,
        };
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.object, &shape);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::new();
        let i = Intersection {
            t: 4.0,
            object: &shape,
        };
        let comps = i.prepare_computations(&r);
        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::new();
        let i = Intersection {
            t: 1.0,
            object: &shape,
        };
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut shape = Sphere::new();
        shape.transform = IDENTITY.translate(0.0, 0.0, 1.0);
        let i = Intersection {
            t: 5.0,
            object: &shape,
        };
        let comps = i.prepare_computations(&r);
        assert!(comps.over_point.z < -0.00001 / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
