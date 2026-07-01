use core::{
    cell::UnsafeCell,
    ffi::*,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use derives::bitflags_newtype;

#[link(name = "dl")]
unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlerror() -> *const c_char;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

#[derive(Debug, Clone, Copy)]
#[bitflags_newtype]
pub struct OpenFlags(c_int);
impl OpenFlags {
    pub const RTLD_LAZY: Self = Self(0x0001);
}

#[repr(transparent)]
pub struct Dylib(UnsafeCell<c_void>);
unsafe impl Sync for Dylib {}
unsafe impl Send for Dylib {}
impl Dylib {
    #[inline]
    pub fn open(filename: &CStr, flags: OpenFlags) -> Result<OwnedDylib, String> {
        match NonNull::new(unsafe { dlopen(filename.as_ptr(), flags.bits()) } as *mut Dylib) {
            Some(x) => Ok(OwnedDylib(x)),
            None => Err(unsafe { CStr::from_ptr(dlerror()).to_string_lossy().into_owned() }),
        }
    }

    #[inline]
    pub fn sym(&self, name: &CStr) -> Result<NonNull<c_void>, String> {
        match NonNull::new(unsafe { dlsym(self.0.get(), name.as_ptr()) }) {
            Some(x) => Ok(x),
            None => Err(unsafe { CStr::from_ptr(dlerror()).to_string_lossy().into_owned() }),
        }
    }
}

#[repr(transparent)]
pub struct OwnedDylib(NonNull<Dylib>);
unsafe impl Sync for OwnedDylib {}
unsafe impl Send for OwnedDylib {}
impl Drop for OwnedDylib {
    fn drop(&mut self) {
        if unsafe { dlclose(self.0.as_ptr() as *mut c_void) } != 0 {
            tracing::warn!(handle = ?self.0.as_ptr(), reason = ?unsafe { CStr::from_ptr(dlerror()) }, "dlclose failed");
        }
    }
}
impl AsRef<Dylib> for OwnedDylib {
    fn as_ref(&self) -> &Dylib {
        unsafe { self.0.as_ref() }
    }
}
impl Deref for OwnedDylib {
    type Target = Dylib;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl DerefMut for OwnedDylib {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
