use std::fmt::Debug;

pub trait Object {}

#[derive(Debug, PartialEq)]
pub struct Intersection<'a, T: Object + Debug + PartialEq> {
    pub t: f64,
    pub object: &'a T,
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
    use crate::*;

    #[derive(Debug, PartialEq)]
    struct Sphere {}

    impl Object for Sphere {}

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
