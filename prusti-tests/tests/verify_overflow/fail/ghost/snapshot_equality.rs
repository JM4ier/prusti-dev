// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn main() {
    prusti_assert!(1 === 1);
}

fn test1() {
    prusti_assert!(1 === 2); //~ ERROR:
}