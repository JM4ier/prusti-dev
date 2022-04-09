#![no_std]

#[cfg(not(feature = "prusti"))]
mod private {
    /// A macro for writing a precondition on a function.
    pub use prusti_contracts_impl::requires;

    /// A macro for writing a postcondition on a function.
    pub use prusti_contracts_impl::ensures;

    /// A macro for writing a pledge on a function.
    pub use prusti_contracts_impl::after_expiry;

    /// A macro for writing a two-state pledge on a function.
    pub use prusti_contracts_impl::assert_on_expiry;

    /// A macro for marking a function as pure.
    pub use prusti_contracts_impl::pure;

    /// A macro for marking a function as trusted.
    pub use prusti_contracts_impl::trusted;

    /// A macro for writing a loop body invariant.
    pub use prusti_contracts_impl::body_invariant;

    /// A macro for defining a closure with a specification.
    /// Note: this is a declarative macro defined in this crate
    /// because declarative macros can't be exported from
    /// the `prusti-contracts-impl` proc-macro crate.
    /// See <https://github.com/rust-lang/rust/issues/40090>.
    #[macro_export]
    macro_rules! closure {
        ($condition:ident ($($args:tt)*), $($tail:tt)*) => {
            $crate::closure!($($tail)*)
        };
        ($($tail:tt)*) => {
            $($tail)*
        };
    }

    /// A macro for impl blocks that refine trait specifications.
    pub use prusti_contracts_impl::refine_trait_spec;

    /// A macro for specifying external functions.
    pub use prusti_contracts_impl::extern_spec;

    /// A macro for defining a predicate using prusti expression syntax instead
    /// of just Rust expressions.
    pub use prusti_contracts_impl::predicate;
}

#[cfg(feature = "prusti")]
mod private {
    /// A macro for writing a precondition on a function.
    pub use prusti_contracts_internal::requires;

    /// A macro for writing a postcondition on a function.
    pub use prusti_contracts_internal::ensures;

    /// A macro for writing a pledge on a function.
    pub use prusti_contracts_internal::after_expiry;

    /// A macro for writing a two-state pledge on a function.
    pub use prusti_contracts_internal::assert_on_expiry;

    /// A macro for marking a function as pure.
    pub use prusti_contracts_internal::pure;

    /// A macro for marking a function as trusted.
    pub use prusti_contracts_internal::trusted;

    /// A macro for writing a loop body invariant.
    pub use prusti_contracts_internal::body_invariant;

    /// A macro for defining a closure with a specification.
    pub use prusti_contracts_internal::closure;

    /// A macro for impl blocks that refine trait specifications.
    pub use prusti_contracts_internal::refine_trait_spec;

    /// A macro for specifying external functions.
    pub use prusti_contracts_internal::extern_spec;

    /// A macro for defining a predicate using prusti expression syntax instead
    /// of just Rust expressions.
    pub use prusti_contracts_internal::predicate;
}

/// This function is used to evaluate an expression in the context just
/// before the borrows expires.
pub fn before_expiry<T>(arg: T) -> T {
    arg
}

/// This function is used to evaluate an expression in the “old”
/// context, that is at the beginning of the method call.
pub fn old<T>(arg: T) -> T {
    arg
}

pub fn forall<T, F>(_trigger_set: T, _closure: F) -> bool {
    true
}

pub fn exists<T, F>(_trigger_set: T, _closure: F) -> bool {
    true
}

pub use private::*;

/// A sequence type
#[non_exhaustive]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Seq<T> {
    _phantom: core::marker::PhantomData<T>,
}

impl<T> Seq<T> {
    fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn empty() -> Self {
        Self::new()
    }
    pub fn single(_: T) -> Self {
        Self::new()
    }
    pub fn concat(_: Self, _: Self) -> Self {
        Self::new()
    }
}

/// A map type
#[non_exhaustive]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Map<K, V> {
    _key_phantom: core::marker::PhantomData<K>,
    _val_phantom: core::marker::PhantomData<V>,
}

impl<K, V> Map<K, V> {
    fn new() -> Self {
        Self {
            _key_phantom: core::marker::PhantomData,
            _val_phantom: core::marker::PhantomData,
        }
    }
    pub fn empty() -> Self {
        Self::new()
    }
    pub fn insert(self, _key: K, _val: V) -> Self {
        self
    }
    pub fn delete(self, _key: K) -> Self {
        self
    }
}
