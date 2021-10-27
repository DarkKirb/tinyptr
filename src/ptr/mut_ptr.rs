use core::{fmt::Pointer, marker::PhantomData, mem::MaybeUninit, ptr::Pointee};

use crate::{util::IntoTiny, Ref, RefMut};

use super::{ConstPtr, NonNull};

pub struct MutPtr<T, const BASE_ADDR: usize>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
    ptr: u16,
    metadata: <<T as Pointee>::Metadata as IntoTiny>::Tiny,
    _phantom: PhantomData<*mut T>,
}

impl<T, const BASE_ADDR: usize> MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
    pub fn new(ptr: *mut T) -> Option<Self> {
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
    pub unsafe fn new_unchecked(ptr: *mut T) -> Self {
        let (ptr, metadata) = ptr.to_raw_parts();
        Self {
            ptr: crate::ptr_to_u16_unchecked::<BASE_ADDR>(ptr),
            metadata: metadata.into_tiny_unchecked(),
            _phantom: PhantomData,
        }
    }
    pub fn as_wide_ptr(self) -> *mut T {
        // SAFE: this function can't be called without the ram buffer being initialized
        let ptr = crate::u16_to_ptr::<BASE_ADDR>(self.ptr) as _;
        core::ptr::from_raw_parts_mut(
            ptr,
            <<T as Pointee>::Metadata as IntoTiny>::from_tiny(self.metadata),
        )
    }
    pub fn is_null(self) -> bool {
        self.ptr == 0
    }
    pub fn cast<U>(self) -> MutPtr<U, BASE_ADDR>
    where
        <U as Pointee>::Metadata: IntoTiny,
    {
        // SAFE: We know from the trait bounds and the fact that it comes from a valid pointer that this function is safe
        unsafe { MutPtr::new_unchecked(self.as_wide_ptr().cast()) }
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
    /// Returns a mutable reference to the pointer destination
    ///
    /// # Safety
    /// This function is unsafe because it converts a pointer into a reference.
    pub unsafe fn as_mut<'a>(self) -> Option<RefMut<'a, T, BASE_ADDR>> {
        if self.is_null() {
            None
        } else {
            Some(self.as_mut_unchecked())
        }
    }
    /// Returns a shared reference to the pointer destination without checking for null
    ///
    /// # Safety
    /// This function is unsafe because it converts a pointer into a reference.
    pub unsafe fn as_ref_unchecked<'a>(self) -> Ref<'a, T, BASE_ADDR> {
        Ref::new(NonNull::new_unchecked(self))
    }
    /// Returns a mutable reference to the pointer destination without checking for null
    ///
    /// # Safety
    /// This function is unsafe because it converts a pointer into a reference.
    pub unsafe fn as_mut_unchecked<'a>(self) -> RefMut<'a, T, BASE_ADDR> {
        RefMut::new(NonNull::new_unchecked(self))
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
    /// Returns a mutable reference to a MaybeUninit
    /// # Safety
    /// This function is unsafe because it converts a pointer into a reference.
    pub unsafe fn as_uninit_mut<'a>(self) -> Option<RefMut<'a, MaybeUninit<T>, BASE_ADDR>>
    where
        T: Sized,
    {
        self.cast::<MaybeUninit<T>>().as_mut()
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
    /// Returns a mutable reference to a MaybeUninit, without checking for null
    /// # Safety
    /// This function is unsafe because it converts a pointer into a reference.
    pub unsafe fn as_uninit_mut_unchecked<'a>(self) -> RefMut<'a, MaybeUninit<T>, BASE_ADDR>
    where
        T: Sized,
    {
        self.cast::<MaybeUninit<T>>().as_mut_unchecked()
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

    pub fn set_ptr_value(mut self, val: MutPtr<u8, BASE_ADDR>) -> Self {
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
    /// Writes the pointer
    ///
    /// # Safety
    /// This dereferences a pointer
    pub unsafe fn write(self, val: T)
    where
        T: Sized,
    {
        self.as_wide_ptr().write(val)
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
    /// Writes the pointer
    ///
    /// # Safety
    /// This dereferences a pointer
    pub unsafe fn write_volatile(self, val: T)
    where
        T: Sized,
    {
        self.as_wide_ptr().write_volatile(val)
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
    /// Writes the pointer
    ///
    /// # Safety
    /// This dereferences a pointer
    pub unsafe fn write_unaligned(self, val: T)
    where
        T: Sized,
    {
        self.as_wide_ptr().write_unaligned(val)
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
    /// Copies count elements to this pointer from the src pointer
    ///
    /// # Safety
    /// This dereferences raw pointers
    pub unsafe fn copy_from(self, src: ConstPtr<T, BASE_ADDR>, count: usize)
    where
        T: Sized,
    {
        self.as_wide_ptr().copy_from(src.as_wide_ptr(), count)
    }
    /// Copies count elements to this pointer from the src pointer
    ///
    /// # Safety
    /// This dereferences raw pointers
    pub unsafe fn copy_from_nonoverlapping(self, src: ConstPtr<T, BASE_ADDR>, count: usize)
    where
        T: Sized,
    {
        self.as_wide_ptr()
            .copy_from_nonoverlapping(src.as_wide_ptr(), count)
    }
    /// Drops the value
    ///
    /// # Safety
    /// See `core::ptr::drop_in_place`
    pub unsafe fn drop_in_place(self) {
        self.as_wide_ptr().drop_in_place()
    }
    /// Write bytes to the destination
    ///
    /// # Safety
    /// This dereferences raw pointers
    pub unsafe fn write_bytes(self, val: u8, count: usize)
    where
        T: Sized,
    {
        self.as_wide_ptr().write_bytes(val, count)
    }
    /// Replaces the value at self with a new one
    ///
    /// # Safety
    /// This dereferences raw pointers
    pub unsafe fn replace(self, val: T) -> T
    where
        T: Sized,
    {
        self.as_wide_ptr().replace(val)
    }
    /// Swaps the contents of two pointers
    ///
    /// # Safety
    /// This dereferences raw pointers
    pub unsafe fn swap(self, other: Self)
    where
        T: Sized,
    {
        self.as_wide_ptr().swap(other.as_wide_ptr())
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

impl<T, const BASE_ADDR: usize> Copy for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}

impl<T, const BASE_ADDR: usize> Clone for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const BASE_ADDR: usize> Eq for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
}
impl<T, const BASE_ADDR: usize> PartialEq for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn eq(&self, o: &Self) -> bool {
        self.as_wide_ptr().eq(&o.as_wide_ptr())
    }
}
impl<T, const BASE_ADDR: usize> Ord for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_wide_ptr().cmp(&other.as_wide_ptr())
    }
}
impl<T, const BASE_ADDR: usize> PartialOrd for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, const BASE_ADDR: usize> Pointer for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_wide_ptr().fmt(f)
    }
}

impl<T, const BASE_ADDR: usize> core::fmt::Debug for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.as_wide_ptr(), f)
    }
}

impl<T, const BASE_ADDR: usize> From<ConstPtr<T, BASE_ADDR>> for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn from(v: ConstPtr<T, BASE_ADDR>) -> Self {
        let (ptr, metadata) = v.as_raw_parts();
        Self::from_raw_parts(ptr, metadata)
    }
}

impl<T, const BASE_ADDR: usize> From<RefMut<'_, T, BASE_ADDR>> for MutPtr<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn from(v: RefMut<'_, T, BASE_ADDR>) -> Self {
        v.ptr.as_ptr()
    }
}
