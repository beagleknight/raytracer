use canvas::{canvas, canvas_to_ppm, write_pixel};
use colors::Color;
use core::f64::consts::PI;
use matrices::{matrix_tuple_multiply, IDENTITY};
use std::fs::File;
use std::io::prelude::*;
use transformations::MatrixTransformations;
use tuples::point;

fn main() -> std::io::Result<()> {
    let canvas_size = 500;
    let clock_length = canvas_size as f64 * 0.40;
    let angle = (2.0 * PI) / 12.0;
    let mut c = canvas(canvas_size, canvas_size);

    let center = point((canvas_size as f64) / 2.0, (canvas_size as f64) / 2.0, 0.0);

    for i in 0..12 {
        let point = center
            + matrix_tuple_multiply(
                &IDENTITY.rotate_z(angle * (i as f64)),
                &point(0.0, 1.0, 0.0),
            ) * clock_length;

        write_pixel(
            &mut c,
            point.x as usize,
            (canvas_size - point.y as i32) as usize,
            Color::new(1.0, 1.0, 1.0),
        );
    }

    let mut file = File::create("clock.ppm")?;
    file.write_all(canvas_to_ppm(&c).as_bytes())?;
    Ok(())
}
