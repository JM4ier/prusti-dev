// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn main() {}

//#[terminates]
//fn x(mut x: i64) {
//    while x > 1 {
//        body_variant!(Int::new(x));
//        x -= 1;
//    }
//}
//
//#[terminates]
//fn a(mut a: i64) {
//    while a > 0 {
//        body_variant!(Int::new(a));
//        let mut b = a;
//        while b > 0 {
//            body_variant!(Int::new(b));
//            b -= 1;
//        }
//        a -= 1;
//    }
//}

//#[terminates]
//fn unsound(mut u : i64) {
//    while u > 0 {
//        body_variant!(Int::new(u)); // ~ERROR
//    }
//}
//
#[terminates]
fn y(mut y: i64) {
    while y > 1 {
        body_invariant!(true);
        body_variant!(Int::new(y)); // FIXME: this should also verify
        y -= 1;
    }
}
//
//#[terminates]
//#[requires(x >= 0)]
//#[ensures(result == fib(x))]
//fn fibi(x: i64) -> i64 {
//    let mut i = 0;
//    let mut a = 0;
//    let mut b = 1;
//    while i < x {
//        body_variant!(Int::new(x) - Int::new(i));
//        body_invariant!(i < x);
//        body_invariant!(i >= 0);
//        body_invariant!(a == fib(i));
//        body_invariant!(b == fib(i + 1));
//        let c = a + b;
//        prusti_assert!(c == fib(i + 2));
//        a = b;
//        b = c;
//        i += 1;
//    }
//    prusti_assert!(i == x);
//    prusti_assert!(a == fib(i));
//    a
//}
//
//#[pure]
//#[requires(x >= 0)]
//#[terminates(Int::new(x))]
//fn fib(x: i64) -> i64 {
//    if x <= 1 {
//        x
//    } else {
//        fib(x - 1) + fib(x - 2)
//    }
//}
