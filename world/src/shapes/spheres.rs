use crate::materials::Material;
use crate::object::Object;
use crate::shapes::Shape;
use rays::Ray;
use std::any::Any;
use std::rc::Rc;
use tuples::{dot, point, Tuple};

pub struct Sphere {}

impl Sphere {
    pub fn glass() -> Object {
        let shape = Sphere::default();
        let mut object = Object::new(Box::new(shape));
        object.material = Rc::new(Material::glass());
        object
    }
}

impl Shape for Sphere {
    fn intersects_at(&self, ray: &Ray) -> Option<[f64; 2]> {
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

        Some([t1, t2])
    }

    fn local_normal_at(&self, local_point: &Tuple) -> Tuple {
        *local_point - point(0.0, 0.0, 0.0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere {}
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::spheres::Sphere;
    use crate::{intersections::Intersection, object::Object};
    use matrices::IDENTITY;
    use rays::Ray;
    use transformations::MatrixTransformations;
    use tuples::{normalize, point, vector};

    #[test]
    fn ray_intersects_a_sphere_at_two_points() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let xo = o.intersect(&r).unwrap();
        assert_eq!(xo[0].t, 4.0);
        assert_eq!(xo[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let xo = o.intersect(&r).unwrap();
        assert_eq!(xo[0].t, 5.0);
        assert_eq!(xo[1].t, 5.0);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let xo = o.intersect(&r);
        assert!(xo.is_none());
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let xo = o.intersect(&r).unwrap();
        assert_eq!(xo[0].t, -1.0);
        assert_eq!(xo[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let xo = o.intersect(&r).unwrap();
        assert_eq!(xo[0].t, -6.0);
        assert_eq!(xo[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let xo = o.intersect(&r).unwrap();
        assert_eq!(xo[0].object, &o);
        assert_eq!(xo[1].object, &o);
    }

    #[test]
    fn normal_on_sphere_at_a_point_on_the_x_axis() {
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let n = o.normal_at(&point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_a_point_on_the_y_axis() {
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let n = o.normal_at(&point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_a_point_on_the_z_axis() {
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let n = o.normal_at(&point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_a_nonaxial_point() {
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let n = o.normal_at(&point(
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
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let n = o.normal_at(&point(
            (3.0 as f64).sqrt() / 3.0,
            (3.0 as f64).sqrt() / 3.0,
            (3.0 as f64).sqrt() / 3.0,
        ));
        assert_eq!(n, normalize(&n));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let i = Intersection { t: 4.0, object: &o };
        let comps = i.prepare_computations(&r, &[&i]);
        assert_eq!(comps.object, &o);
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
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let i = Intersection { t: 4.0, object: &o };
        let comps = i.prepare_computations(&r, &[&i]);
        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let shape = Sphere::default();
        let o = Object::new(Box::new(shape));
        let i = Intersection { t: 1.0, object: &o };
        let comps = i.prepare_computations(&r, &[&i]);
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
        let shape = Sphere::default();
        let mut o = Object::new(Box::new(shape));
        o.transform = IDENTITY.translate(0.0, 0.0, 1.0);
        let i = Intersection { t: 5.0, object: &o };
        let comps = i.prepare_computations(&r, &[&i]);
        assert!(comps.over_point.z < -0.0001 / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = Sphere::glass();
        assert_eq!(s.transform, IDENTITY);
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }
}
