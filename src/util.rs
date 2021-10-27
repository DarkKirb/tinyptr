pub trait IntoTiny {
    type Tiny: Copy;
    /// Converts the type into a smaller version, without range-checking.
    ///
    /// # Safety
    /// The caller has to ensure that the target type can fit the current object.
    unsafe fn into_tiny_unchecked(self) -> Self::Tiny;
    fn into_tiny(self) -> Option<Self::Tiny>;
    fn from_tiny(t: Self::Tiny) -> Self;
}

impl IntoTiny for () {
    type Tiny = ();

    unsafe fn into_tiny_unchecked(self) -> Self::Tiny {
        self
    }

    fn into_tiny(self) -> Option<Self::Tiny> {
        Some(self)
    }

    fn from_tiny(t: Self::Tiny) -> Self {
        t
    }
}

impl IntoTiny for usize {
    type Tiny = crate::TinyUSize;

    unsafe fn into_tiny_unchecked(self) -> Self::Tiny {
        self as Self::Tiny
    }

    fn into_tiny(self) -> Option<Self::Tiny> {
        self.try_into().ok()
    }

    fn from_tiny(t: Self::Tiny) -> Self {
        t as Self
    }
}

impl<T: ?Sized> IntoTiny for core::ptr::DynMetadata<T> {
    type Tiny = Self;

    unsafe fn into_tiny_unchecked(self) -> Self::Tiny {
        self
    }

    fn into_tiny(self) -> Option<Self::Tiny> {
        Some(self)
    }

    fn from_tiny(t: Self::Tiny) -> Self {
        t
    }
}
