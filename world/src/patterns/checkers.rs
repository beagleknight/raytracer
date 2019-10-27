use crate::patterns::PatternShape;
use colors::Color;
use tuples::Tuple;

pub struct CheckersPatternShape {
    pub a: Color,
    pub b: Color,
}

impl PatternShape for CheckersPatternShape {
    fn pattern_at(&self, point: &Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            return self.a;
        }
        self.b
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::{checkers::CheckersPatternShape, Pattern};
    use colors::Color;
    use lazy_static::lazy_static;
    use tuples::point;

    lazy_static! {
        static ref BLACK: Color = Color::new(0.0, 0.0, 0.0);
        static ref WHITE: Color = Color::new(1.0, 1.0, 1.0);
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::new(Box::new(CheckersPatternShape {
            a: *WHITE,
            b: *BLACK,
        }));
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.99, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(1.01, 0.0, 0.0)), *BLACK);
    }
    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::new(Box::new(CheckersPatternShape {
            a: *WHITE,
            b: *BLACK,
        }));
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.99, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 1.01, 0.0)), *BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::new(Box::new(CheckersPatternShape {
            a: *WHITE,
            b: *BLACK,
        }));
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 0.99)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 1.01)), *BLACK);
    }
}
