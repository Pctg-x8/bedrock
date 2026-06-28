use std::ptr::NonNull;

#[allow(clippy::upper_case_acronyms)]
pub type HANDLE = *mut core::ffi::c_void;
#[allow(clippy::upper_case_acronyms)]
pub type HINSTANCE = HANDLE;
#[allow(clippy::upper_case_acronyms)]
pub type HMODULE = HINSTANCE;

#[allow(clippy::upper_case_acronyms)]
pub type BOOL = core::ffi::c_int;
#[allow(clippy::upper_case_acronyms)]
pub type DWORD = core::ffi::c_ulong;

#[link(name = "kernel32")]
unsafe extern "system" {
    pub unsafe fn LoadLibraryExW(lpLibFileName: *const u16, hFile: HANDLE, dwFlags: DWORD) -> HMODULE;
    pub fn FreeLibrary(hLibModule: HMODULE) -> BOOL;
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: *const core::ffi::c_char) -> *mut core::ffi::c_void;
}

#[repr(transparent)]
pub struct OwnedLibrary(HMODULE);
unsafe impl Sync for OwnedLibrary {}
unsafe impl Send for OwnedLibrary {}
impl Drop for OwnedLibrary {
    #[inline]
    fn drop(&mut self) {
        if unsafe { FreeLibrary(self.0) } == 0 {
            tracing::warn!(handle = ?self.0, reason = ?std::io::Error::last_os_error(), "FreeLibrary failed");
        }
    }
}
impl OwnedLibrary {
    #[inline(always)]
    pub fn open(path: &[u16]) -> std::io::Result<Self> {
        match unsafe { LoadLibraryExW(path.as_ptr(), core::ptr::null_mut(), 0) } {
            r if r.is_null() => Err(std::io::Error::last_os_error()),
            r => Ok(Self(r)),
        }
    }

    #[inline(always)]
    pub fn sym(&self, name: &core::ffi::CStr) -> std::io::Result<NonNull<core::ffi::c_void>> {
        match unsafe { GetProcAddress(self.0, name.as_ptr()) } {
            p if p.is_null() => Err(std::io::Error::last_os_error()),
            p => Ok(unsafe { NonNull::new_unchecked(p) }),
        }
    }
}
