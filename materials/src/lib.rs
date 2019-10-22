use colors::{color, Color};
use lights::PointLight;
use tuples::{dot, normalize, reflect, Tuple};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn lightning(
        &self,
        light: &PointLight,
        point: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = normalize(&(light.position - *point));
        let ambient = effective_color * self.ambient;
        let light_dot_normal = dot(&lightv, normalv);
        let mut diffuse = color(0.0, 0.0, 0.0);
        let mut specular = color(0.0, 0.0, 0.0);
        if light_dot_normal > 0.0 {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = reflect(&-lightv, normalv);
            let reflect_dot_eye = dot(&reflectv, eyev);

            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        if in_shadow {
            return ambient;
        }
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use lights::PointLight;
    use tuples::{point, vector};

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color, color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, -10.0),
            intensity: color(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degree() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, (2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, -10.0),
            intensity: color(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degree() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 10.0, -10.0),
            intensity: color(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, -(2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 10.0, -10.0),
            intensity: color(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lightning_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, 10.0),
            intensity: color(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lightning_with_the_surface_in_shadow() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, -10.0),
            intensity: color(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&light, &position, &eyev, &normalv, true);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }
}
