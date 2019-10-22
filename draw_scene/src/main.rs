use core::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use camera::Camera;
use canvas::canvas_to_ppm;
use colors::color;
use lights::PointLight;
use materials::Material;
use matrices::IDENTITY;
use spheres::Sphere;
use transformations::{view_transform, MatrixTransformations};
use tuples::{point, vector};
use world::World;

fn main() -> std::io::Result<()> {
    let mut floor = Sphere::new();
    floor.transform = IDENTITY.scale(10.0, 0.01, 10.0);
    floor.material = Material::default();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    let mut left_wall = Sphere::new();
    left_wall.transform = IDENTITY
        .scale(10.0, 0.01, 10.0)
        .rotate_x(PI / 2.0)
        .rotate_y(-PI / 4.0)
        .translate(0.0, 0.0, 5.0);
    left_wall.material = floor.material;
    let mut right_wall = Sphere::new();
    right_wall.transform = IDENTITY
        .scale(10.0, 0.01, 10.0)
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0)
        .translate(0.0, 0.0, 5.0);
    right_wall.material = floor.material;
    let mut middle = Sphere::new();
    middle.transform = IDENTITY.translate(-0.5, 1.0, 0.5);
    middle.material = Material::default();
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    let mut right = Sphere::new();
    right.transform = IDENTITY.scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5);
    right.material = Material::default();
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    let mut left = Sphere::new();
    left.transform = IDENTITY
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75);
    left.material = Material::default();
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.light_source = Some(PointLight {
        position: point(-10.0, 10.0, -10.0),
        intensity: color(1.0, 1.0, 1.0),
    });
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];

    let mut camera = Camera::new(600, 300, PI / 3.0);
    camera.transform = view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let mut file = File::create("draw_scene.ppm")?;
    file.write_all(canvas_to_ppm(&canvas).as_bytes())?;
    Ok(())
}
