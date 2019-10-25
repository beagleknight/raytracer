use crate::World;
use canvas::Canvas;
use matrices::{inverse, matrix_tuple_multiply, IDENTITY};
use rays::Ray;
use tuples::{normalize, point};

pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub half_width: f64,
    pub half_height: f64,
    pub field_of_view: f64,
    pub transform: [[f64; 4]; 4],
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let mut half_width = half_view * aspect;
        let mut half_height = half_view;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        }

        Camera {
            hsize,
            vsize,
            half_width,
            half_height,
            field_of_view,
            transform: IDENTITY,
            pixel_size: (half_width * 2.0) / hsize as f64,
        }
    }

    pub fn ray_for_pixel(&self, px: i32, py: i32) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inv_camera_transform = inverse(&self.transform);

        let pixel = matrix_tuple_multiply(&inv_camera_transform, &point(world_x, world_y, -1.0));
        let origin = matrix_tuple_multiply(&inv_camera_transform, &point(0.0, 0.0, 0.0));
        let direction = normalize(&(pixel - origin));

        Ray { origin, direction }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x as usize, y as usize, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use crate::{camera::Camera, World};
    use colors::color;
    use core::f64::consts::PI;
    use float_cmp::ApproxEq;
    use matrices::IDENTITY;
    use transformations::{view_transform, MatrixTransformations};
    use tuples::{point, vector};

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c.field_of_view, field_of_view);
        assert_eq!(c.transform, IDENTITY);
    }

    #[test]
    fn pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(c.pixel_size.approx_eq(0.01, (0.00001, 2)));
    }

    #[test]
    fn pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(c.pixel_size.approx_eq(0.01, (0.00001, 2)));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 00);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = IDENTITY.translate(0.0, -2.0, 5.0).rotate_y(PI / 4.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            vector((2.0 as f64).sqrt() / 2.0, 0.0, -(2.0 as f64).sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.transform = view_transform(&from, &to, &up);
        let image = c.render(&w);
        assert_eq!(
            *image.pixel_at(5, 5).unwrap(),
            color(0.38066, 0.47583, 0.2855)
        );
    }
}
