// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn empty_ghost_block() {
    ghost! {};
}

fn return_disallowed() {
    //~ ERROR:
    ghost! {
        return;
    };
}

//// these errors are already triggered by rustc, which means to see the other errors I need to comment this out.
//fn break_disallowed() {
//    //~ ERROR:
//    while true {
//        ghost! {
//            break;
//        }
//    }
//}
//
//fn continue_disallowed() {
//    //~ ERROR:
//    while true {
//        ghost! {
//            continue;
//        }
//    }
//}

fn cannot_mutate_variables() {
    let mut x = 1;
    let g = ghost! {
        // do some random code here
        let mut x = 2;
        while x < 100 {
            x += 1;
        }
        5 // spooky return value
    };

    ghost! {
        let g = g.get();
        prusti_assert!(g == 5);
    };
}

fn main() {}
