#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedCStrBuffer<const L: usize>([u8; L]);
impl<const L: usize> FixedCStrBuffer<L> {
    pub const fn as_cstr(&self) -> Result<&std::ffi::CStr, core::ffi::FromBytesUntilNulError> {
        core::ffi::CStr::from_bytes_until_nul(&self.0)
    }
}
