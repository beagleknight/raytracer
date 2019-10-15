use float_cmp::ApproxEq;
use tuples::{tuple, Tuple};

pub const IDENTITY: [[f64; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub fn matrix_multiply(a: &[[f64; 4]; 4], b: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut m = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            m[row][col] = a[row][0] * b[0][col]
                + a[row][1] * b[1][col]
                + a[row][2] * b[2][col]
                + a[row][3] * b[3][col]
        }
    }
    m
}

pub fn matrix_tuple_multiply(a: &[[f64; 4]; 4], b: &Tuple) -> Tuple {
    let mut result = [0.0; 4];
    let b = [b.x, b.y, b.z, b.w];
    for row in 0..4 {
        result[row] = a[row][0] * b[0] + a[row][1] * b[1] + a[row][2] * b[2] + a[row][3] * b[3];
    }
    result.into()
}

fn transpose(a: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut m = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            m[row][col] = a[col][row];
        }
    }
    m
}

fn determinant_2(a: &[[f64; 2]; 2]) -> f64 {
    a[0][0] * a[1][1] - a[0][1] * a[1][0]
}

fn determinant_3(a: &[[f64; 3]; 3]) -> f64 {
    let mut det = 0.0;

    for col in 0..3 {
        det = det + a[0][col] * cofactor_3(a, 0, col);
    }

    det
}

fn determinant_4(a: &[[f64; 4]; 4]) -> f64 {
    let mut det = 0.0;

    for col in 0..4 {
        det = det + a[0][col] * cofactor_4(a, 0, col);
    }

    det
}

fn submatrix_3(a: &[[f64; 3]; 3], ignore_row: usize, ignore_col: usize) -> [[f64; 2]; 2] {
    let mut result = [[0.0; 2]; 2];
    let mut sub = vec![];

    for row in 0..3 {
        let mut row_vec = vec![];

        if row == ignore_row {
            continue;
        }

        for col in 0..3 {
            if col == ignore_col {
                continue;
            }
            row_vec.push(a[row][col]);
        }

        sub.push(row_vec);
    }

    for row in 0..2 {
        for col in 0..2 {
            result[row][col] = sub[row][col];
        }
    }

    result
}

fn submatrix_4(a: &[[f64; 4]; 4], ignore_row: usize, ignore_col: usize) -> [[f64; 3]; 3] {
    let mut result = [[0.0; 3]; 3];
    let mut sub = vec![];

    for row in 0..4 {
        let mut row_vec = vec![];

        if row == ignore_row {
            continue;
        }

        for col in 0..4 {
            if col == ignore_col {
                continue;
            }
            row_vec.push(a[row][col]);
        }

        sub.push(row_vec);
    }

    for row in 0..3 {
        for col in 0..3 {
            result[row][col] = sub[row][col];
        }
    }

    result
}

fn minor_3(a: &[[f64; 3]; 3], row: usize, col: usize) -> f64 {
    determinant_2(&submatrix_3(a, row, col))
}

fn minor_4(a: &[[f64; 4]; 4], row: usize, col: usize) -> f64 {
    determinant_3(&submatrix_4(a, row, col))
}

fn cofactor_3(a: &[[f64; 3]; 3], row: usize, col: usize) -> f64 {
    let minor = minor_3(a, row, col);
    if (row + col) % 2 != 0 {
        return -minor;
    }
    minor
}

fn cofactor_4(a: &[[f64; 4]; 4], row: usize, col: usize) -> f64 {
    let minor = minor_4(a, row, col);
    if (row + col) % 2 != 0 {
        return -minor;
    }
    minor
}

fn is_invertible(a: &[[f64; 4]; 4]) -> bool {
    determinant_4(a) != 0.0
}

pub fn inverse(a: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut cofactors = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            cofactors[row][col] = cofactor_4(&a, row, col);
        }
    }
    let mut result = transpose(&cofactors);
    let determinant = determinant_4(&a);
    for row in 0..4 {
        for col in 0..4 {
            result[row][col] = result[row][col] / determinant;
        }
    }
    result
}

fn approx_eq(a: &[[f64; 4]; 4], b: &[[f64; 4]; 4]) -> bool {
    for row in 0..4 {
        for col in 0..4 {
            if !a[row][col].approx_eq(b[row][col], (0.00001, 2)) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn can_represent_2x2_matrix() {
        let m = [[-3.0, 5.0], [1.0, -2.0]];
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn can_represent_3x3_matrix() {
        let m = [[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]];
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let b = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let b = [
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ];
        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let b = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ];
        assert_eq!(
            matrix_multiply(&a, &b),
            [
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]
            ]
        )
    }

    #[test]
    fn matrix_multiplied_by_a_tuple() {
        let a = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let b = tuple(1.0, 2.0, 3.0, 1.0);
        assert_eq!(matrix_tuple_multiply(&a, &b), tuple(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn multipliying_a_matrix_by_the_identity_matrix() {
        let a = [
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ];
        assert_eq!(matrix_multiply(&a, &IDENTITY), a);
    }

    #[test]
    fn multipliying_a_matrix_by_a_tuple() {
        let a = tuple(1.0, 2.0, 3.0, 4.0);
        assert_eq!(matrix_tuple_multiply(&IDENTITY, &a), a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = [
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ];
        assert_eq!(
            transpose(&a),
            [
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]
            ]
        )
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let a = transpose(&IDENTITY);
        assert_eq!(a, IDENTITY);
    }

    #[test]
    fn calculating_the_determinant_of_2x2_matrix() {
        let a = [[1.0, 5.0], [-3.0, 2.0]];
        assert_eq!(determinant_2(&a), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let a = [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]];
        assert_eq!(submatrix_3(&a, 0, 2), [[-3.0, 2.0], [0.0, 6.0]]);
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let a = [
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ];
        assert_eq!(
            submatrix_4(&a, 2, 1),
            [[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]
        );
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]];
        let b = submatrix_3(&a, 1, 0);
        assert_eq!(determinant_2(&b), minor_3(&a, 1, 0));
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]];
        assert_eq!(minor_3(&a, 0, 0), cofactor_3(&a, 0, 0));
        assert_eq!(minor_3(&a, 1, 0), -cofactor_3(&a, 1, 0));
    }

    #[test]
    fn calculating_the_determinant_of_3x3_matrix() {
        let a = [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]];
        assert_eq!(cofactor_3(&a, 0, 0), 56.0);
        assert_eq!(cofactor_3(&a, 0, 1), 12.0);
        assert_eq!(cofactor_3(&a, 0, 2), -46.0);
        assert_eq!(determinant_3(&a), -196.0);
    }

    #[test]
    fn calculating_the_determinant_of_4x4_matrix() {
        let a = [
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ];
        assert_eq!(cofactor_4(&a, 0, 0), 690.0);
        assert_eq!(cofactor_4(&a, 0, 1), 447.0);
        assert_eq!(cofactor_4(&a, 0, 2), 210.0);
        assert_eq!(cofactor_4(&a, 0, 3), 51.0);
        assert_eq!(determinant_4(&a), -4071.0);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = [
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ];
        assert_eq!(determinant_4(&a), -2120.0);
        assert!(is_invertible(&a));
    }

    #[test]
    fn testing_a_non_invertible_matrix_for_invertibility() {
        let a = [
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ];
        assert_eq!(determinant_4(&a), 0.0);
        assert!(!is_invertible(&a));
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = [
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ];
        let b = inverse(&a);
        assert_eq!(determinant_4(&a), 532.0);
        assert_eq!(cofactor_4(&a, 2, 3), -160.0);
        assert_eq!(b[3][2], -160.0 / 532.0);
        assert_eq!(cofactor_4(&a, 3, 2), 105.0);
        assert_eq!(b[2][3], 105.0 / 532.0);
        assert!(approx_eq(
            &b,
            &[
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639]
            ]
        ));
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = [
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ];
        let b = [
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ];
        let c = matrix_multiply(&a, &b);
        let inv_b = inverse(&b);
        assert!(approx_eq(&matrix_multiply(&c, &inv_b), &a))
    }
}
