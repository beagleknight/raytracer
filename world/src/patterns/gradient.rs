use crate::patterns::PatternShape;
use colors::Color;
use tuples::Tuple;

pub struct GradientPatternShape {
    pub a: Color,
    pub b: Color,
}

impl PatternShape for GradientPatternShape {
    fn pattern_at(&self, point: &Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        self.a + distance * fraction
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::{gradient::GradientPatternShape, Pattern};
    use colors::Color;
    use lazy_static::lazy_static;
    use tuples::point;

    lazy_static! {
        static ref BLACK: Color = Color::new(0.0, 0.0, 0.0);
        static ref WHITE: Color = Color::new(1.0, 1.0, 1.0);
    }

    #[test]
    fn gradient_pattern_interpolates_between_colors() {
        let pattern = Pattern::new(Box::new(GradientPatternShape {
            a: *WHITE,
            b: *BLACK,
        }));
        assert_eq!(pattern.shape.pattern_at(&point(0.0, 0.0, 0.0)), *WHITE);
        assert_eq!(
            pattern.shape.pattern_at(&point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.shape.pattern_at(&point(0.5, 2.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.shape.pattern_at(&point(0.75, 2.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
