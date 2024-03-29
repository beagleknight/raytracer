pub mod camera;
pub mod intersections;
pub mod materials;
pub mod object;
pub mod patterns;
pub mod shapes;

use crate::intersections::{hit, Computations, Intersection};
use crate::materials::Material;
use crate::object::Object;
use crate::shapes::spheres::Sphere;
use colors::Color;
use lights::PointLight;
use matrices::IDENTITY;
use rays::Ray;
use std::rc::Rc;
use transformations::MatrixTransformations;
use tuples::{magnitude, normalize, point, Tuple};

pub const RAY_LIMIT: u32 = 5;

pub struct World {
    pub light_source: Option<PointLight>,
    pub objects: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        World {
            light_source: None,
            objects: vec![],
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut result = vec![];
        for object in self.objects.iter() {
            if let Some(intersections) = object.intersect(ray) {
                result.extend_from_slice(&intersections);
            }
        }
        result.sort_by(|Intersection { t: ta, .. }, Intersection { t: tb, .. }| {
            ta.partial_cmp(tb).unwrap()
        });
        result
    }

    pub fn shade_hit(&self, comps: &Computations, remaining: u32) -> Color {
        let surface = comps.object.material.lightning(
            &comps.object,
            &self.light_source.as_ref().unwrap(),
            &comps.point,
            &comps.eyev,
            &comps.normalv,
            self.is_shadowed(&comps.over_point),
        );
        let reflected = self.reflected_color(comps, remaining);
        surface + reflected
    }

    pub fn color_at(&self, ray: &Ray, remaining: u32) -> Color {
        let intersections = self.intersect(ray);

        match hit(&intersections) {
            Some(intersection) => {
                let comps = intersection.prepare_computations(ray, &[intersection]);
                self.shade_hit(&comps, remaining)
            }
            None => Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        let v = self.light_source.as_ref().unwrap().position - *point;
        let distance = magnitude(&v);
        let direction = normalize(&v);

        let ray = Ray {
            origin: *point,
            direction,
        };

        let intersections = self.intersect(&ray);

        match hit(&intersections) {
            Some(intersection) => intersection.t < distance,
            None => false,
        }
    }

    pub fn reflected_color(&self, comps: &Computations, remaining: u32) -> Color {
        if remaining <= 0 || comps.object.material.reflective == 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining - 1);

        color * comps.object.material.reflective
    }
}

impl Default for World {
    fn default() -> World {
        let light = PointLight {
            position: point(-10.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        let s1 = Sphere::default();
        let mut o1 = Object::new(Box::new(s1));
        let mut material = Material::default();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        o1.material = Rc::new(material);
        let s2 = Sphere::default();
        let mut o2 = Object::new(Box::new(s2));
        o2.transform = IDENTITY.scale(0.5, 0.5, 0.5);
        World {
            light_source: Some(light),
            objects: vec![o1, o2],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::intersections::Intersection;
    use crate::materials::Material;
    use crate::object::Object;
    use crate::shapes::{planes::Plane, spheres::Sphere};
    use crate::{World, RAY_LIMIT};
    use colors::Color;
    use lights::PointLight;
    use matrices::IDENTITY;
    use rays::Ray;
    use std::rc::Rc;
    use transformations::MatrixTransformations;
    use tuples::{point, vector};

    #[test]
    fn creating_a_world() {
        let w = World::new();
        assert_eq!(w.objects, vec![]);
        assert_eq!(w.light_source, None);
    }

    #[test]
    fn default_world() {
        let w = World::default();
        let light = PointLight {
            position: point(-10.0, 10.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        };
        assert_eq!(w.light_source, Some(light));
        assert_eq!(w.objects[0].material.color, Color::new(0.8, 1.0, 0.6));
        assert_eq!(w.objects[0].material.diffuse, 0.7);
        assert_eq!(w.objects[0].material.specular, 0.2);
        assert_eq!(w.objects[1].transform, IDENTITY.scale(0.5, 0.5, 0.5));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs = w.intersect(&r);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_and_intersection() {
        let w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let object = &w.objects[0];
        let i = Intersection { t: 4.0, object };
        let comps = i.prepare_computations(&r, &[&i]);
        let c = w.shade_hit(&comps, RAY_LIMIT);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let c = w.color_at(&r, RAY_LIMIT);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let c = w.color_at(&r, RAY_LIMIT);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        let mut material = Material::default();
        material.ambient = 1.0;
        let material = Rc::new(material);
        w.objects[0].material = Rc::clone(&material);
        w.objects[1].material = Rc::clone(&material);
        let inner = &w.objects[1];
        let r = Ray {
            origin: point(0.0, 0.0, 0.75),
            direction: vector(0.0, 0.0, -1.0),
        };
        let c = w.color_at(&r, RAY_LIMIT);
        assert_eq!(c, inner.material.color);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = point(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&p));
    }

    #[test]
    fn no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = point(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = point(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.light_source = Some(PointLight {
            position: point(0.0, 0.0, -10.0),
            intensity: Color::new(1.0, 1.0, 1.0),
        });
        let o1 = Object::new(Box::new(Sphere::default()));
        let mut o2 = Object::new(Box::new(Sphere::default()));
        o2.transform = IDENTITY.translate(0.0, 0.0, 10.0);
        let r = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        w.objects = vec![o1, o2];
        let i = Intersection {
            t: 4.0,
            object: &w.objects[1],
        };
        let comps = i.prepare_computations(&r, &[&i]);
        let c = w.shade_hit(&comps, RAY_LIMIT);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_for_a_nonreflective_material() {
        let mut w = World::default();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let mut material = Material::default();
        material.ambient = 1.0;
        let material = Rc::new(material);
        w.objects[1].material = Rc::clone(&material);
        let i = Intersection::new(1.0, &w.objects[1]);
        let comps = i.prepare_computations(&r, &[&i]);
        assert_eq!(
            w.reflected_color(&comps, RAY_LIMIT),
            Color::new(0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn reflected_color_for_a_reflective_material() {
        let mut w = World::default();
        let mut plane = Object::new(Box::new(Plane::default()));
        let mut material = Material::default();
        material.reflective = 0.5;
        plane.material = Rc::new(material);
        plane.transform = IDENTITY.translate(0.0, -1.0, 0.0);
        w.objects.push(plane);
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0),
        );
        let i = Intersection::new((2.0 as f64).sqrt(), &w.objects[2]);
        let comps = i.prepare_computations(&r, &[&i]);
        assert_eq!(
            w.reflected_color(&comps, RAY_LIMIT),
            Color::new(0.19032, 0.2379, 0.14274)
        );
    }

    #[test]
    fn shade_hit_for_a_reflective_material() {
        let mut w = World::default();
        let mut plane = Object::new(Box::new(Plane::default()));
        let mut material = Material::default();
        material.reflective = 0.5;
        plane.material = Rc::new(material);
        plane.transform = IDENTITY.translate(0.0, -1.0, 0.0);
        w.objects.push(plane);
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0),
        );
        let i = Intersection::new((2.0 as f64).sqrt(), &w.objects[2]);
        let comps = i.prepare_computations(&r, &[&i]);
        assert_eq!(
            w.shade_hit(&comps, RAY_LIMIT),
            Color::new(0.87677, 0.92436, 0.82918)
        );
    }

    #[test]
    fn color_at_with_manually_reflective_surfaces() {
        let mut w = World::new();
        w.light_source = Some(PointLight::new(
            point(0.0, 0.0, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let material = {
            let mut m = Material::default();
            m.reflective = 0.5;
            Rc::new(m)
        };
        let mut lower = Object::new(Box::new(Plane::default()));
        lower.material = Rc::clone(&material);
        lower.transform = IDENTITY.translate(0.0, -1.0, 0.0);
        w.objects.push(lower);
        let mut upper = Object::new(Box::new(Plane::default()));
        upper.material = Rc::clone(&material);
        upper.transform = IDENTITY.translate(0.0, 1.0, 0.0);
        w.objects.push(upper);
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        w.color_at(&r, RAY_LIMIT);
    }

    #[test]
    fn reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        let mut plane = Object::new(Box::new(Plane::default()));
        let mut material = Material::default();
        material.reflective = 0.5;
        plane.material = Rc::new(material);
        plane.transform = IDENTITY.translate(0.0, -1.0, 0.0);
        w.objects.push(plane);
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0),
        );
        let i = Intersection::new((2.0 as f64).sqrt(), &w.objects[2]);
        let comps = i.prepare_computations(&r, &[&i]);
        assert_eq!(w.reflected_color(&comps, 0), Color::new(0.0, 0.0, 0.0));
    }
}
