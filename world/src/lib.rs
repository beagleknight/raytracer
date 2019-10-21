use colors::{color, Color};
use intersections::{hit, Computations, Intersection};
use lights::PointLight;
use matrices::IDENTITY;
use rays::Ray;
use spheres::Sphere;
use transformations::MatrixTransformations;
use tuples::point;

pub struct World {
    pub light_source: Option<PointLight>,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        World {
            light_source: None,
            objects: vec![],
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection<Sphere>> {
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

    pub fn shade_hit(&self, comps: &Computations<Sphere>) -> Color {
        comps.object.material.lightning(
            &self.light_source.as_ref().unwrap(),
            &comps.point,
            &comps.eyev,
            &comps.normalv,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);

        match hit(&intersections) {
            Some(intersection) => {
                let comps = intersection.prepare_computations(ray);
                self.shade_hit(&comps)
            }
            None => color(0.0, 0.0, 0.0),
        }
    }
}

impl Default for World {
    fn default() -> World {
        let light = PointLight {
            position: point(-10.0, 10.0, -10.0),
            intensity: color(1.0, 1.0, 1.0),
        };
        let mut s1 = Sphere::new();
        s1.material.color = color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.transform = IDENTITY.scale(0.5, 0.5, 0.5);
        World {
            light_source: Some(light),
            objects: vec![s1, s2],
        }
    }
}

#[cfg(test)]
mod tests {
    use colors::color;
    use lights::PointLight;
    use matrices::IDENTITY;
    use rays::Ray;
    use spheres::Sphere;
    use transformations::MatrixTransformations;
    use tuples::{point, vector};

    use crate::*;

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
            intensity: color(1.0, 1.0, 1.0),
        };
        let mut s1 = Sphere::new();
        s1.material.color = color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.transform = IDENTITY.scale(0.5, 0.5, 0.5);
        assert_eq!(w.light_source, Some(light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
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
        let shape = &w.objects[0];
        let i = Intersection {
            t: 4.0,
            object: shape,
        };
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let c = w.color_at(&r);
        assert_eq!(c, color(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let c = w.color_at(&r);
        assert_eq!(c, color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let inner = &w.objects[1];
        let r = Ray {
            origin: point(0.0, 0.0, 0.75),
            direction: vector(0.0, 0.0, -1.0),
        };
        let c = w.color_at(&r);
        assert_eq!(c, inner.material.color);
    }
}