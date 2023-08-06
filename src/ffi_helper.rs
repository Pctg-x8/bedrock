#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedCStrBuffer<const L: usize>([u8; L]);
impl<const L: usize> FixedCStrBuffer<L> {
    pub const unsafe fn as_cstr_unchecked(&self) -> &std::ffi::CStr {
        std::ffi::CStr::from_bytes_with_nul_unchecked(&self.0)
    }
}
