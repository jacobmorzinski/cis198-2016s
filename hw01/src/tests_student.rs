#![cfg(test)]

use problem1::{sum};
use problem2::{mat_mult};
use problem3::{sieve};
use problem4::{hanoi,Peg};

//
// Problem 1
//

#[test]
fn test_sum() {
    let array = [1,1,2,3,5,8];
    assert_eq!(sum(&array), 20);
}

//
// Problem 2
//

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

//
// Problem 3
//

#[test]
fn test_sieve() {
    assert_eq!(vec![2,3,5,7,11,13,17,19,21], sieve(22));
}

//
// Problem 4
//

#[test]
fn test_hanoi_2_disks() {
    let result = hanoi(2, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![(Peg::A, Peg::B),
                    (Peg::A, Peg::C),
                    (Peg::B, Peg::C)], result);
    assert_eq!(3, result.len());
}
