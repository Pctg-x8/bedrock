#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedCStrBuffer<const L: usize>([u8; L]);
impl<const L: usize> FixedCStrBuffer<L> {
    pub const fn as_cstr(&self) -> Result<&core::ffi::CStr, core::ffi::FromBytesUntilNulError> {
        core::ffi::CStr::from_bytes_until_nul(&self.0)
    }
}

pub(crate) trait ArrayFFIExtensions<T> {
    /// pointer of the array, or null if the array is empty
    fn as_ptr_empty_null(&self) -> *const T;
}
impl<T> ArrayFFIExtensions<T> for Vec<T> {
    fn as_ptr_empty_null(&self) -> *const T {
        if self.is_empty() {
            core::ptr::null()
        } else {
            self.as_ptr()
        }
    }
}
impl<T> ArrayFFIExtensions<T> for [T] {
    fn as_ptr_empty_null(&self) -> *const T {
        if self.is_empty() {
            core::ptr::null()
        } else {
            self.as_ptr()
        }
    }
}
