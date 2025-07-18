//! Vulkan Function Resolver

#![allow(non_snake_case)]

use crate::*;
use derives::implements;
#[cfg(feature = "DynamicLoaded")]
use libloading::*;

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

        pub struct Resolver(Library);
        impl Resolver {
            fn new() -> Self {
                cfg_if::cfg_if! {
                    if #[cfg(target_os = "macos")] {
                        fn libname() -> &'static str {
                            // let mut exepath = std::env::current_exe().unwrap();
                            // exepath.pop();
                            // exepath.push("libvulkan.dylib");
                            // return exepath;
                            "libvulkan.dylib"
                        }
                    } else if #[cfg(windows)] {
                        fn libname() -> &'static str {
                            "vulkan-1.dll"
                        }
                    } else {
                        // assumes unix environment
                        fn libname() -> &'static str {
                            "libvulkan.so"
                        }
                    }
                }

                Library::new(&libname())
                    .map(Self)
                    .expect(&format!("Unable to open libvulkan: {:?}", libname()))
            }
        }
        impl ResolverInterface for Resolver {
            unsafe fn load_symbol_unconstrainted<T: FromPtr>(&self, name: &core::ffi::CStr) -> T {
                T::from_ptr(self.0.get::<T>(name.to_bytes_with_nul()).unwrap().into_raw().into_raw() as _)
            }

            unsafe fn load_function_unconstrainted<F: PFN>(&self) -> F {
                F::from_ptr(self.0.get::<F>(F::NAME_CSTR.to_bytes_with_nul()).unwrap().into_raw().into_raw() as _)
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
