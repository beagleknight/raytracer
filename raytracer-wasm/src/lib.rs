mod utils;

use canvas::Canvas;
use colors::Color;
use core::f64::consts::PI;
use transformations::{view_transform, MatrixTransformations};
use tuples::{point, vector};
use wasm_bindgen::prelude::*;
use world::RAY_LIMIT;
use world::{camera::Camera, World};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn create_world() -> World {
    World::default()
}

#[wasm_bindgen]
pub fn create_camera(width: i32, height: i32) -> Camera {
    Camera::new(width, height, PI / 3.0)
}

#[wasm_bindgen]
pub fn color_at(world: &World, camera: &Camera, x: i32, y: i32) -> Color {
    let ray = camera.ray_for_pixel(x, y);
    world.color_at(&ray, RAY_LIMIT)
}
