// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn main() {}

fn foo() {
    'outer: loop {
        ghost! {
            loop {
                break 'outer;
            }
        }
    }
}
