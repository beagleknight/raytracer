use crate::{object::Object, patterns::Pattern};
use colors::Color;
use lights::PointLight;
use tuples::{dot, normalize, reflect, Tuple};
use uuid::Uuid;

pub struct Material {
    // pub id: Uuid,
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn glass() -> Material {
        let mut m = Material::default();
        m.transparency = 1.0;
        m.refractive_index = 1.5;
        m
    }

    pub fn lightning(
        &self,
        object: &Object,
        light: &PointLight,
        point: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Color {
        let base_color = match &self.pattern {
            Some(pattern) => pattern.pattern_at_object(&object, &point),
            None => self.color,
        };
        let effective_color = base_color * light.intensity;
        let lightv = normalize(&(light.position - *point));
        let ambient = effective_color * self.ambient;
        let light_dot_normal = dot(&lightv, normalv);
        let mut diffuse = Color::new(0.0, 0.0, 0.0);
        let mut specular = Color::new(0.0, 0.0, 0.0);
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
            // id: Uuid::new_v4(),
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        // self.id == other.id
        false
    }
}

impl std::fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        // write!(f, "Material {}", self.id)
        write!(f, "Material")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        materials::Material,
        object::Object,
        patterns::{stripes::StripesPatternShape, Pattern},
        shapes::spheres::Sphere,
    };
    use colors::Color;
    use lights::PointLight;
    use tuples::{point, vector};

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
        assert_eq!(m.reflective, 0.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let object = Object::new(Box::new(Sphere::default()));
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&object, &light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degree() {
        let object = Object::new(Box::new(Sphere::default()));
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, (2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&object, &light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degree() {
        let object = Object::new(Box::new(Sphere::default()));
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&object, &light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let object = Object::new(Box::new(Sphere::default()));
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, -(2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&object, &light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lightning_with_the_light_behind_the_surface() {
        let object = Object::new(Box::new(Sphere::default()));
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, 10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&object, &light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lightning_with_the_surface_in_shadow() {
        let object = Object::new(Box::new(Sphere::default()));
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight {
            position: point(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let result = m.lightning(&object, &light, &position, &eyev, &normalv, true);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lightning_with_a_pattern_applied() {
        let object = Object::new(Box::new(Sphere::default()));
        let mut m = Material::default();
        m.pattern = Some(Pattern::new(Box::new(StripesPatternShape {
            a: Color::new(1.0, 1.0, 1.0),
            b: Color::new(0.0, 0.0, 0.0),
        })));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let c1 = m.lightning(
            &object,
            &light,
            &point(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        let c2 = m.lightning(
            &object,
            &light,
            &point(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(c1, Color::new(1.0, 1.0, 1.0));
        assert_eq!(c2, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn transparency_and_refractive_index_for_the_default_material() {
        let m = Material::default();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}
