// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn assert1() {
    prusti_assert!(true);
}

//fn failing_assert() {
//    prusti_assert!(false); //~ ERROR
//    body_invariant!(false);
//}
//
//fn loop_shouldnt_crash() {
//    loop {
//        prusti_assert!(true);
//        body_invariant!(true);
//    }
//}

fn main() {}
