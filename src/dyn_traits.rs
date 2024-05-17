use std::{any::Any, cmp::Ordering};

pub use dyn_clone::DynClone as Clone;
pub use dyn_hash::DynHash as Hash;

#[cfg(feature = "erased-serde")]
pub use erased_serde::Serialize;

pub trait PartialEq: std::any::Any {
    fn dyn_eq(&self, other: &dyn PartialEq) -> bool;
    fn as_dyn_eq(&self) -> &dyn PartialEq;
    fn as_any(&self) -> &dyn Any;
}

pub trait Eq: PartialEq {}

impl<T> PartialEq for T
where
    T: std::cmp::PartialEq + std::any::Any,
{
    fn dyn_eq(&self, other: &dyn PartialEq) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map(|x| x == self)
            .unwrap_or(false)
    }

    fn as_dyn_eq(&self) -> &dyn PartialEq {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T> Eq for T where T: PartialEq + std::cmp::Eq {}

pub trait PartialOrd: std::any::Any {
    fn dyn_partial_cmp(&self, other: &dyn PartialOrd) -> Option<Ordering>;
    fn as_partial_ord(&self) -> &dyn PartialOrd;
    fn as_any(&self) -> &dyn Any;
}

pub trait Ord: PartialOrd {
    fn dyn_cmp(&self, other: &dyn Ord) -> Ordering;
    fn as_ord(&self) -> &dyn Ord;
}

impl<T> PartialOrd for T
where
    T: std::cmp::PartialOrd + std::any::Any,
{
    fn dyn_partial_cmp(&self, other: &dyn PartialOrd) -> Option<Ordering> {
        other
            .as_any()
            .downcast_ref::<Self>()
            .and_then(|x| self.partial_cmp(x))
    }

    fn as_partial_ord(&self) -> &dyn PartialOrd {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T> Ord for T
where
    T: std::cmp::Ord + PartialOrd + PartialEq,
{
    fn dyn_cmp(&self, other: &dyn Ord) -> Ordering {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map(|x| self.cmp(x))
            .unwrap_or(self.type_id().cmp(&other.type_id()))
    }

    fn as_ord(&self) -> &dyn Ord {
        self
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! partial_eq_trait_object {
    ($obj: ident) => {
        impl ::std::cmp::PartialEq for Box<dyn $obj> {
            fn eq(&self, other: &Self) -> bool {
                self.dyn_eq(other.as_dyn_eq())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! eq_trait_object {
    ($obj: ident) => {
        impl ::std::cmp::PartialEq for Box<dyn $obj> {
            fn eq(&self, other: &Self) -> bool {
                self.dyn_eq(other.as_dyn_eq())
            }
        }

        impl ::std::cmp::Eq for Box<dyn $obj> {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! partial_ord_trait_object {
    ($obj: ident) => {
        $crate::partial_eq_trait_object!($obj);
        impl ::std::cmp::PartialOrd for Box<dyn $obj> {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                self.dyn_partial_cmp(other.as_partial_ord())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ord_trait_object {
    ($obj: ident) => {
        $crate::eq_trait_object!($obj);
        impl ::std::cmp::PartialOrd for Box<dyn $obj> {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl ::std::cmp::Ord for Box<dyn $obj> {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.dyn_cmp(other.as_ord())
            }
        }
    };
}
