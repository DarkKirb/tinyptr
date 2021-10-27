use core::{
    borrow::{Borrow, BorrowMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::Pointee,
};

use crate::{ptr::NonNull, util::IntoTiny};

pub struct RefMut<'a, T, const BASE_ADDR: usize>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    pub(crate) ptr: NonNull<T, BASE_ADDR>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T, const BASE_ADDR: usize> RefMut<'a, T, BASE_ADDR>
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

impl<'a, T, const BASE_ADDR: usize> Deref for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ptr().as_wide_ptr() }
    }
}

impl<'a, T, const BASE_ADDR: usize> Borrow<T> for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn borrow(&self) -> &T {
        &*self
    }
}

impl<'a, T, const BASE_ADDR: usize> DerefMut for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr.as_ptr().as_wide_ptr() }
    }
}

impl<'a, T, const BASE_ADDR: usize> BorrowMut<T> for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn borrow_mut(&mut self) -> &mut T {
        &mut *self
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Pointer for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.ptr.fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Binary for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::Binary,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Debug for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::Debug,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Display for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::Display,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::LowerExp for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::LowerExp,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::LowerHex for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::LowerHex,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Octal for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::Octal,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::UpperExp for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::UpperExp,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::UpperHex for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::UpperHex,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<'a, T, const BASE_ADDR: usize> PartialOrd for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + PartialOrd,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        (**self).partial_cmp(&*other)
    }
}

impl<'a, T, const BASE_ADDR: usize> Ord for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + Ord,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (**self).cmp(&*other)
    }
}

impl<'a, T, const BASE_ADDR: usize> PartialEq for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + PartialEq,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        (**self).eq(&*other)
    }
}

impl<'a, T, const BASE_ADDR: usize> Eq for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + Eq,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}

impl<'a, T, U, const BASE_ADDR: usize> AsRef<U> for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + AsRef<U>,
    U: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn as_ref(&self) -> &U {
        (**self).as_ref()
    }
}

impl<'a, T, const BASE_ADDR: usize> core::hash::Hash for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::hash::Hash,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<'a, T, U, const BASE_ADDR: usize> AsMut<U> for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + AsMut<U>,
    U: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn as_mut(&mut self) -> &mut U {
        (**self).as_mut()
    }
}

impl<'a, T, const BASE_ADDR: usize> core::fmt::Write for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + core::fmt::Write,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        (**self).write_str(s)
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        (**self).write_char(c)
    }

    fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
        (**self).write_fmt(args)
    }
}

unsafe impl<'a, T, const BASE_ADDR: usize> Send for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + Send,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}

unsafe impl<'a, T, const BASE_ADDR: usize> Sync for RefMut<'a, T, BASE_ADDR>
where
    T: ?Sized + Sync,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}
