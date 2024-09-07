#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedCStrBuffer<const L: usize>([u8; L]);
impl<const L: usize> FixedCStrBuffer<L> {
    pub const fn as_cstr(&self) -> Result<&core::ffi::CStr, core::ffi::FromBytesUntilNulError> {
        core::ffi::CStr::from_bytes_until_nul(&self.0)
    }
}

/// pointer of the slice, or null if the slice is empty
#[inline(always)]
pub(crate) const fn slice_as_ptr_empty_null<T>(slice: &[T]) -> *const T {
    if slice.is_empty() {
        core::ptr::null()
    } else {
        slice.as_ptr()
    }
}

pub(crate) trait ArrayFFIExtensions<T> {
    /// pointer of the array, or null if the array is empty
    fn as_ptr_empty_null(&self) -> *const T;

    /// pointer of the array, or null if the array is empty
    fn as_mut_ptr_empty_null(&mut self) -> *mut T;
}
impl<T> ArrayFFIExtensions<T> for Vec<T> {
    #[inline(always)]
    fn as_ptr_empty_null(&self) -> *const T {
        slice_as_ptr_empty_null(self)
    }

    #[inline(always)]
    fn as_mut_ptr_empty_null(&mut self) -> *mut T {
        if self.is_empty() {
            core::ptr::null_mut()
        } else {
            self.as_mut_ptr()
        }
    }
}
impl<T> ArrayFFIExtensions<T> for [T] {
    #[inline(always)]
    fn as_ptr_empty_null(&self) -> *const T {
        slice_as_ptr_empty_null(self)
    }

    #[inline(always)]
    fn as_mut_ptr_empty_null(&mut self) -> *mut T {
        if self.is_empty() {
            core::ptr::null_mut()
        } else {
            self.as_mut_ptr()
        }
    }
}
