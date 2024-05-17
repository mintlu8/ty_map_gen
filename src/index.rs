use std::any::TypeId;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::Hash;

pub trait DualKeyIndex<T: ?Sized> {
    fn ty(&self) -> TypeId;
    fn name(&self) -> &T;
}

impl<T: ?Sized> PartialEq for dyn DualKeyIndex<T> + '_
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.ty() == other.ty() && self.name() == other.name()
    }
}

impl<T: ?Sized> Eq for dyn DualKeyIndex<T> + '_ where T: Eq {}

impl<T: ?Sized> PartialOrd for dyn DualKeyIndex<T> + '_
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ty().cmp(&other.ty()) {
            Ordering::Equal => self.name().partial_cmp(other.name()),
            order => Some(order),
        }
    }
}

impl<T: ?Sized> Ord for dyn DualKeyIndex<T> + '_
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ty()
            .cmp(&other.ty())
            .then(self.name().cmp(other.name()))
    }
}

impl<T: ?Sized> Hash for dyn DualKeyIndex<T> + '_
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ty().hash(state);
        self.name().hash(state);
    }
}

impl<A, T: ?Sized> DualKeyIndex<T> for (TypeId, A)
where
    A: Borrow<T>,
{
    fn ty(&self) -> TypeId {
        self.0
    }

    fn name(&self) -> &T {
        self.1.borrow()
    }
}
impl<'a, A, T: ?Sized> Borrow<dyn DualKeyIndex<T> + 'a> for (TypeId, A)
where
    A: Borrow<T> + 'a,
{
    fn borrow(&self) -> &(dyn DualKeyIndex<T> + 'a) {
        self
    }
}
