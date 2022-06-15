// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn empty_ghost_block() {
    ghost! {}
}

fn return_disallowed() {
    ghost! {
        // FIXME: currently this causes a panic, need to figure out how to make this a normal error
        return; //~ ERROR:
    }
}

fn main() {}
