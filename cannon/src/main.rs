use std::fs::File;
use std::io::prelude::*;

use canvas::{canvas, canvas_to_ppm, write_pixel};
use colors::color;
use tuples::{normalize, point, vector, Tuple};

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(environment: &Environment, projectile: &mut Projectile) {
    projectile.position = projectile.position + projectile.velocity;
    projectile.velocity = projectile.velocity + environment.gravity + environment.wind;
}

fn print_projectile_position(projectile: &Projectile) {
    println!(
        "Projectile position is ({}, {}, {})",
        projectile.position.x, projectile.position.y, projectile.position.z
    );
}

fn print_projectile_velocity(projectile: &Projectile) {
    println!(
        "Projectile velocity is ({}, {}, {})",
        projectile.velocity.x, projectile.velocity.y, projectile.velocity.z
    );
}

fn main() -> std::io::Result<()> {
    let mut ticks = 0;
    let start = point(0.0, 1.0, 0.0);
    let velocity = normalize(vector(1.0, 1.8, 0.0)) * 11.25;
    let gravity = vector(0.0, -0.1, 0.0);
    let wind = vector(-0.01, 0.0, 0.0);
    let environment = Environment { gravity, wind };
    let mut projectile = Projectile {
        position: start,
        velocity,
    };
    let canvas_width = 900;
    let canvas_height = 550;
    let mut c = canvas(canvas_width, canvas_height);

    loop {
        tick(&environment, &mut projectile);
        ticks += 1;
        print_projectile_position(&projectile);
        print_projectile_velocity(&projectile);
        write_pixel(
            &mut c,
            projectile.position.x as usize,
            (canvas_height - projectile.position.y as i32) as usize,
            color(0.5, 0.0, 0.5),
        );
        if projectile.position.y <= 0.0 {
            println!("Crashed at ticks equal to {}", ticks);
            break;
        }
    }

    let mut file = File::create("rocket.ppm")?;
    file.write_all(canvas_to_ppm(&c).as_bytes())?;
    Ok(())
}
