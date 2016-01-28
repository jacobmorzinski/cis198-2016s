#![allow(dead_code)]
#![allow(unused_variables)]

/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let r1 = mat1.len();
    let c1 = mat1[0].len();
    let r2 = mat2.len();
    let c2 = mat2[0].len();
    assert_eq!(c1,r2);
    let mut out: Matrix = vec![ vec![0.0;c2]; r1];
    for ro in 0..r1 {
        for co in 0..c2 {
            // out[ro][co] = Sum(i, mat1[ro][i] * mat2[i][co])
            for i in 0..c1 {
                out[ro][co] += mat1[ro][i] * mat2[i][co];
            }
        }
    }
    out
}

