use std::fs::File;
use std::io::prelude::*;

use canvas::{canvas, canvas_to_ppm, write_pixel};
use colors::color;
use matrices::IDENTITY;
use rays::Ray;
use spheres::Sphere;
use transformations::MatrixTransformations;
use tuples::{normalize, point};

fn main() -> std::io::Result<()> {
    let canvas_size = 500;
    let mut c = canvas(canvas_size, canvas_size);
    let mut s = Sphere::new();
    s.transform(&IDENTITY.scale(0.5, 0.25, 1.0));
    let color = color(1.0, 0.0, 0.0);
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
            let xs = s.intersect(&r);

            if xs.is_some() {
                write_pixel(&mut c, x as usize, (canvas_size - y as i32) as usize, color);
            }
        }
    }

    let mut file = File::create("sphere.ppm")?;
    file.write_all(canvas_to_ppm(&c).as_bytes())?;
    Ok(())
}
