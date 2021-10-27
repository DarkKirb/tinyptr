use core::{borrow::Borrow, marker::PhantomData, ops::Deref, ptr::Pointee};

use crate::{ptr::NonNull, util::IntoTiny};

pub struct Ref<'a, T, const BASE_ADDR: usize>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    pub(crate) ptr: NonNull<T, BASE_ADDR>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T, const BASE_ADDR: usize> Ref<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    pub(crate) const unsafe fn new(ptr: NonNull<T, BASE_ADDR>) -> Self {
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, const BASE_ADDR: usize> Copy for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}

impl<'a, T, const BASE_ADDR: usize> Clone for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T, const BASE_ADDR: usize> Deref for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ptr().as_wide_ptr() }
    }
}

impl<'a, T, const BASE_ADDR: usize> Borrow<T> for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn borrow(&self) -> &T {
        &*self
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Pointer for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.ptr.fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Binary for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::Binary,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Debug for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::Debug,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Display for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::Display,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::LowerExp for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::LowerExp,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::LowerHex for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::LowerHex,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Octal for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::Octal,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::UpperExp for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::UpperExp,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::UpperHex for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::UpperHex,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> PartialOrd for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + PartialOrd,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        (**self).partial_cmp(&*other)
    }
}

impl<'a, T, const BASE_ADDR: usize> Ord for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + Ord,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (**self).cmp(&*other)
    }
}

impl<'a, T, const BASE_ADDR: usize> PartialEq for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + PartialEq,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        (**self).eq(&*other)
    }
}

impl<'a, T, const BASE_ADDR: usize> Eq for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + Eq,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}

impl<'a, T, U, const BASE_ADDR: usize> AsRef<U> for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + AsRef<U>,
    U: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn as_ref(&self) -> &U {
        (**self).as_ref()
    }
}

impl<'a, T, const BASE_ADDR: usize> core::hash::Hash for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + core::hash::Hash,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

unsafe impl<'a, T, const BASE_ADDR: usize> Send for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + Sync,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}

unsafe impl<'a, T, const BASE_ADDR: usize> Sync for Ref<'a, T, BASE_ADDR>
where
    T: ?Sized + Sync,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}
