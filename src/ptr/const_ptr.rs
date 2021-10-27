use core::{fmt::Pointer, marker::PhantomData, mem::MaybeUninit, ptr::Pointee};

use crate::{util::IntoTiny, Ref, RefMut};

use super::{MutPtr, NonNull};

pub struct ConstPtr<T, const BASE_ADDR: usize>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
    ptr: u16,
    metadata: <<T as Pointee>::Metadata as IntoTiny>::Tiny,
    _phantom: PhantomData<*const T>,
}

impl<T, const BASE_ADDR: usize> ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
    pub fn new(ptr: *const T) -> Option<Self> {
        let (ptr, metadata) = ptr.to_raw_parts();
        Some(Self {
            ptr: crate::ptr_to_u16::<BASE_ADDR>(ptr)?,
            metadata: metadata.into_tiny()?,
            _phantom: PhantomData,
        })
    }
    /// Creates a new tiny pointer, unchecked
    ///
    /// # Safety
    /// The caller has to ensure that the pointer points to the initialized ram buffer. They also have to ensure that the fat pointer metadata can be represented in a compressed form
    pub unsafe fn new_unchecked(ptr: *const T) -> Self {
        let (ptr, metadata) = ptr.to_raw_parts();
        Self {
            ptr: crate::ptr_to_u16_unchecked::<BASE_ADDR>(ptr),
            metadata: metadata.into_tiny_unchecked(),
            _phantom: PhantomData,
        }
    }
    pub fn as_wide_ptr(self) -> *const T {
        // SAFE: this function can't be called without the ram buffer being initialized
        let ptr = crate::u16_to_ptr::<BASE_ADDR>(self.ptr);
        core::ptr::from_raw_parts(
            ptr,
            <<T as Pointee>::Metadata as IntoTiny>::from_tiny(self.metadata),
        )
    }
    pub fn is_null(self) -> bool {
        self.ptr == 0
    }
    pub fn cast<U>(self) -> ConstPtr<U, BASE_ADDR>
    where
        <U as Pointee>::Metadata: IntoTiny,
    {
        // SAFE: We know from the trait bounds and the fact that it comes from a valid pointer that this function is safe
        unsafe { ConstPtr::new_unchecked(self.as_wide_ptr().cast()) }
    }
    pub fn as_raw_parts(self) -> (u16, <<T as Pointee>::Metadata as IntoTiny>::Tiny) {
        (self.ptr, self.metadata)
    }
    /// Returns a shared reference to the pointer destination
    ///
    /// # Safety
    /// This function is unsafe because it converts a pointer into a reference.
    pub unsafe fn as_ref<'a>(self) -> Option<Ref<'a, T, BASE_ADDR>> {
        if self.is_null() {
            None
        } else {
            Some(self.as_ref_unchecked())
        }
    }
    /// Returns a shared reference to the pointer destination without checking for null
    ///
    /// # Safety
    /// This function is unsafe because it converts a pointer into a reference.
    pub unsafe fn as_ref_unchecked<'a>(self) -> Ref<'a, T, BASE_ADDR> {
        Ref::new(NonNull::new_unchecked(self.into()))
    }
    /// Returns a shared reference to a MaybeUninit
    /// # Safety
    /// This function is unsafe because it converts a pointer into a reference.
    pub unsafe fn as_uninit_ref<'a>(self) -> Option<Ref<'a, MaybeUninit<T>, BASE_ADDR>>
    where
        T: Sized,
    {
        self.cast::<MaybeUninit<T>>().as_ref()
    }
    /// Returns a shared reference to a MaybeUninit, without checking for null
    /// # Safety
    /// This function is unsafe because it converts a pointer into a reference.
    pub unsafe fn as_uninit_ref_unchecked<'a>(self) -> Ref<'a, MaybeUninit<T>, BASE_ADDR>
    where
        T: Sized,
    {
        self.cast::<MaybeUninit<T>>().as_ref_unchecked()
    }
    /// Calculates the offset from a pointer
    ///
    /// # Safety
    /// This function has undefined behaviour if count or the resulting pointer overflows
    pub unsafe fn offset(self, count: i16) -> Self
    where
        T: Sized,
    {
        self.wrapping_offset(count)
    }
    /// Calculates the offset from a pointer, wrapping if count or the pointer overflows
    pub fn wrapping_offset(mut self, count: i16) -> Self
    where
        T: Sized,
    {
        self.ptr = self
            .ptr
            .wrapping_add_signed(count.wrapping_mul(core::mem::size_of::<T>() as i16));
        self
    }
    /// Calculates the offset between two pointers
    ///
    /// # Safety
    /// See the normal offset_from documentation
    pub unsafe fn offset_from(self, origin: Self) -> isize
    where
        T: Sized,
    {
        self.as_wide_ptr().offset_from(origin.as_wide_ptr())
    }

    /// Shorthand for `offset(cont as i16)`
    ///
    /// # Safety
    /// see documentation for offset()
    pub unsafe fn add(self, count: u16) -> Self
    where
        T: Sized,
    {
        self.offset(count as i16)
    }

    /// Shorthand for `offset((count as i16).wrapping_neg())`
    ///
    /// # Safety
    /// see documentation for offset()
    pub unsafe fn sub(self, count: u16) -> Self
    where
        T: Sized,
    {
        self.offset((count as i16).wrapping_neg())
    }

    pub fn wrapping_add(self, count: u16) -> Self
    where
        T: Sized,
    {
        self.wrapping_offset(count as i16)
    }

    pub fn wrapping_sub(self, count: u16) -> Self
    where
        T: Sized,
    {
        self.wrapping_offset((count as i16).wrapping_neg())
    }

    pub fn set_ptr_value(mut self, val: ConstPtr<u8, BASE_ADDR>) -> Self {
        self.ptr = val.ptr;
        self
    }

    /// Reads the pointer
    ///
    /// # Safety
    /// This dereferences a pointer
    pub unsafe fn read(self) -> T
    where
        T: Sized,
    {
        self.as_wide_ptr().read()
    }
    /// Reads the pointer in a volatile manner
    ///
    /// # Safety
    /// This dereferences a pointer
    pub unsafe fn read_volatile(self) -> T
    where
        T: Sized,
    {
        self.as_wide_ptr().read_volatile()
    }
    /// Reads the pointer in an unaligned manner
    ///
    /// # Safety
    /// This dereferences a pointer
    pub unsafe fn read_unaligned(self) -> T
    where
        T: Sized,
    {
        self.as_wide_ptr().read_unaligned()
    }
    /// Copies count elements from this pointer to the dest pointer
    ///
    /// # Safety
    /// This dereferences raw pointers
    pub unsafe fn copy_to(self, dest: MutPtr<T, BASE_ADDR>, count: usize)
    where
        T: Sized,
    {
        self.as_wide_ptr().copy_to(dest.as_wide_ptr(), count)
    }
    /// Copies count elements from this pointer to the dest pointer
    ///
    /// # Safety
    /// This dereferences raw pointers
    pub unsafe fn copy_to_nonoverlapping(self, dest: MutPtr<T, BASE_ADDR>, count: usize)
    where
        T: Sized,
    {
        self.as_wide_ptr()
            .copy_to_nonoverlapping(dest.as_wide_ptr(), count)
    }
    pub fn align_offset(self, align: usize) -> usize
    where
        T: Sized,
    {
        self.as_wide_ptr().align_offset(align)
    }

    pub fn from_raw_parts(
        ptr: u16,
        metadata: <<T as Pointee>::Metadata as IntoTiny>::Tiny,
    ) -> Self {
        Self {
            ptr,
            metadata,
            _phantom: PhantomData,
        }
    }
}

impl<T, const BASE_ADDR: usize> Copy for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}

impl<T, const BASE_ADDR: usize> Clone for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const BASE_ADDR: usize> Eq for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}
impl<T, const BASE_ADDR: usize> PartialEq for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn eq(&self, o: &Self) -> bool {
        self.as_wide_ptr().eq(&o.as_wide_ptr())
    }
}
impl<T, const BASE_ADDR: usize> Ord for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_wide_ptr().cmp(&other.as_wide_ptr())
    }
}
impl<T, const BASE_ADDR: usize> PartialOrd for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, const BASE_ADDR: usize> Pointer for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_wide_ptr().fmt(f)
    }
}

impl<T, const BASE_ADDR: usize> core::fmt::Debug for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.as_wide_ptr(), f)
    }
}

impl<T, const BASE_ADDR: usize> From<MutPtr<T, BASE_ADDR>> for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn from(v: MutPtr<T, BASE_ADDR>) -> Self {
        let (ptr, metadata) = v.as_raw_parts();
        Self::from_raw_parts(ptr, metadata)
    }
}

impl<T, const BASE_ADDR: usize> From<Ref<'_, T, BASE_ADDR>> for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn from(v: Ref<'_, T, BASE_ADDR>) -> Self {
        v.ptr.as_ptr().into()
    }
}

impl<T, const BASE_ADDR: usize> From<RefMut<'_, T, BASE_ADDR>> for ConstPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn from(v: RefMut<'_, T, BASE_ADDR>) -> Self {
        v.ptr.as_ptr().into()
    }
}
