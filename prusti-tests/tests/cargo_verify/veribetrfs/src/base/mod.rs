pub mod option;
pub mod vector;

pub use option::Opt;
pub use vector::Vector;

pub use prusti_contracts::*;

pub fn replace<T>(dest: &mut T, src: T) -> T {
    std::mem::replace(dest, src)
}
