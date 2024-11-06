//! Vulkan Function Resolver

#![allow(non_snake_case)]

use crate::vk::*;
use cfg_if::cfg_if;
use derives::implements;
#[cfg(feature = "DynamicLoaded")]
use libloading::*;

use core::ffi::*;

#[cfg(feature = "Implements")]
cfg_if! {
    if #[cfg(feature = "CustomResolver")] {
        static GLOBAL_RESOLVER: std::sync::OnceLock<Box<dyn ResolverInterface>> = std::sync::OnceLock::new();

        pub fn set_custom_resolver(resolver: Box<dyn ResolverInterface>) {
            crate::vkfn::FunctionPointerTable::reset();
            GLOBAL_RESOLVER.set(Box::into_raw(resolver))
        }
        pub fn get_resolver() -> &'static dyn ResolverInterface {
            GLOBAL_RESOLVER.get().expect("no global resolver set")
        }
    } else if #[cfg(feature = "DynamicLoaded")] {
        static GLOBAL_RESOLVER: std::sync::LazyLock<Box<Resolver>> = std::sync::LazyLock::new(|| Box::new(Resolver::new()));

        pub struct Resolver(Library);
        impl Resolver {
            fn new() -> Self {
                cfg_if! {
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

            unsafe fn load_function_unconstrainted<F: PFN>(&self, name: &core::ffi::CStr) -> F {
                F::from_ptr(self.0.get::<F>(name.to_bytes_with_nul()).unwrap().into_raw().into_raw() as _)
            }
        }

        #[inline(always)]
        pub fn get_resolver<'a>() -> &'a Resolver {
            &*GLOBAL_RESOLVER
        }
    }
}

#[implements]
pub trait ResolverInterface {
    unsafe fn load_symbol_unconstrainted<T: FromPtr>(&self, name: &core::ffi::CStr) -> T;
    unsafe fn load_function_unconstrainted<F: PFN>(&self, name: &core::ffi::CStr) -> F;
}

#[cfg(feature = "Implements")]
cfg_if! {
    if #[cfg(feature = "DynamicLoaded")] {
        pub struct DefaultGlobalResolver;
        impl ResolverInterface for DefaultGlobalResolver {
            unsafe fn load_symbol_unconstrainted<T: FromPtr>(&self, name: &core::ffi::CStr) -> T {
                get_resolver().load_symbol_unconstrainted(name)
            }

            unsafe fn load_function_unconstrainted<F: PFN>(&self, name: &core::ffi::CStr) -> F {
                get_resolver().load_function_unconstrainted(name)
            }
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

#[cfg(feature = "Implements")]
pub struct ResolvedFnCell<F: PFN, R>(R, std::sync::OnceLock<F>);
#[cfg(feature = "Implements")]
impl<F: PFN, R: ResolverInterface> ResolvedFnCell<F, R> {
    pub const fn new(resolver: R) -> Self {
        Self(resolver, std::sync::OnceLock::new())
    }

    pub fn resolve(&self) -> &F {
        self.1
            .get_or_init(|| unsafe { self.0.load_function_unconstrainted::<F>(F::NAME_CSTR) })
    }
}
