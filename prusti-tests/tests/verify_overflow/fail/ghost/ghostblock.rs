// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn empty_ghost_block() {
    ghost! {}
}

fn return_disallowed() {
    //~ ERROR: ghost code might trigger non-ghost code
    ghost! {
        return;
    }
}

fn break_disallowed() {
    //~ ERROR: ghost code might trigger non-ghost code
    while true {
        ghost! {
            break;
        }
    }
}

fn continue_disallowed() {
    //~ ERROR: ghost code might trigger non-ghost code
    while true {
        ghost! {
            continue;
        }
    }
}

fn main() {}
