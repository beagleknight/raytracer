use crate::patterns::PatternShape;
use colors::Color;
use tuples::Tuple;

pub struct StripesPatternShape {
    pub a: Color,
    pub b: Color,
}

impl PatternShape for StripesPatternShape {
    fn pattern_at(&self, point: &Tuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            return self.a;
        }
        self.b
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::{stripes::StripesPatternShape, Pattern};
    use colors::Color;
    use lazy_static::lazy_static;
    use tuples::point;

    lazy_static! {
        static ref BLACK: Color = Color::new(0.0, 0.0, 0.0);
        static ref WHITE: Color = Color::new(1.0, 1.0, 1.0);
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::new(Box::new(StripesPatternShape {
            a: *WHITE,
            b: *BLACK,
        }));
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 1.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 2.0, 0.0)), *WHITE);
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::new(Box::new(StripesPatternShape {
            a: *WHITE,
            b: *BLACK,
        }));
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 1.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 2.0)), *WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Pattern::new(Box::new(StripesPatternShape {
            a: *WHITE,
            b: *BLACK,
        }));
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.9, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(1.0, 0.0, 0.0)), *BLACK);
        assert_eq!(pattern.shape.pattern_at(&point(-0.1, 0.0, 0.0)), *BLACK);
        assert_eq!(pattern.shape.pattern_at(&point(-1.0, 0.0, 0.0)), *BLACK);
        assert_eq!(pattern.shape.pattern_at(&point(-1.1, 0.0, 0.0)), *WHITE);
    }
}
