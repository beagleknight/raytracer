use colors::Color;
use tuples::Tuple;

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use colors::Color;
    use tuples::point;

    #[test]
    fn point_light_has_a_position_and_intensity() {
        let position = point(0.0, 0.0, 0.0);
        let intensity = Color::new(1.0, 1.0, 1.0);
        let light = PointLight {
            position,
            intensity,
        };
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
