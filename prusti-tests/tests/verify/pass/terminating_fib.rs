// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn main() {}

#[pure(Int::new(x))]
fn fib(x: i64) -> i64 {
    if x <= 1 {
        x
    } else {
        fib(x - 1) + fib(x - 2)
    }
}
