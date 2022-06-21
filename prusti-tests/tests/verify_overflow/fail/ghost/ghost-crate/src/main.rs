// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn main() {
    let mut foo = 10;

    let mut five: Ghost<u32> = ghost! {
        5
    };
    ghost! {
        let val = five.get();
        five.set(6);
        prusti_assert!(val == 5);
    };
}

//fn ghost_eq(x: u32) {
//    //prusti_assert!((Ghost::new(x) === Ghost::new(x)));
//    prusti_assert!(1 === 1);
//}
