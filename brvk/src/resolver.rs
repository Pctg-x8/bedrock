//! Vulkan Function Resolver

use core::ffi::*;
use std::ptr::NonNull;

pub trait ResolverInterface {
    /// Loads a symbol using the resolver, without any constraints on the symbol's type.
    ///
    /// # Safety
    ///
    /// retrieved symbol must be valid value of type T.
    unsafe fn load_symbol_unconstrainted(&self, name: &CStr) -> NonNull<core::ffi::c_void>;

    /// Loads a function using the resolver, without any constraints on the function's type.
    ///
    /// # Safety
    ///
    /// retrieved function must be valid function pointer of type F.
    unsafe fn load_function_unconstrainted(&self, name: &CStr) -> crate::PFN_vkVoidFunction;
}
impl ResolverInterface for Box<dyn ResolverInterface> {
    #[inline(always)]
    unsafe fn load_symbol_unconstrainted(&self, name: &CStr) -> NonNull<core::ffi::c_void> {
        unsafe { self.as_ref().load_symbol_unconstrainted(name) }
    }

    #[inline(always)]
    unsafe fn load_function_unconstrainted(&self, name: &CStr) -> crate::PFN_vkVoidFunction {
        unsafe { self.as_ref().load_function_unconstrainted(name) }
    }
}

#[repr(transparent)]
pub struct ResolverWrapper<'a, I: ResolverInterface>(&'a I);
impl<'a, I: ResolverInterface> ResolverWrapper<'a, I> {
    /// Loads a symbol using the resolver, without any constraints on the symbol's type.
    ///
    /// # Safety
    ///
    /// retrieved symbol must be valid value of type T.
    pub unsafe fn load_symbol_unconstrainted<T: crate::FromPtr>(&self, name: &CStr) -> T {
        unsafe { T::from_ptr(self.0.load_symbol_unconstrainted(name).as_ptr()) }
    }

    /// Loads a function using the resolver, without any constraints on the function's type.
    ///
    /// # Safety
    ///
    /// retrieved function must be valid function pointer of type F.
    pub unsafe fn load_function_unconstrainted<F: crate::PFN>(&self) -> F {
        unsafe { F::from_void_fn(self.0.load_function_unconstrainted(F::NAME_CSTR)) }
    }
}

#[cfg(feature = "DynamicLoaded")]
#[inline(always)]
pub(crate) fn current_resolver<'a>() -> ResolverWrapper<'a, impl ResolverInterface> {
    #[cfg(feature = "CustomResolver")]
    #[allow(clippy::deref_addrof)]
    unsafe {
        ResolverWrapper((*&raw const GLOBAL_RESOLVER).as_ref().expect("no global resolver set"))
    }
    #[cfg(not(feature = "CustomResolver"))]
    {
        GLOBAL_RESOLVER_INIT.call_once(|| unsafe {
            DefaultResolver::init_inplace(&raw mut GLOBAL_RESOLVER);
        });
        #[allow(clippy::deref_addrof)]
        ResolverWrapper(unsafe { (*&raw const GLOBAL_RESOLVER).assume_init_ref() })
    }
}

#[cfg(feature = "DynamicLoaded")]
static GLOBAL_RESOLVER_INIT: parking_lot::Once = parking_lot::Once::new();
#[cfg(feature = "CustomResolver")]
static mut GLOBAL_RESOLVER: Option<Box<dyn ResolverInterface>> = None;
#[cfg(feature = "CustomResolver")]
pub fn set_resolver(resolver: Box<dyn ResolverInterface>) {
    crate::fns::FunctionPointerTable::reset();
    GLOBAL_RESOLVER_INIT.call_once(|| {});
    #[allow(clippy::deref_addrof)]
    unsafe {
        *&raw mut GLOBAL_RESOLVER = Some(resolver);
    }
}

#[cfg(all(feature = "DynamicLoaded", not(feature = "CustomResolver"), not(windows)))]
mod libdl;
#[cfg(all(feature = "DynamicLoaded", not(feature = "CustomResolver"), windows))]
mod libloaderapi;

#[cfg(all(feature = "DynamicLoaded", not(feature = "CustomResolver")))]
static mut GLOBAL_RESOLVER: core::mem::MaybeUninit<DefaultResolver> = core::mem::MaybeUninit::uninit();

#[cfg(all(feature = "DynamicLoaded", not(feature = "CustomResolver")))]
pub struct DefaultResolver(
    #[cfg(windows)] self::libloaderapi::OwnedLibrary,
    #[cfg(not(windows))] self::libdl::OwnedLibrary,
);
#[cfg(all(feature = "DynamicLoaded", not(feature = "CustomResolver")))]
impl ResolverInterface for DefaultResolver {
    unsafe fn load_symbol_unconstrainted(&self, name: &core::ffi::CStr) -> NonNull<core::ffi::c_void> {
        match self.0.sym(name) {
            Ok(x) => x,
            Err(e) => {
                tracing::error!(?name, reason = %e, "could not resolve symbol");
                std::process::abort();
            }
        }
    }

    unsafe fn load_function_unconstrainted(&self, name: &core::ffi::CStr) -> crate::PFN_vkVoidFunction {
        match self.0.sym(name) {
            Ok(x) => unsafe { core::mem::transmute::<*const core::ffi::c_void, crate::PFN_vkVoidFunction>(x.as_ptr()) },
            Err(e) => {
                tracing::error!(?name, reason = %e, "could not resolve function symbol");
                std::process::abort();
            }
        }
    }
}
#[cfg(all(feature = "DynamicLoaded", not(feature = "CustomResolver")))]
impl DefaultResolver {
    #[cfg(windows)]
    unsafe fn init_inplace(sink: *mut core::mem::MaybeUninit<Self>) {
        // "vulkan-1.dll\0" in utf-16
        const LIBNAME: &[u16] = &[
            b'v' as _, b'u' as _, b'l' as _, b'k' as _, b'a' as _, b'n' as _, b'-' as _, b'1' as _, b'.' as _,
            b'd' as _, b'l' as _, b'l' as _, 0,
        ];

        let lib = match self::libloaderapi::OwnedLibrary::open(LIBNAME) {
            Ok(x) => x,
            Err(e) => {
                tracing::error!(
                    reason = ?e,
                    libpath = ?LIBNAME,
                    "Failed to open libvulkan, bedrock could not continue"
                );
                std::process::abort();
            }
        };

        unsafe {
            core::ptr::write(core::ptr::addr_of_mut!((*(*sink).as_mut_ptr()).0), lib);
        }
    }

    #[cfg(not(windows))]
    fn new() -> Self {
        #[cfg(target_os = "macos")]
        fn libname() -> &'static core::ffi::CStr {
            // TODO: packed app
            // let mut exepath = std::env::current_exe().unwrap();
            // exepath.pop();
            // exepath.push("libvulkan.dylib");
            // return exepath;
            c"libvulkan.dylib"
        }
        #[cfg(not(any(target_os = "macos", windows)))]
        fn libname() -> &'static core::ffi::CStr {
            // assumes unix environment
            c"libvulkan.so"
        }

        #[cfg(not(windows))]
        match self::libdl::Dylib::open(libname(), crate::libdl::OpenFlags::RTLD_LAZY) {
            Ok(x) => Self(x),
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

pub struct ResolvedFnCell<F: crate::PFN + crate::FromPtr, R>(R, std::sync::OnceLock<F>);
impl<F: crate::PFN + crate::FromPtr, R: ResolverInterface> ResolvedFnCell<F, R> {
    pub const fn new(resolver: R) -> Self {
        Self(resolver, std::sync::OnceLock::new())
    }

    #[inline(always)]
    pub fn resolve(&self) -> &F {
        self.1
            .get_or_init(|| unsafe { ResolverWrapper(&self.0).load_function_unconstrainted() })
    }
}
