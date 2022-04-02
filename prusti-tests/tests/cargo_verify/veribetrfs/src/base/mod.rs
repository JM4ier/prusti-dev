pub mod option;
pub mod vector;

pub use option::Opt;
pub use vector::Vector;

pub use prusti_contracts::*;

//#[ensures(*dest == src && result == old(*dest))]
pub fn replace<T: Eq>(dest: &mut T, src: T) -> T {
    std::mem::replace(dest, src)
}