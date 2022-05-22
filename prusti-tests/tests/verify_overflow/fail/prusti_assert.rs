// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn assert1() {
    prusti_assert!(true);
}

fn failing_assert() {
    // FIXME prusti reports this as an internal error
    prusti_assert!(false); //~ ERROR
}

//fn loop_shouldnt_crash() {
//    // FIXME loop actually crashes
//    loop {
//        //prusti_assert!(true);
//        //body_invariant!(true);
//    }
//}

fn main() {}
