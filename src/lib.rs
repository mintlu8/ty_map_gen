#![doc = include_str!("../README.md")]
mod index;

#[doc(hidden)]
pub use std::{
    any::{Any, TypeId},
    boxed::Box,
    collections::{BTreeMap, HashMap},
    hash::Hash,
    hash::RandomState,
};

#[doc(hidden)]
pub mod dyn_traits;
#[doc(hidden)]
pub use index::DualKeyIndex;

#[doc(hidden)]
pub use dyn_clone::clone_trait_object;
#[doc(hidden)]
pub use dyn_hash::hash_trait_object;

#[cfg(feature = "erased-serde")]
#[doc(hidden)]
pub use erased_serde::{serialize_trait_object, Serialize};

#[doc(hidden)]
pub use paste::paste;

#[doc(hidden)]
#[macro_export]
/// Generate trait impls for ths hidden trait
macro_rules! alias_impl {
    ($ty: ident Clone) => {
        $crate::clone_trait_object!($ty);
    };
    ($ty: ident PartialEq) => {
        $crate::partial_eq_trait_object!($ty);
    };
    ($ty: ident Eq) => {
        $crate::eq_trait_object!($ty);
    };
    ($ty: ident PartialOrd) => {
        $crate::partial_ord_trait_object!($ty);
    };
    ($ty: ident Ord) => {
        $crate::ord_trait_object!($ty);
    };
    ($ty: ident Hash) => {
        $crate::hash_trait_object!($ty);
    };
    ($ty: ident Serialize) => {
        $crate::serialize_trait_object!($ty);
    };
    ($($tt: tt)*) => {};
}

#[macro_export]
macro_rules! type_map {
    ($(#[$($attr: tt)*])* $vis: vis $ty: ident where $T: tt $([$($extras: tt)*])? => $OUT: ty as $map: ident ) => {
        $crate::type_map!($(#[$($attr)*])* $vis $ty where $T $([$($extras)*])? => $OUT [$crate::Any] as $map );
    };
    (@trait_obj $ty: ident [$($impls:tt)*]) => {
        $crate::paste!{
            mod [<__ $ty: snake _internal>] {
                #[allow(unused_imports)]
                use $crate::dyn_traits::*;
                pub(super) trait [<__ $ty Internal>]: $($impls)* {
                    fn __as_ref_ptr(&self) -> *const ();
                    fn __as_mut_ptr(&mut self) -> *mut ();
                }
                impl<T> [<__ $ty Internal>] for T where T: $($impls)* {
                    fn __as_ref_ptr(&self) -> *const (){
                        self as *const Self as *const ()
                    }

                    fn __as_mut_ptr(&mut self) -> *mut () {
                        self as *mut Self as *mut ()
                    }
                }

                impl dyn [<__ $ty Internal>] {
                    /// Safety: self must contain T.
                    pub unsafe fn downcast_unchecked<T>(self: Box<Self>) -> T{
                        *$crate::Box::<T>::from_raw($crate::Box::into_raw(self) as *mut T)
                    }
                }
                $($crate::alias_impl!{[<__ $ty Internal>] $impls})*
            }

            use [<__ $ty: snake _internal>]::[<__ $ty Internal>];
        }
    };
    ($(#[$($attr: tt)*])* $vis: vis $ty: ident where $T: ident $([$($extras: tt)*])? => $OUT: ty [$($impls:tt)*] as $map: ident ) => {
        $crate::type_map!(@trait_obj $ty [$($impls)*]);
        $crate::paste!{
            $(#[$($attr)*])*
            #[derive(Default)]
            $vis struct $ty($map<$crate::TypeId, $crate::Box<dyn [<__ $ty Internal>]>>);

            impl $ty {
                /// Creates an empty map.
                pub fn new() -> Self {
                    Self::default()
                }

                /// Returns `true` if the map contains no elements.
                pub fn is_empty(&self) -> bool {
                    self.0.is_empty()
                }

                /// Returns the number of elements in the map.
                pub fn len(&self) -> usize {
                    self.0.len()
                }

                /// Clears the map, removing all values. Keeps the allocated memory for reuse.
                pub fn clear(&mut self) {
                    self.0.clear()
                }

                /// Returns `true` if the map contains a value for the specified type.
                pub fn contains<$T $(: $($extras)*)?>(&mut self) -> bool where T: 'static, $OUT: $($impls)* {
                    self.0.contains_key(&$crate::TypeId::of::<T>())
                }

                /// Inserts a value into the map.
                pub fn insert<$T $(: $($extras)*)?>(&mut self, item: $OUT) -> Option<$OUT> where T: 'static, $OUT: $($impls)* {
                    self.0.insert($crate::TypeId::of::<T>(), Box::new(item))
                        .map(|x| unsafe {x.downcast_unchecked::<$OUT>()})
                }

                /// Removes a value from the map.
                pub fn remove<$T $(: $($extras)*)?>(&mut self) -> Option<$OUT> where T: 'static, $OUT: $($impls)* {
                    self.0.remove(&$crate::TypeId::of::<T>())
                        .map(|x| unsafe {x.downcast_unchecked::<$OUT>()})
                }

                /// Get a value into the map.
                pub fn get<$T $(: $($extras)*)?>(&self) -> Option<&$OUT> where T: 'static, $OUT: $($impls)* {
                    self.0.get(&$crate::TypeId::of::<T>())
                        .and_then(|x| unsafe {([<__ $ty Internal>]::__as_ref_ptr(&**x) as *const $OUT).as_ref()})
                }

                /// Get a mutable value into the map.
                pub fn get_mut<$T $(: $($extras)*)?>(&mut self) -> Option<&mut $OUT> where T: 'static, $OUT: $($impls)* {
                    self.0.get_mut(&$crate::TypeId::of::<T>())
                    .and_then(|x| unsafe {([<__ $ty Internal>]::__as_mut_ptr(&mut **x) as *mut $OUT).as_mut()})
                }
            }
        }
    };

    ($(#[$($attr: tt)*])* $vis: vis $ty: ident where ($T: ident, $K: ty) $([$($extras: tt)*])? => $OUT: ty [$($impls:tt)*] as $map: ident ) => {
        $crate::type_map!(@trait_obj $ty [$($impls)*]);
        $crate::paste!{
            $(#[$($attr)*])*
            #[derive(Default)]
            $vis struct $ty($map<($crate::TypeId, $K), $crate::Box<dyn [<__ $ty Internal>]>>);

            impl $ty {
                /// Creates an empty map.
                pub fn new() -> Self {
                    Self::default()
                }

                /// Returns `true` if the map contains no elements.
                pub fn is_empty(&self) -> bool {
                    self.0.is_empty()
                }

                /// Returns the number of elements in the map.
                pub fn len(&self) -> usize {
                    self.0.len()
                }

                /// Clears the map, removing all values. Keeps the allocated memory for reuse.
                pub fn clear(&mut self) {
                    self.0.clear()
                }

                /// Returns `true` if the map contains a value for the specified type.
                pub fn contains<$T $(: $($extras)*)?, Q: ?Sized>(&mut self, key: &Q) -> bool
                where
                    T: 'static, $OUT: $($impls)*, Q: ::std::cmp::Ord + ::std::hash::Hash, $K: std::borrow::Borrow<Q> {
                    self.0.contains_key(&($crate::TypeId::of::<T>(), key) as &dyn $crate::DualKeyIndex<Q>)
                }

                /// Inserts a value into the map.
                pub fn insert<$T $(: $($extras)*)?>(&mut self, key: $K, item: $OUT) -> Option<$OUT>
                where
                    T: 'static, $OUT: $($impls)* {
                    self.0.insert(($crate::TypeId::of::<T>(), key), Box::new(item))
                        .map(|x| unsafe {x.downcast_unchecked::<$OUT>()})
                }

                /// Removes a value from the map.
                pub fn remove<$T $(: $($extras)*)?, Q: ?Sized>(&mut self, key: &Q) -> Option<$OUT>
                where
                    T: 'static, $OUT: $($impls)*, Q: ::std::cmp::Ord + ::std::hash::Hash, $K: std::borrow::Borrow<Q> {
                    self.0.remove(&($crate::TypeId::of::<T>(), key) as &dyn $crate::DualKeyIndex<Q>)
                        .map(|x| unsafe {x.downcast_unchecked::<$OUT>()})
                }

                /// Get a value into the map.
                pub fn get<$T $(: $($extras)*)?, Q: ?Sized>(&self, key: &Q) -> Option<&$OUT>
                where
                    T: 'static, $OUT: $($impls)*, Q: ::std::cmp::Ord + ::std::hash::Hash, $K: std::borrow::Borrow<Q> {
                    self.0.get(&($crate::TypeId::of::<T>(), key) as &dyn $crate::DualKeyIndex<Q>)
                        .and_then(|x| unsafe {([<__ $ty Internal>]::__as_ref_ptr(&**x) as *const $OUT).as_ref()})
                }

                /// Get a mutable value into the map.
                pub fn get_mut<$T $(: $($extras)*)?, Q: ?Sized>(&mut self, key: &Q) -> Option<&mut $OUT>
                where
                    T: 'static, $OUT: $($impls)*, Q: ::std::cmp::Ord + ::std::hash::Hash, $K: std::borrow::Borrow<Q> {
                    self.0.get_mut(&($crate::TypeId::of::<T>(), key) as &dyn $crate::DualKeyIndex<Q>)
                        .and_then(|x| unsafe {([<__ $ty Internal>]::__as_mut_ptr(&mut **x) as *mut $OUT).as_mut()})
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::{BTreeMap, HashMap},
        sync::Arc,
    };

    macro_rules! is_send {
        ($ty: ty) => {
            const _: () = {
                struct IsSend<T: Send>(T);
                type AssetIsSend = IsSend<$ty>;
            };
        };
    }

    macro_rules! is_sync {
        ($ty: ty) => {
            const _: () = {
                struct IsSync<T: Sync>(T);
                type AssetIsSync = IsSync<$ty>;
            };
        };
    }

    type_map!(
        /// Maps T to Box<T>
        pub BoxMap where T [] => Box<T> as HashMap
    );
    //type_map!(pub BoxMap2 where T: (Clone) => Box<T>);
    type_map!(pub BoxSend where T [] => Arc<T> [Send + Sync] as BTreeMap);
    is_send!(BoxSend);
    is_sync!(BoxSend);
    type_map!(
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub OrdMap where T  => T [Ord + Clone] as BTreeMap
    );
    type_map!(pub PartialEqMap where T  => T [PartialEq] as BTreeMap);
    type_map!(pub EqMap where T  => T [Eq] as BTreeMap);
    type_map!(pub PartialOrdMap where T  => T [PartialOrd] as BTreeMap);

    type_map!(pub StringMap where (T, String)  => Vec<T> as BTreeMap);
}
