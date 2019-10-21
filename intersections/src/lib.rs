use std::fmt::Debug;

use rays::Ray;
use tuples::{dot, Tuple};

pub trait Object {
    fn normal_at(&self, world_point: &Tuple) -> Tuple;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a, T: Object + Debug + PartialEq> {
    pub t: f64,
    pub object: &'a T,
}

pub struct Computations<'a, T: Object + Debug + PartialEq> {
    pub t: f64,
    pub object: &'a T,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<'a, T: Object + Debug + PartialEq> Intersection<'a, T> {
    pub fn prepare_computations(&self, ray: &Ray) -> Computations<T> {
        let world_point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(&world_point);
        let mut inside = false;

        if dot(&normalv, &eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        Computations {
            t: self.t,
            object: self.object,
            point: world_point,
            eyev,
            normalv,
            inside,
        }
    }
}

pub fn hit<'a, T: Object + Debug + PartialEq>(
    intersections: &'a [Intersection<T>],
) -> Option<&'a Intersection<'a, T>> {
    let mut positive_intersections: Vec<_> = intersections
        .iter()
        .filter(|x| x.t.is_sign_positive())
        .collect();

    if positive_intersections.len() == 0 {
        return None;
    }

    positive_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    Some(&positive_intersections[0])
}

#[cfg(test)]
mod tests {
    use tuples::vector;

    use crate::*;

    #[derive(Debug, PartialEq)]
    struct Sphere {}

    impl Object for Sphere {
        fn normal_at(&self, _world_point: &Tuple) -> Tuple {
            vector(0.0, 0.0, 0.0)
        }
    }

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere {};
        let i = Intersection { t: 3.5, object: &s };
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere {};
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        let xs = vec![i1, i2];
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere {};
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        let xs = vec![i1, i2];
        let i = hit(&xs);
        assert_eq!(i.unwrap(), &xs[0]);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere {};
        let i1 = Intersection {
            t: -1.0,
            object: &s,
        };
        let i2 = Intersection { t: 1.0, object: &s };
        let xs = vec![i1, i2];
        let i = hit(&xs);
        assert_eq!(i.unwrap(), &xs[1]);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere {};
        let i1 = Intersection {
            t: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            t: -1.0,
            object: &s,
        };
        let xs = vec![i1, i2];
        let i = hit(&xs);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere {};
        let i1 = Intersection { t: 5.0, object: &s };
        let i2 = Intersection { t: 7.0, object: &s };
        let i3 = Intersection {
            t: -3.0,
            object: &s,
        };
        let i4 = Intersection { t: 2.0, object: &s };
        let xs = vec![i1, i2, i3, i4];
        let i = hit(&xs);
        assert_eq!(i.unwrap(), &xs[3]);
    }
}
