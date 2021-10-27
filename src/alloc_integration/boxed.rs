use core::{
    alloc::{AllocError, Allocator, Layout},
    any::Any,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    pin::Pin,
    ptr::Pointee,
};

use alloc::alloc::Global;

use crate::{
    ptr::{MutPtr, Unique},
    util::IntoTiny,
    Ref, RefMut,
};

pub struct Box<T, A, const BASE_ADDR: usize>(Unique<T, BASE_ADDR>, A)
where
    T: ?Sized,
    A: Allocator,
    <T as Pointee>::Metadata: IntoTiny;

impl<T, A, const BASE_ADDR: usize> Box<T, A, BASE_ADDR>
where
    T: ?Sized,
    A: Allocator,
    <T as Pointee>::Metadata: IntoTiny,
{
    pub fn into_raw_with_allocator(b: Self) -> (MutPtr<T, BASE_ADDR>, A) {
        let alloc = unsafe { core::ptr::read(&b.1) };
        (b.0.as_ptr(), alloc)
    }
    pub fn into_raw(b: Self) -> MutPtr<T, BASE_ADDR> {
        Self::into_raw_with_allocator(b).0
    }

    /// Creates a new Box from raw data
    ///
    /// # Safety
    /// See alloc's box safety docs
    pub unsafe fn from_raw_in(ptr: MutPtr<T, BASE_ADDR>, allocator: A) -> Self {
        Self(Unique::new_unchecked(ptr), allocator)
    }

    /// Creates a new Box from raw data
    ///
    /// # Safety
    /// See alloc's box safety docs
    pub unsafe fn from_raw(ptr: MutPtr<T, BASE_ADDR>) -> Box<T, Global, BASE_ADDR> {
        Box::from_raw_in(ptr, Global)
    }

    pub fn as_mut_ptr(&mut self) -> MutPtr<T, BASE_ADDR> {
        self.0.as_ptr()
    }
}

impl<T, A, const BASE_ADDR: usize> Box<T, A, BASE_ADDR>
where
    A: Allocator,
    <T as Pointee>::Metadata: IntoTiny,
{
    pub fn try_new_uninit_in(alloc: A) -> Result<Box<MaybeUninit<T>, A, BASE_ADDR>, AllocError> {
        let layout = Layout::new::<MaybeUninit<T>>();
        let ptr = alloc.allocate(layout)?.cast();
        unsafe {
            Ok(Box::from_raw_in(
                MutPtr::new(ptr.as_ptr()).ok_or(AllocError)?,
                alloc,
            ))
        }
    }
    pub fn try_new_zeroed_in(alloc: A) -> Result<Box<MaybeUninit<T>, A, BASE_ADDR>, AllocError> {
        let layout = Layout::new::<MaybeUninit<T>>();
        let ptr = alloc.allocate_zeroed(layout)?.cast();
        unsafe {
            Ok(Box::from_raw_in(
                MutPtr::new(ptr.as_ptr()).ok_or(AllocError)?,
                alloc,
            ))
        }
    }

    pub fn try_new_in(x: T, alloc: A) -> Result<Self, AllocError> {
        let mut boxed = Self::try_new_uninit_in(alloc)?;
        unsafe {
            boxed.as_mut_ptr().cast::<T>().write(x);
            Ok(boxed.assume_init())
        }
    }

    pub fn new_uninit_in(alloc: A) -> Box<MaybeUninit<T>, A, BASE_ADDR> {
        Self::try_new_uninit_in(alloc).expect("Out of Memory")
    }

    pub fn new_zeroed_in(alloc: A) -> Box<MaybeUninit<T>, A, BASE_ADDR> {
        Self::try_new_zeroed_in(alloc).expect("Out of Memory")
    }

    pub fn new_in(x: T, alloc: A) -> Self {
        Self::try_new_in(x, alloc).expect("Out of Memory")
    }

    pub fn try_new_uninit() -> Result<Box<MaybeUninit<T>, Global, BASE_ADDR>, AllocError> {
        Box::try_new_uninit_in(Global)
    }

    pub fn try_new_zeroed() -> Result<Box<MaybeUninit<T>, Global, BASE_ADDR>, AllocError> {
        Box::try_new_zeroed_in(Global)
    }

    pub fn try_new(x: T) -> Result<Box<T, Global, BASE_ADDR>, AllocError> {
        Box::try_new_in(x, Global)
    }

    pub fn new_uninit() -> Box<MaybeUninit<T>, Global, BASE_ADDR> {
        Box::new_uninit_in(Global)
    }
    pub fn new_zeroed() -> Box<MaybeUninit<T>, Global, BASE_ADDR> {
        Box::new_zeroed_in(Global)
    }
    pub fn new(x: T) -> Box<T, Global, BASE_ADDR> {
        Box::new_in(x, Global)
    }

    pub fn into_pin(boxed: Self) -> Pin<Self>
    where
        A: 'static,
    {
        unsafe { Pin::new_unchecked(boxed) }
    }

    pub fn try_pin_in(x: T, alloc: A) -> Result<Pin<Self>, AllocError>
    where
        A: 'static,
    {
        Self::try_new_in(x, alloc).map(Box::into_pin)
    }

    pub fn pin_in(x: T, alloc: A) -> Pin<Self>
    where
        A: 'static,
    {
        Box::into_pin(Self::new_in(x, alloc))
    }

    pub fn try_pin(x: T) -> Result<Pin<Box<T, Global, BASE_ADDR>>, AllocError> {
        Box::try_pin_in(x, Global)
    }

    pub fn pin(x: T) -> Pin<Box<T, Global, BASE_ADDR>> {
        Box::pin_in(x, Global)
    }

    pub fn into_boxed_slice(boxed: Self) -> Box<[T], A, BASE_ADDR> {
        let (ptr, alloc) = Box::into_raw_with_allocator(boxed);
        unsafe {
            let ptr = MutPtr::new_unchecked(ptr.as_wide_ptr() as *mut [T; 1] as *mut [T]);
            Box::from_raw_in(ptr, alloc)
        }
    }

    pub fn into_inner(boxed: Self) -> T {
        let (ptr, alloc) = Box::into_raw_with_allocator(boxed);
        unsafe {
            let boxed = Box::from_raw_in(ptr.cast::<MaybeUninit<T>>(), alloc);
            let ptr = ptr.read();
            core::mem::drop(boxed);
            ptr
        }
    }
}

impl<T, A, const BASE_ADDR: usize> Box<MaybeUninit<T>, A, BASE_ADDR>
where
    A: Allocator,
    <T as Pointee>::Metadata: IntoTiny,
{
    /// Converts to `Box<T, A, BASE_ADDR>`
    ///
    /// # Safety
    /// The caller has to ensure that the data is validly initialized
    pub unsafe fn assume_init(self) -> Box<T, A, BASE_ADDR> {
        let (raw, alloc) = Box::into_raw_with_allocator(self);
        Box::from_raw_in(raw.cast(), alloc)
    }
}

impl<T, A, const BASE_ADDR: usize> Box<[MaybeUninit<T>], A, BASE_ADDR>
where
    A: Allocator,
    <T as Pointee>::Metadata: IntoTiny,
{
    /// Converts to `Box<T, A, BASE_ADDR>`
    ///
    /// # Safety
    /// The caller has to ensure that the data is validly initialized
    pub unsafe fn assume_init(self) -> Box<[T], A, BASE_ADDR> {
        let (raw, alloc) = Box::into_raw_with_allocator(self);
        let (ptr, metadata) = raw.as_raw_parts();
        Box::from_raw_in(MutPtr::from_raw_parts(ptr, metadata), alloc)
    }
}

impl<T, A, const BASE_ADDR: usize> Drop for Box<T, A, BASE_ADDR>
where
    T: ?Sized,
    A: Allocator,
    <T as Pointee>::Metadata: IntoTiny,
{
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::for_value(&*self);
            self.0.as_ptr().as_wide_ptr().drop_in_place();
            self.1.deallocate(
                core::ptr::NonNull::new_unchecked(self.0.as_ptr().as_wide_ptr().cast::<u8>()),
                layout,
            );
        }
    }
}

impl<T, A, const BASE_ADDR: usize> Deref for Box<T, A, BASE_ADDR>
where
    T: ?Sized,
    A: Allocator,
    <T as Pointee>::Metadata: IntoTiny,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { Ref::as_std_ref(self.0.as_ptr().as_ref_unchecked()) }
    }
}

impl<T, A, const BASE_ADDR: usize> DerefMut for Box<T, A, BASE_ADDR>
where
    T: ?Sized,
    A: Allocator,
    <T as Pointee>::Metadata: IntoTiny,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { RefMut::as_std_mut(self.0.as_ptr().as_mut_unchecked()) }
    }
}

impl<A, const BASE_ADDR: usize> Box<dyn Any, A, BASE_ADDR>
where
    A: Allocator,
{
    pub fn downcast<T: Any>(self) -> Result<Box<T, A, BASE_ADDR>, Self>
    where
        <T as Pointee>::Metadata: IntoTiny,
    {
        if self.is::<T>() {
            unsafe {
                let (raw, alloc) = Box::into_raw_with_allocator(self);
                Ok(Box::from_raw_in(raw.cast::<T>(), alloc))
            }
        } else {
            Err(self)
        }
    }
}

impl<A, const BASE_ADDR: usize> Box<dyn Any + Send, A, BASE_ADDR>
where
    A: Allocator,
{
    pub fn downcast<T: Any>(self) -> Result<Box<T, A, BASE_ADDR>, Self>
    where
        <T as Pointee>::Metadata: IntoTiny,
    {
        if self.is::<T>() {
            unsafe {
                let (raw, alloc) = Box::into_raw_with_allocator(self);
                Ok(Box::from_raw_in(raw.cast::<T>(), alloc))
            }
        } else {
            Err(self)
        }
    }
}

impl<A, const BASE_ADDR: usize> Box<dyn Any + Send + Sync, A, BASE_ADDR>
where
    A: Allocator,
{
    pub fn downcast<T: Any>(self) -> Result<Box<T, A, BASE_ADDR>, Self>
    where
        <T as Pointee>::Metadata: IntoTiny,
    {
        if self.is::<T>() {
            unsafe {
                let (raw, alloc) = Box::into_raw_with_allocator(self);
                Ok(Box::from_raw_in(raw.cast::<T>(), alloc))
            }
        } else {
            Err(self)
        }
    }
}

pub fn test(test: Box<u32, Global, 0x8000_0000>) -> u32 {
    Box::into_inner(test)
}
