use crate::object::Object;
use rays::Ray;
use tuples::{dot, Tuple};

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Object,
}

pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
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
            over_point: world_point + normalv * 0.00001,
            eyev,
            normalv,
            inside,
        }
    }
}

pub fn hit<'a>(intersections: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
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
    use crate::intersections::{hit, Intersection};
    use crate::object::Object;
    use crate::shape::test::TestShape;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let shape = TestShape::default();
        let o = Object::new(Box::new(shape));
        let i = Intersection { t: 3.5, object: &o };
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &o);
    }

    #[test]
    fn aggregating_intersections() {
        let shape = TestShape::default();
        let o = Object::new(Box::new(shape));
        let i1 = Intersection { t: 1.0, object: &o };
        let i2 = Intersection { t: 2.0, object: &o };
        let xs = vec![i1, i2];
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let shape = TestShape::default();
        let o = Object::new(Box::new(shape));
        let i1 = Intersection { t: 1.0, object: &o };
        let i2 = Intersection { t: 2.0, object: &o };
        let xs = vec![i1, i2];
        let i = hit(&xs);
        assert_eq!(i.unwrap(), &xs[0]);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let shape = TestShape::default();
        let o = Object::new(Box::new(shape));
        let i1 = Intersection {
            t: -1.0,
            object: &o,
        };
        let i2 = Intersection { t: 1.0, object: &o };
        let xs = vec![i1, i2];
        let i = hit(&xs);
        assert_eq!(i.unwrap(), &xs[1]);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let shape = TestShape::default();
        let o = Object::new(Box::new(shape));
        let i1 = Intersection {
            t: -2.0,
            object: &o,
        };
        let i2 = Intersection {
            t: -1.0,
            object: &o,
        };
        let xs = vec![i1, i2];
        let i = hit(&xs);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let shape = TestShape::default();
        let o = Object::new(Box::new(shape));
        let i1 = Intersection { t: 5.0, object: &o };
        let i2 = Intersection { t: 7.0, object: &o };
        let i3 = Intersection {
            t: -3.0,
            object: &o,
        };
        let i4 = Intersection { t: 2.0, object: &o };
        let xs = vec![i1, i2, i3, i4];
        let i = hit(&xs);
        assert_eq!(i.unwrap(), &xs[3]);
    }
}
