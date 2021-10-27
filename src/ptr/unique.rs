use core::{marker::PhantomData, ptr::Pointee};

use crate::{util::IntoTiny, Ref, RefMut};

use super::{MutPtr, NonNull};

/// A wrapper around a raw non-null `*mut T` that indicates that the possessor
/// of this wrapper owns the referent. Useful for building abstractions like
/// `Box<T>`, `Vec<T>`, `String`, and `HashMap<K, V>`.
///
/// Unlike `*mut T`, `Unique<T>` behaves "as if" it were an instance of `T`.
/// It implements `Send`/`Sync` if `T` is `Send`/`Sync`. It also implies
/// the kind of strong aliasing guarantees an instance of `T` can expect:
/// the referent of the pointer should not be modified without a unique path to
/// its owning Unique.
///
/// If you're uncertain of whether it's correct to use `Unique` for your purposes,
/// consider using `NonNull`, which has weaker semantics.
///
/// Unlike `*mut T`, the pointer must always be non-null, even if the pointer
/// is never dereferenced. This is so that enums may use this forbidden value
/// as a discriminant -- `Option<Unique<T>>` has the same size as `Unique<T>`.
/// However the pointer may still dangle if it isn't dereferenced.
///
/// Unlike `*mut T`, `Unique<T>` is covariant over `T`. This should always be correct
/// for any type which upholds Unique's aliasing requirements.
#[repr(transparent)]
pub struct Unique<T, const BASE_ADDR: usize>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
    ptr: NonNull<T, BASE_ADDR>,
    _phantom: PhantomData<T>,
}

/// `Unique` pointers are `Send` if `T` is `Send` because the data they
/// reference is unaliased. Note that this aliasing invariant is
/// unenforced by the type system; the abstraction using the
/// `Unique` must enforce it.
unsafe impl<T, const BASE_ADDR: usize> Send for Unique<T, BASE_ADDR>
where
    T: Send + ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
}

/// `Unique` pointers are `Sync` if `T` is `Sync` because the data they
/// reference is unaliased. Note that this aliasing invariant is
/// unenforced by the type system; the abstraction using the
/// `Unique` must enforce it.
unsafe impl<T, const BASE_ADDR: usize> Sync for Unique<T, BASE_ADDR>
where
    T: Sync + ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
}

impl<T, const BASE_ADDR: usize> Unique<T, BASE_ADDR>
where
    <T as Pointee>::Metadata: IntoTiny,
{
    /// Creates a new `Unique` that is dangling, but well-aligned.
    ///
    /// This is useful for initializing types which lazily allocate, like
    /// `Vec::new` does.
    ///
    /// Note that the pointer value may potentially represent a valid pointer to
    /// a `T`, which means this must not be used as a "not yet initialized"
    /// sentinel value. Types that lazily allocate must track initialization by
    /// some other means.
    #[inline]
    pub fn dangling() -> Self {
        Self {
            ptr: NonNull::dangling(),
            _phantom: PhantomData,
        }
    }
}

impl<T, const BASE_ADDR: usize> Unique<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
    pub const fn from_nonnull(nn: NonNull<T, BASE_ADDR>) -> Self {
        Self {
            ptr: nn,
            _phantom: PhantomData,
        }
    }
    /// Creates a new `Unique`.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    pub unsafe fn new_unchecked(ptr: MutPtr<T, BASE_ADDR>) -> Self {
        Self::from_nonnull(NonNull::new_unchecked(ptr))
    }
    /// Creates a new `Unique` if `ptr` is non-null.
    #[inline]
    pub fn new(ptr: MutPtr<T, BASE_ADDR>) -> Option<Self> {
        Some(Self::from_nonnull(NonNull::new(ptr)?))
    }
    /// Acquires the underlying `*mut` pointer.
    #[inline]
    pub fn as_ptr(self) -> MutPtr<T, BASE_ADDR> {
        self.ptr.as_ptr()
    }

    /// Dereferences the content.
    ///
    /// The resulting lifetime is bound to self so this behaves "as if"
    /// it were actually an instance of T that is getting borrowed. If a longer
    /// (unbound) lifetime is needed, use `&*my_ptr.as_ptr()`.
    /// # Safety
    /// the caller must guarantee that `self` meets all the requirements for a reference.
    #[inline]
    pub unsafe fn as_ref(&self) -> Ref<'_, T, BASE_ADDR> {
        self.ptr.as_ref()
    }

    /// Mutably dereferences the content.
    ///
    /// The resulting lifetime is bound to self so this behaves "as if"
    /// it were actually an instance of T that is getting borrowed. If a longer
    /// (unbound) lifetime is needed, use `&mut *my_ptr.as_ptr()`.
    ///
    /// # Safety
    /// the caller must guarantee that `self` meets all the requirements for a mutable reference.
    #[inline]
    pub unsafe fn as_mut(&mut self) -> RefMut<'_, T, BASE_ADDR> {
        self.ptr.as_mut()
    }

    /// Casts to a pointer of another type.
    #[inline]
    pub fn cast<U>(self) -> Unique<U, BASE_ADDR>
    where
        <U as Pointee>::Metadata: IntoTiny,
    {
        Unique::from_nonnull(self.ptr.cast())
    }
}

impl<T, const BASE_ADDR: usize> Clone for Unique<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<T, const BASE_ADDR: usize> Copy for Unique<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny,
{
}

impl<T, const BASE_ADDR: usize> core::fmt::Pointer for Unique<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.ptr.fmt(f)
    }
}

impl<T, const BASE_ADDR: usize> core::fmt::Debug for Unique<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.ptr, f)
    }
}

impl<T, const BASE_ADDR: usize> From<NonNull<T, BASE_ADDR>> for Unique<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn from(nn: NonNull<T, BASE_ADDR>) -> Self {
        Self::from_nonnull(nn)
    }
}

impl<T, const BASE_ADDR: usize> From<RefMut<'_, T, BASE_ADDR>> for Unique<T, BASE_ADDR>
where
    T: ?Sized,
    <T as Pointee>::Metadata: IntoTiny + Copy,
{
    fn from(v: RefMut<'_, T, BASE_ADDR>) -> Self {
        v.ptr.into()
    }
}
