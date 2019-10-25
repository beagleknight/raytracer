use canvas::canvas_to_ppm;
use colors::Color;
use core::f64::consts::PI;
use lights::PointLight;
use matrices::IDENTITY;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use transformations::{view_transform, MatrixTransformations};
use tuples::{point, vector};
use world::{
    camera::Camera, materials::Material, object::Object, patterns::stripes::StripesPatternShape,
    patterns::Pattern, planes::Plane, spheres::Sphere, World,
};

fn main() -> std::io::Result<()> {
    let canvas_width = 600;
    let canvas_height = 300;
    let mut floor = Object::new(Box::new(Plane::default()));
    let mut floor_material = Material::default();
    floor_material.color = Color::new(1.0, 0.9, 0.9);
    floor_material.specular = 0.0;
    floor_material.pattern = Some(Pattern::new(Box::new(StripesPatternShape {
        a: Color::new(1.0, 1.0, 1.0),
        b: Color::new(0.7, 0.7, 0.7),
    })));
    floor.transform = IDENTITY.scale(10.0, 1.0, 10.0);
    floor.material = Rc::new(floor_material);
    let mut middle = Object::new(Box::new(Sphere::default()));
    let mut middle_material = Material::default();
    middle_material.color = Color::new(0.1, 1.0, 0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle.transform = IDENTITY.translate(-0.5, 1.0, 0.5);
    middle.material = Rc::new(middle_material);
    let mut right = Object::new(Box::new(Sphere::default()));
    let mut right_material = Material::default();
    right_material.color = Color::new(0.5, 1.0, 0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    right.transform = IDENTITY.scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5);
    right.material = Rc::new(right_material);
    let mut left = Object::new(Box::new(Sphere::default()));
    let mut left_material = Material::default();
    left_material.color = Color::new(1.0, 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left.transform = IDENTITY
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75);
    left.material = Rc::new(left_material);
    let mut world = World::new();
    world.light_source = Some(PointLight {
        position: point(-10.0, 10.0, -10.0),
        intensity: Color::new(1.0, 1.0, 1.0),
    });
    world.objects = vec![floor, middle, right, left];

    let mut camera = Camera::new(canvas_width, canvas_height, PI / 3.0);
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
