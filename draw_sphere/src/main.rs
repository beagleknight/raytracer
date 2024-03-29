use canvas::{canvas, canvas_to_ppm, write_pixel};
use colors::Color;
use lights::PointLight;
use rays::Ray;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use tuples::{normalize, point};
use world::{materials::Material, object::Object, shapes::{spheres::Sphere}};

fn main() -> std::io::Result<()> {
    let canvas_size = 500;
    let light = PointLight {
        position: point(-10.0, 10.0, -10.0),
        intensity: Color::new(1.0, 1.0, 1.0),
    };
    let mut c = canvas(canvas_size, canvas_size);
    let mut s = Object::new(Box::new(Sphere::default()));
    let mut material = Material::default();
    material.color = Color::new(0.443, 0.502, 0.725);
    s.material = Rc::new(material);
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_size as f64;
    let half = wall_size / 2.0;

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_size {
            let world_x = -half + pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);
            let r = Ray {
                origin: ray_origin,
                direction: normalize(&(position - ray_origin)),
            };

            if let Some(intersection) = s.intersect(&r) {
                let point = r.position(intersection[0].t);
                let normal = intersection[0].object.normal_at(&point);
                let eye = -r.direction;
                let color = intersection[0]
                    .object
                    .material
                    .lightning(&s, &light, &point, &eye, &normal, false);
                write_pixel(&mut c, x as usize, y as usize, color);
            }
        }
    }

    let mut file = File::create("sphere.ppm")?;
    file.write_all(canvas_to_ppm(&c).as_bytes())?;
    Ok(())
}
