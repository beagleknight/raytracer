use crate::object::Object;
use colors::Color;
use matrices::{inverse, matrix_tuple_multiply, IDENTITY};
use tuples::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
    pub transform: [[f64; 4]; 4],
}

impl Pattern {
    pub fn new(a: Color, b: Color) -> Pattern {
        Pattern {
            a,
            b,
            transform: IDENTITY,
        }
    }

    pub fn stripe_at(&self, point: &Tuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            return self.a;
        }
        self.b
    }

    pub fn stripe_at_object(&self, object: &Object, world_point: &Tuple) -> Color {
        let object_point = matrix_tuple_multiply(&inverse(&object.transform), &world_point);
        let pattern_point = matrix_tuple_multiply(&inverse(&self.transform), &object_point);
        return self.stripe_at(&pattern_point);
    }
}

#[cfg(test)]
mod tests {
    use crate::{object::Object, patterns::Pattern, spheres::Sphere};
    use colors::{color, Color};
    use lazy_static::lazy_static;
    use matrices::IDENTITY;
    use transformations::MatrixTransformations;
    use tuples::point;

    lazy_static! {
        static ref BLACK: Color = color(0.0, 0.0, 0.0);
        static ref WHITE: Color = color(1.0, 1.0, 1.0);
    }

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Pattern::new(*WHITE, *BLACK);
        assert_eq!(pattern.a, *WHITE);
        assert_eq!(pattern.b, *BLACK);
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::new(*WHITE, *BLACK);
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.stripe_at(&point(0.0, 1.0, 0.0)), *WHITE);
        assert_eq!(pattern.stripe_at(&point(0.0, 2.0, 0.0)), *WHITE);
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::new(*WHITE, *BLACK);
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 1.0)), *WHITE);
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 2.0)), *WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Pattern::new(*WHITE, *BLACK);
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.stripe_at(&point(0.9, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.stripe_at(&point(1.0, 0.0, 0.0)), *BLACK);
        assert_eq!(pattern.stripe_at(&point(-0.1, 0.0, 0.0)), *BLACK);
        assert_eq!(pattern.stripe_at(&point(-1.0, 0.0, 0.0)), *BLACK);
        assert_eq!(pattern.stripe_at(&point(-1.1, 0.0, 0.0)), *WHITE);
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Object::new(Box::new(Sphere::default()));
        object.transform = IDENTITY.scale(2.0, 2.0, 2.0);
        let pattern = Pattern::new(*WHITE, *BLACK);
        assert_eq!(
            pattern.stripe_at_object(&object, &point(1.5, 0.0, 0.0)),
            *WHITE
        );
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Object::new(Box::new(Sphere::default()));
        let mut pattern = Pattern::new(*WHITE, *BLACK);
        pattern.transform = IDENTITY.scale(2.0, 2.0, 2.0);
        assert_eq!(
            pattern.stripe_at_object(&object, &point(1.5, 0.0, 0.0)),
            *WHITE
        );
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Object::new(Box::new(Sphere::default()));
        object.transform = IDENTITY.scale(2.0, 2.0, 2.0);
        let mut pattern = Pattern::new(*WHITE, *BLACK);
        pattern.transform = IDENTITY.translate(0.5, 0.0, 0.0);
        assert_eq!(
            pattern.stripe_at_object(&object, &point(2.5, 0.0, 0.0)),
            *WHITE
        );
    }
}
