#![allow(dead_code)]
#![allow(unused_variables)]

/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for item in slice {
        sum = sum + item;
    }
    sum
}

/// Deduplicate items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, in order.
pub fn dedup(vector: &Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    for &item in vector {
        if !out.contains(&item) {
            out.push(item);
        }
    }
    out
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut out = vec![];
    for &item in vs {
        if pred(item) {
            out.push(item);
        }
    }
    out
}

