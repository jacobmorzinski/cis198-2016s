#![allow(dead_code)]
#![allow(unused_variables)]

/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let m = n as usize;
    let mut sieve = vec![1; m];
    let mut out: Vec<u32> = Vec::new();
    sieve[0] = 0;
    sieve[1] = 0;
    for i in 2..m {
        if sieve[i] == 1 {
            out.push(i as u32);
            for j in i..m/i {
                sieve[i*j] = 0;
            }
        }
    }
    out
}

