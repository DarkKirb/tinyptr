use core::{marker::PhantomData, mem::MaybeUninit, num::NonZeroU16, ptr::Pointee};

use crate::{util::IntoTiny, Ref, RefMut};

use super::{ConstPtr, MutPtr};

pub struct NonNull<T, const BASE_ADDR: usize>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
    ptr: NonZeroU16,
    metadata: <<T as Pointee>::Metadata as IntoTiny>::Tiny,
    _phantom: PhantomData<ConstPtr<T, BASE_ADDR>>,
}

impl<T, const BASE_ADDR: usize> NonNull<T, BASE_ADDR>
where
    <T as Pointee>::Metadata: IntoTiny,
{
    pub fn dangling() -> Self {
        unsafe {
            Self::new_unchecked(MutPtr::new_unchecked(
                (core::mem::align_of::<T>() + BASE_ADDR) as *mut T,
            ))
        }
    }

    /// Returns a shared reference to the value as MaybeUninit
    ///
    /// # Safety
    /// This function converts a pointer to a reference
    pub unsafe fn as_uninit_ref<'a>(&self) -> Ref<'a, MaybeUninit<T>, BASE_ADDR> {
        self.as_ptr().as_uninit_ref_unchecked()
    }

    /// Returns a mutable reference to the value as MaybeUninit
    ///
    /// # Safety
    /// This function converts a pointer to a reference
    pub unsafe fn as_uninit_mut<'a>(&mut self) -> RefMut<'a, MaybeUninit<T>, BASE_ADDR> {
        self.as_ptr().as_uninit_mut_unchecked()
    }
}

impl<T, const BASE_ADDR: usize> NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
    /// Creates a new NonNull<T>
    ///
    /// # Safety
    /// The caller has to ensure that ptr is not null
    pub unsafe fn new_unchecked(ptr: MutPtr<T, BASE_ADDR>) -> Self {
        let (ptr, metadata) = ptr.as_raw_parts();
        Self {
            ptr: NonZeroU16::new_unchecked(ptr),
            metadata,
            _phantom: PhantomData,
        }
    }

    pub fn new(ptr: MutPtr<T, BASE_ADDR>) -> Option<Self> {
        let (ptr, metadata) = ptr.as_raw_parts();
        Some(Self {
            ptr: NonZeroU16::new(ptr)?,
            metadata,
            _phantom: PhantomData,
        })
    }

    pub fn from_raw_parts(
        data_address: NonNull<(), BASE_ADDR>,
        metadata: <<T as Pointee>::Metadata as IntoTiny>::Tiny,
    ) -> Self {
        Self {
            ptr: data_address.ptr,
            metadata,
            _phantom: PhantomData,
        }
    }

    pub fn to_raw_parts(
        self,
    ) -> (
        NonNull<(), BASE_ADDR>,
        <<T as Pointee>::Metadata as IntoTiny>::Tiny,
    ) {
        let (ptr, metadata) = self.as_ptr().as_raw_parts();
        (
            NonNull {
                ptr: unsafe { NonZeroU16::new_unchecked(ptr) },
                metadata: (),
                _phantom: PhantomData,
            },
            metadata,
        )
    }

    pub fn as_ptr(self) -> MutPtr<T, BASE_ADDR> {
        MutPtr::from_raw_parts(self.ptr.get(), self.metadata)
    }

    /// Returns a shared reference to the value
    ///
    /// # Safety
    /// This function converts a pointer to a reference
    pub unsafe fn as_ref<'a>(&self) -> Ref<'a, T, BASE_ADDR> {
        self.as_ptr().as_ref_unchecked()
    }

    /// Returns a mutable reference to the value
    ///
    /// # Safety
    /// This function converts a pointer to a reference
    pub unsafe fn as_mut<'a>(&mut self) -> RefMut<'a, T, BASE_ADDR> {
        self.as_ptr().as_mut_unchecked()
    }

    pub fn cast<U>(self) -> NonNull<U, BASE_ADDR>
    where
        <U as Pointee>::Metadata: IntoTiny,
    {
        unsafe { NonNull::new_unchecked(self.as_ptr().cast()) }
    }
}

impl<T, const BASE_ADDR: usize> Copy for NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}

impl<T, const BASE_ADDR: usize> Clone for NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const BASE_ADDR: usize> Eq for NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}
impl<T, const BASE_ADDR: usize> PartialEq for NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn eq(&self, o: &Self) -> bool {
        self.as_ptr().eq(&o.as_ptr())
    }
}
impl<T, const BASE_ADDR: usize> Ord for NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_ptr().cmp(&other.as_ptr())
    }
}
impl<T, const BASE_ADDR: usize> PartialOrd for NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T, const BASE_ADDR: usize> core::fmt::Pointer for NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl<T, const BASE_ADDR: usize> core::fmt::Debug for NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.as_ptr(), f)
    }
}

impl<T, const BASE_ADDR: usize> From<RefMut<'_, T, BASE_ADDR>> for NonNull<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn from(v: RefMut<'_, T, BASE_ADDR>) -> Self {
        v.ptr
    }
}
