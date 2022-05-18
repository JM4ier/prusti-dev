#![allow(unused)]

use prusti_contracts::*;

fn assert1() {
    prusti_assert!(true);
}

fn failing_assert() {
    prusti_assert!(false); //~ ERROR
    body_invariant!(false);
}

fn main() {}