// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn empty_ghost_block() {
    ghost! {}
}

fn return_disallowed() {
    ghost! {
        return; //~ ERROR:
    }
}

fn main() {}
