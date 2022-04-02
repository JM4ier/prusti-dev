// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

type Seq = prusti_contracts::Seq<i32>;
type Map = prusti_contracts::Map<i32, i32>;

#[ensures(Seq::empty().len() == Int::new(0))]
fn test1() {}

fn main() {}
