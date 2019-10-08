use float_cmp::ApproxEq;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(other.x, (std::f64::EPSILON, 2))
            && self.y.approx_eq(other.y, (std::f64::EPSILON, 2))
            && self.z.approx_eq(other.z, (std::f64::EPSILON, 2))
            && self.w.approx_eq(other.w, (std::f64::EPSILON, 2))
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

pub fn magnitude(v: Tuple) -> f64 {
    (v.x.powi(2) + v.y.powi(2) + v.z.powi(2) + v.w.powi(2)).sqrt()
}

pub fn normalize(v: Tuple) -> Tuple {
    let m = magnitude(v);
    Tuple {
        x: v.x / m,
        y: v.y / m,
        z: v.z / m,
        w: v.w / m,
    }
}

pub fn dot(v1: Tuple, v2: Tuple) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w
}

pub fn cross(v1: Tuple, v2: Tuple) -> Tuple {
    vector(
        v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x,
    )
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tuple_with_w_equal_to_1_is_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_equal_to_0_is_a_vector() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w_equal_to_1() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p, tuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuples_with_w_equal_to_0() {
        let v = vector(4.0, -4.0, 3.0);
        assert_eq!(v, tuple(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = tuple(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, tuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn substracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn substracting_a_vector_from_a_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn substracting_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, tuple(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, tuple(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computing_the_magnitude_of_some_vectors() {
        let v1 = vector(1.0, 0.0, 0.0);
        let v2 = vector(0.0, 1.0, 0.0);
        let v3 = vector(0.0, 0.0, 1.0);
        let v4 = vector(1.0, 2.0, 3.0);
        let v5 = vector(-1.0, -2.0, -3.0);
        assert_eq!(magnitude(v1), 1.0);
        assert_eq!(magnitude(v2), 1.0);
        assert_eq!(magnitude(v3), 1.0);
        assert_eq!(magnitude(v4), (14.0 as f64).sqrt());
        assert_eq!(magnitude(v5), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalizing_some_vectors() {
        let v1 = vector(4.0, 0.0, 0.0);
        let v2 = vector(1.0, 2.0, 3.0);
        assert_eq!(normalize(v1), vector(1.0, 0.0, 0.0));
        assert_eq!(
            normalize(v2),
            vector(
                1.0 / (14.0 as f64).sqrt(),
                2.0 / (14.0 as f64).sqrt(),
                3.0 / (14.0 as f64).sqrt()
            )
        );
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = vector(1.0, 2.0, 3.0);
        let norm = normalize(v);
        assert_eq!(magnitude(norm), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(dot(a, b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(cross(a, b), vector(-1.0, 2.0, -1.0));
        assert_eq!(cross(b, a), vector(1.0, -2.0, 1.0));
    }
}
