use matrices::{matrix_multiply, IDENTITY};
use tuples::{cross, normalize, Tuple};

pub trait MatrixTransformations {
    fn translate(self, x: f64, y: f64, z: f64) -> Self;
    fn scale(self, x: f64, y: f64, z: f64) -> Self;
    fn rotate_x(self, r: f64) -> Self;
    fn rotate_y(self, r: f64) -> Self;
    fn rotate_z(self, r: f64) -> Self;
    fn skew(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self;
}

impl MatrixTransformations for [[f64; 4]; 4] {
    fn translate(self, x: f64, y: f64, z: f64) -> Self {
        let mut result = IDENTITY;
        result[0][3] = x;
        result[1][3] = y;
        result[2][3] = z;
        matrix_multiply(&result, &self)
    }

    fn scale(self, x: f64, y: f64, z: f64) -> Self {
        let mut result = IDENTITY;
        result[0][0] = x;
        result[1][1] = y;
        result[2][2] = z;
        matrix_multiply(&result, &self)
    }

    fn rotate_x(self, r: f64) -> Self {
        let mut result = IDENTITY;
        result[1][1] = r.cos();
        result[1][2] = -r.sin();
        result[2][1] = r.sin();
        result[2][2] = r.cos();
        matrix_multiply(&result, &self)
    }

    fn rotate_y(self, r: f64) -> Self {
        let mut result = IDENTITY;
        result[0][0] = r.cos();
        result[0][2] = r.sin();
        result[2][0] = -r.sin();
        result[2][2] = r.cos();
        matrix_multiply(&result, &self)
    }

    fn rotate_z(self, r: f64) -> Self {
        let mut result = IDENTITY;
        result[0][0] = r.cos();
        result[0][1] = -r.sin();
        result[1][0] = r.sin();
        result[1][1] = r.cos();
        matrix_multiply(&result, &self)
    }

    fn skew(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut result = IDENTITY;
        result[0][1] = xy;
        result[0][2] = xz;
        result[1][0] = yx;
        result[1][2] = yz;
        result[2][0] = zx;
        result[2][1] = zy;
        matrix_multiply(&result, &self)
    }
}

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> [[f64; 4]; 4] {
    let forward = normalize(&(*to - *from));
    let upn = normalize(up);
    let left = cross(&forward, &upn);
    let true_up = cross(&left, &forward);
    let orientation = [
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    matrix_multiply(&orientation, &IDENTITY.translate(-from.x, -from.y, -from.z))
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;
    use matrices::{approx_eq, inverse, matrix_tuple_multiply, IDENTITY};
    use tuples::{point, vector};

    use crate::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = IDENTITY.translate(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(matrix_tuple_multiply(&transform, &p), point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_translation_matrix() {
        let transform = IDENTITY.translate(5.0, -3.0, 2.0);
        let inv = inverse(&transform);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(matrix_tuple_multiply(&inv, &p), point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = IDENTITY.translate(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(matrix_tuple_multiply(&transform, &v), v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let transform = IDENTITY.scale(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(
            matrix_tuple_multiply(&transform, &p),
            point(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let transform = IDENTITY.scale(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(
            matrix_tuple_multiply(&transform, &v),
            vector(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn multiplying_by_the_inverse_of_scaling_matrix() {
        let transform = IDENTITY.scale(2.0, 3.0, 4.0);
        let inv = inverse(&transform);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(matrix_tuple_multiply(&inv, &v), vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = IDENTITY.scale(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(matrix_tuple_multiply(&transform, &p), point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = IDENTITY.rotate_x(PI / 4.0);
        let full_quarter = IDENTITY.rotate_x(PI / 2.0);
        assert_eq!(
            matrix_tuple_multiply(&half_quarter, &p),
            point(0.0, (2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0)
        );
        assert_eq!(
            matrix_tuple_multiply(&full_quarter, &p),
            point(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = IDENTITY.rotate_x(PI / 4.0);
        let inv = inverse(&half_quarter);
        assert_eq!(
            matrix_tuple_multiply(&inv, &p),
            point(0.0, (2.0 as f64).sqrt() / 2.0, -((2.0 as f64).sqrt() / 2.0))
        )
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = IDENTITY.rotate_y(PI / 4.0);
        let full_quarter = IDENTITY.rotate_y(PI / 2.0);
        assert_eq!(
            matrix_tuple_multiply(&half_quarter, &p),
            point((2.0 as f64).sqrt() / 2.0, 0.0, (2.0 as f64).sqrt() / 2.0)
        );
        assert_eq!(
            matrix_tuple_multiply(&full_quarter, &p),
            point(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = IDENTITY.rotate_z(PI / 4.0);
        let full_quarter = IDENTITY.rotate_z(PI / 2.0);
        assert_eq!(
            matrix_tuple_multiply(&half_quarter, &p),
            point(-(2.0 as f64).sqrt() / 2.0, (2.0 as f64).sqrt() / 2.0, 0.0)
        );
        assert_eq!(
            matrix_tuple_multiply(&full_quarter, &p),
            point(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = IDENTITY.skew(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(matrix_tuple_multiply(&transform, &p), point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = IDENTITY.skew(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(matrix_tuple_multiply(&transform, &p), point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = IDENTITY.skew(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(matrix_tuple_multiply(&transform, &p), point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = IDENTITY.skew(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(matrix_tuple_multiply(&transform, &p), point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = IDENTITY.skew(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(matrix_tuple_multiply(&transform, &p), point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = IDENTITY.skew(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(matrix_tuple_multiply(&transform, &p), point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let a = IDENTITY.rotate_x(PI / 2.0);
        let b = IDENTITY.scale(5.0, 5.0, 5.0);
        let c = IDENTITY.translate(10.0, 5.0, 7.0);
        let p2 = matrix_tuple_multiply(&a, &p);
        let p3 = matrix_tuple_multiply(&b, &p2);
        let p4 = matrix_tuple_multiply(&c, &p3);
        assert_eq!(p2, point(1.0, -1.0, 0.0));
        assert_eq!(p3, point(5.0, -5.0, 0.0));
        assert_eq!(p4, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = point(1.0, 0.0, 1.0);
        let t = IDENTITY
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        assert_eq!(matrix_tuple_multiply(&t, &p), point(15.0, 0.0, 7.0));
    }

    #[test]
    fn view_transformation_matrix_for_the_default_orientation() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, IDENTITY);
    }

    #[test]
    fn view_transformation_matrix_looking_in_positive_z_direction() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, IDENTITY.scale(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_matrix_moves_the_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, IDENTITY.translate(0.0, 0.0, -8.0));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert!(approx_eq(
            &t,
            &[
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000]
            ]
        ));
    }
}
