// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn main() {}

#[terminates(Int::new(x))]
fn y1(x: i64) {
    if x > 0 {
        y2(x - 1)
    }
}

#[terminates(Int::new(x))]
fn y2(x: i64) {
    if x > 0 {
        y1(x - 1)
    }
    leaf()
}

#[terminates]
fn leaf() {}

#[terminates]
fn nonterminating() {
    nonterminating() //~ ERROR
}
