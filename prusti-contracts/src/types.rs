/// A sequence type
#[non_exhaustive]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Seq<T> {
    _phantom: core::marker::PhantomData<T>,
}

impl<T> Ghost<T> {
    pub fn new(_: T) -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub unsafe fn unsafe_new(_: &dyn Fn() -> T) -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn set(&self, _: T) {
        // this should be replaced by prusti
    }
}

/// a mathematical (unbounded) integer type
#[derive(PartialOrd, Ord)]
pub struct Int(());
impl Int {
    pub fn new(_: i64) -> Self {
        Self(())
    }
}

macro_rules! __int_dummy_trait_impls__ {
        ($($trait:ident $fun:ident),*) => {$(
            impl core::ops::$trait for Int {
                type Output = Self;
                fn $fun(self, _other: Self) -> Self {
                    self
                }
            }
        )*}
    }

__int_dummy_trait_impls__!(Add add, Sub sub, Mul mul, Div div, Rem rem);

impl core::ops::Neg for Int {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}

/// A sequence type
#[non_exhaustive]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Seq<T: Copy> {
    _phantom: core::marker::PhantomData<T>,
}

impl<T: Copy> Seq<T> {
    pub fn empty() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn single(_: T) -> Self {
        Self::empty()
    }
    pub fn concat(self, _: Self) -> Self {
        Self::empty()
    }
    #[cfg(feature = "prusti")]
    pub fn lookup(self, _index: usize) -> T {
        panic!()
    }
    pub fn len(self) -> Int {
        Int::new(0)
    }
}

#[macro_export]
macro_rules! seq {
        ($val:expr) => {
            $crate::Seq::single($val)
        };
        ($($val:expr),*) => {
            $crate::Seq::empty()
            $(
                .concat(seq![$val])
            )*
        };
    }

#[cfg(feature = "prusti")]
impl<T: Copy> Index<usize> for Seq<T> {
    type Output = T;
    fn index(&self, _: usize) -> &T {
        panic!()
    }
}

#[cfg(feature = "prusti")]
impl<T: Copy> Index<Int> for Seq<T> {
    type Output = T;
    fn index(&self, _: Int) -> &T {
        panic!()
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
    pub fn empty() -> Self {
        Self {
            _key_phantom: core::marker::PhantomData,
            _val_phantom: core::marker::PhantomData,
        }
    }
    pub fn insert(self, _key: K, _val: V) -> Self {
        Self::empty()
    }
    pub fn delete(self, _key: K) -> Self {
        Self::empty()
    }
    pub fn len(self) -> Int {
        Self::empty()
    }
    #[cfg(feature = "prusti")]
    pub fn lookup(self, _key: K) -> V {
        panic!()
    }
    #[cfg(feature = "prusti")]
    pub fn contains(self, _key: K) -> bool {
        panic!()
    }
}

#[macro_export]
macro_rules! map {
    ($($key:expr => $val:expr),*) => {
        map!($crate::Map::empty(), $($key => $val),*)
    };
    ($existing_map:expr, $($key:expr => $val:expr),*) => {
        $existing_map
        $(
            .insert($key, $val)
        )*
    };
}

#[cfg(feature = "prusti")]
impl<K, V> core::ops::Index<K> for Map<K, V> {
    type Output = V;
    fn index(&self, _key: K) -> &V {
        panic!()
    }
}

#[cfg(not(feature = "prusti"))]
#[non_exhaustive]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Ghost<T> {
    _phantom: core::marker::PhantomData<T>,
}
#[cfg(feature = "prusti")]
pub type Ghost<T> = T;

#[cfg(feature = "prusti")]
pub trait GetSet {
    fn get(self) -> Self;
    fn set(&mut self, value: Self);
}

#[cfg(feature = "prusti")]
impl<T> GetSet for T {
    fn get(self) -> Self {
        panic!()
    }
    fn set(&mut self, value: Self) {
        panic!()
    }
}

#[cfg(feature = "prusti")]
pub fn snapshot_equality<T>(_: T, _: T) -> bool {
    panic!()
}
