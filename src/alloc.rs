//! rust std allocation functionalities wrapper

#[inline(always)]
pub(crate) const fn empty_sink_buffer<T>() -> Vec<T> {
    Vec::new()
}

#[inline(always)]
pub(crate) fn collect_vec<T>(iter: impl IntoIterator<Item = T>) -> Vec<T> {
    Vec::from_iter(iter)
}

#[inline(always)]
pub(crate) fn unzip_vec<A, B>(iter: impl IntoIterator<Item = (A, B)>) -> (Vec<A>, Vec<B>) {
    iter.into_iter().unzip()
}

#[inline(always)]
pub(crate) fn str_to_cstr(s: &str) -> core::result::Result<std::ffi::CString, std::ffi::NulError> {
    std::ffi::CString::new(s)
}
