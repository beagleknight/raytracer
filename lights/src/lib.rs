use colors::Color;
use tuples::Tuple;

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use colors::color;
    use tuples::point;

    #[test]
    fn point_light_has_a_position_and_intensity() {
        let position = point(0.0, 0.0, 0.0);
        let intensity = color(1.0, 1.0, 1.0);
        let light = PointLight {
            position,
            intensity,
        };
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
