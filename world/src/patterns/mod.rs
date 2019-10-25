pub mod stripes;

use crate::object::Object;
use colors::Color;
use matrices::{inverse, matrix_tuple_multiply, IDENTITY};
use tuples::Tuple;

pub struct Pattern {
    pub transform: [[f64; 4]; 4],
    pub shape: Box<dyn PatternShape>,
}

impl Pattern {
    pub fn new(shape: Box<dyn PatternShape>) -> Pattern {
        Pattern {
            transform: IDENTITY,
            shape,
        }
    }

    pub fn pattern_at_object(&self, object: &Object, world_point: &Tuple) -> Color {
        let object_point = matrix_tuple_multiply(&inverse(&object.transform), &world_point);
        let pattern_point = matrix_tuple_multiply(&inverse(&self.transform), &object_point);
        return self.shape.pattern_at(&pattern_point);
    }
}

pub trait PatternShape {
    fn pattern_at(&self, point: &Tuple) -> Color;
}

#[cfg(test)]
mod tests {
    use crate::{
        object::Object,
        patterns::{Pattern, PatternShape},
        spheres::Sphere,
    };
    use colors::Color;
    use matrices::IDENTITY;
    use transformations::MatrixTransformations;
    use tuples::point;
    use tuples::Tuple;

    struct TestPatternShape {}

    impl PatternShape for TestPatternShape {
        fn pattern_at(&self, point: &Tuple) -> Color {
            Color::new(point.x, point.y, point.z)
        }
    }

    #[test]
    fn default_papttern_transformation() {
        let pattern = Pattern::new(Box::new(TestPatternShape {}));
        assert_eq!(pattern.transform, IDENTITY);
    }

    #[test]
    fn assigning_a_transformation() {
        let mut pattern = Pattern::new(Box::new(TestPatternShape {}));
        pattern.transform = IDENTITY.translate(1.0, 2.0, 3.0);
        assert_eq!(pattern.transform, IDENTITY.translate(1.0, 2.0, 3.0));
    }

    #[test]
    fn pattern_with_an_object_transformation() {
        let mut object = Object::new(Box::new(Sphere::default()));
        object.transform = IDENTITY.scale(2.0, 2.0, 2.0);
        let pattern = Pattern::new(Box::new(TestPatternShape {}));
        assert_eq!(
            pattern.pattern_at_object(&object, &point(2.0, 3.0, 4.0)),
            Color::new(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn pattern_with_a_pattern_transformation() {
        let object = Object::new(Box::new(Sphere::default()));
        let mut pattern = Pattern::new(Box::new(TestPatternShape {}));
        pattern.transform = IDENTITY.scale(2.0, 2.0, 2.0);
        assert_eq!(
            pattern.pattern_at_object(&object, &point(2.0, 3.0, 4.0)),
            Color::new(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Object::new(Box::new(Sphere::default()));
        object.transform = IDENTITY.scale(2.0, 2.0, 2.0);
        let mut pattern = Pattern::new(Box::new(TestPatternShape {}));
        pattern.transform = IDENTITY.translate(0.5, 1.0, 1.5);
        assert_eq!(
            pattern.pattern_at_object(&object, &point(2.5, 3.0, 3.5)),
            Color::new(0.75, 0.5, 0.25)
        );
    }
}
