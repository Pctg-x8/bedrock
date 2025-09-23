//! Vulkan Function Resolver

#![allow(non_snake_case)]

use crate::*;
use derives::implements;

use core::ffi::*;

#[implements]
pub trait ResolverInterface {
    unsafe fn load_symbol_unconstrainted<T: FromPtr>(&self, name: &core::ffi::CStr) -> T;
    unsafe fn load_function_unconstrainted<F: PFN>(&self) -> F;
}

#[implements]
cfg_if::cfg_if! {
    if #[cfg(feature = "CustomResolver")] {
        static GLOBAL_RESOLVER: parking_lot::RwLock<Option<Box<dyn ResolverInterface>>> = parking_low::RwLock::new(None);

        /// Sets custom resolver object for vulkan api call
        pub fn set_custom_resolver(resolver: Box<dyn ResolverInterface>) {
            crate::vkfn::FunctionPointerTable::reset();
            *GLOBAL_RESOLVER.write() = Some(resolver);
        }

        /// Gets current resolver object
        #[inline(always)]
        pub fn get_resolver<'a>() -> &'a dyn ResolverInterface {
            GLOBAL_RESOLVER.read().expect("no global resolver set")
        }
    } else if #[cfg(feature = "DynamicLoaded")] {
        static GLOBAL_RESOLVER: std::sync::LazyLock<Box<Resolver>> = std::sync::LazyLock::new(|| Box::new(Resolver::new()));

        #[cfg(windows)]
        pub struct Resolver(crate::libloaderapi::OwnedLibrary);
        #[cfg(not(windows))]
        pub struct Resolver(crate::libdl::OwnedDylib);
        impl Resolver {
            fn new() -> Self {
                #[cfg(target_os  ="macos")]
                fn libname() -> &'static core::ffi::CStr {
                    // TODO: packed app
                    // let mut exepath = std::env::current_exe().unwrap();
                    // exepath.pop();
                    // exepath.push("libvulkan.dylib");
                    // return exepath;
                    c"libvulkan.dylib"
                }
                #[cfg(windows)]
                fn libname() -> &'static [u16] {
                    &[
                        b'v' as _,
                        b'u' as _,
                        b'l' as _,
                        b'k' as _,
                        b'a' as _,
                        b'n' as _,
                        b'-' as _,
                        b'1' as _,
                        b'.' as _,
                        b'd' as _,
                        b'l' as _,
                        b'l' as _,
                        0
                    ]
                }
                #[cfg(not(any(target_os = "macos", windows)))]
                fn libname() -> &'static core::ffi::CStr {
                    // assumes unix environment
                    c"libvulkan.so"
                }

                #[cfg(windows)]
                match crate::libloaderapi::OwnedLibrary::open(libname()) {
                    Ok(x) => Resolver(x),
                    Err(e) => {
                        tracing::error!(
                            reason = ?e,
                            libpath = ?libname(),
                            "Failed to open libvulkan, bedrock could not continue"
                        );
                        std::process::abort();
                    }
                }
                #[cfg(not(windows))]
                match crate::libdl::Dylib::open(libname(), crate::libdl::OpenFlags::RTLD_LAZY) {
                    Ok(x) => Resolver(x),
                    Err(e) => {
                        tracing::error!(
                            reason = ?e,
                            libpath = ?libname(),
                            "Failed to open libvulkan, bedrock could not continue"
                        );
                        std::process::abort();
                    }
                }
            }
        }
        impl ResolverInterface for Resolver {
            unsafe fn load_symbol_unconstrainted<T: FromPtr>(&self, name: &core::ffi::CStr) -> T {
                let p = match self.0.sym(name) {
                    Ok(x) => x.as_ptr(),
                    Err(e) => {
                        tracing::warn!(?name, reason = ?e, "could not resolve symbol");
                        core::ptr::null_mut()
                    }
                };

                unsafe { T::from_ptr(p as _) }
            }

            unsafe fn load_function_unconstrainted<F: PFN>(&self) -> F {
                let p = match self.0.sym(F::NAME_CSTR) {
                    Ok(x) => x.as_ptr(),
                    Err(e) => {
                        tracing::warn!(
                            name = ?F::NAME_CSTR,
                            reason = ?e,
                            "could not resolve function symbol"
                        );
                        core::ptr::null_mut()
                    }
                };

                unsafe { F::from_ptr(p as _) }
            }
        }

        #[inline(always)]
        pub fn get_resolver<'a>() -> &'a Resolver {
            &*GLOBAL_RESOLVER
        }
    }
}

pub unsafe trait FromPtr {
    unsafe fn from_ptr(p: *const c_void) -> Self;
}
pub unsafe trait PFN {
    const NAME_CSTR: &'static core::ffi::CStr;

    unsafe fn from_ptr(p: *const c_void) -> Self;
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self;
}
pub trait StaticCallable: PFN {
    const STATIC: Self;
}

#[implements]
pub struct ResolvedFnCell<F: PFN, R>(R, std::sync::OnceLock<F>);
#[implements]
impl<F: PFN, R: ResolverInterface> ResolvedFnCell<F, R> {
    pub const fn new(resolver: R) -> Self {
        Self(resolver, std::sync::OnceLock::new())
    }

    pub fn resolve(&self) -> &F {
        self.1
            .get_or_init(|| unsafe { self.0.load_function_unconstrainted::<F>() })
    }
}
