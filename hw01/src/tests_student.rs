#![cfg(test)]

use problem1::{sum};
use problem2::{mat_mult};

#[test]
fn test_sum() {
    let array = [1,1,2,3,5,8];
    assert_eq!(sum(&array), 20);
}

#[test]
fn test_matmul() {
    let mat1 = vec![vec![1.;3]; 3];
    let mat2 = vec![vec![1.;3]; 3];
    let result = mat_mult(&mat1, &mat2);
    let desired = vec![vec![3.;3]; 3];
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            assert_eq!(result[i][j], desired[i][j]);
        }
    }
}
