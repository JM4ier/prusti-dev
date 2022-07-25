// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

#[terminates]
fn main() {
    while false {
        body_invariant!(false);
        body_variant!(Int::new(0));
    }
}

fn ghost_terminates() {
    ghost! {
        while false {
            body_invariant!(false);
            body_variant!(Int::new(0));
        }
    };
}

fn ghost_nontermination_error() {
    ghost! {
        loop {} //~ ERROR
    };
}

#[pure]
fn pure_fn() -> u32 {
    42
}

fn allows_pure_calls() {
    ghost! {
        let x = pure_fn();
    };
}