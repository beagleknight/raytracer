use crate::patterns::PatternShape;
use colors::Color;
use tuples::Tuple;

pub struct RingPatternShape {
    pub a: Color,
    pub b: Color,
}

impl PatternShape for RingPatternShape {
    fn pattern_at(&self, point: &Tuple) -> Color {
        if (point.x.powf(2.0) + point.z.powf(2.0)).sqrt() % 2.0 == 0.0 {
            return self.a;
        }
        self.b
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::{ring::RingPatternShape, Pattern};
    use colors::Color;
    use lazy_static::lazy_static;
    use tuples::point;

    lazy_static! {
        static ref BLACK: Color = Color::new(0.0, 0.0, 0.0);
        static ref WHITE: Color = Color::new(1.0, 1.0, 1.0);
    }

    #[test]
    fn ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::new(Box::new(RingPatternShape {
            a: *WHITE,
            b: *BLACK,
        }));
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(pattern.shape.pattern_at(&point(1.0, 0.0, 0.0)), *BLACK);
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 1.0)), *BLACK);
        assert_eq!(pattern.shape.pattern_at(&point(0.708, 0.0, 0.708)), *BLACK);
    }
}
