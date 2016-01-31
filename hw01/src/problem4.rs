#![allow(unused_variables)]
#![allow(dead_code)]

/// #[derive(...)] statements define certain properties on the enum
/// for you for free (printing, equality testing, the ability to copy
/// values). More on this when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing
/// `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from
/// `src` to `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    // Recursive solution:
    // 1. move n−1 discs from A(src) to B(aux).
    // 2. move disc n from A(src) to C(dst)
    // 3. move n−1 discs from B(aux) to C(dst) so they sit on disc n
    
    let mut result: Vec<Move> = Vec::new();
    let thismove = (src,dst);

    // 1.
    if num_discs>1 {
        let mut m1 = hanoi(num_discs-1, src, dst, aux).clone();
        result.append(&mut m1);
    }

    // 2.
    result.push( thismove );
    //println!("This move is {:?} {:?}", thismove.0, thismove.1);

    // 3.
    if num_discs>1 {
        let mut m2 = hanoi(num_discs-1, aux, src, dst).clone();
        result.append(&mut m2);
    }
    result
}

