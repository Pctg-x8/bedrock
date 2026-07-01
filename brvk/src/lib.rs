mod vk1;
mod vk2;

pub use vk1::*;
pub use vk2::*;

#[cfg(feature = "Implements")]
pub mod fns;
#[cfg(feature = "Implements")]
mod resolver;
#[cfg(feature = "CustomResolver")]
pub use resolver::set_resolver;
#[cfg(feature = "Implements")]
pub use resolver::{ResolvedFnCell, ResolverInterface, load_function_unconstrainted, load_symbol_unconstrainted};

mod result_str;
pub use result_str::*;
pub mod extensions;

// define macros

#[allow(clippy::inconsistent_digit_grouping)]
#[inline]
pub(crate) const fn ext_enum_value(ext_number: u16, index: u16) -> u64 {
    1000_000_000 + ((ext_number - 1) as u64 * 1_000) + index as u64
}

// ffi helper

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedCStrBuffer<const L: usize>(pub [u8; L]);
impl<const L: usize> FixedCStrBuffer<L> {
    pub const fn as_cstr(&self) -> Result<&core::ffi::CStr, core::ffi::FromBytesUntilNulError> {
        core::ffi::CStr::from_bytes_until_nul(&self.0)
    }
}

/// # Safety
///
/// provides a safe wrapper around a raw function pointer, ensuring it is valid.
pub unsafe trait FromPtr {
    /// # Safety
    ///
    /// p must be a valid pointer to a function of type Self.
    unsafe fn from_ptr(p: *const core::ffi::c_void) -> Self;
}

/// # Safety
///
/// provides a safe wrapper around a raw function pointer, ensuring it is valid and callable.
pub unsafe trait PFN {
    const NAME_CSTR: &core::ffi::CStr;

    /// # Safety
    ///
    /// p must be a valid function pointer of type F.
    unsafe fn from_void_fn(p: PFN_vkVoidFunction) -> Self;
}
pub trait StaticCallable: PFN {
    const STATIC: Self;
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GenericVulkanStructure {
    pub sType: crate::VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub _rest: [u8; 0],
}
impl GenericVulkanStructure {
    /// # Safety
    ///
    /// self must be a valid Vulkan structure type of T.
    pub const unsafe fn cast_unchecked<T>(&self) -> &T {
        unsafe { core::mem::transmute(self) }
    }
}

/// Trait for Vulkan structures that can be cast to [`GenericVulkanStructure`] safely.
///
/// # Safety
///
/// self must be a valid Vulkan structure type.
pub unsafe trait VulkanStructure {
    /// Cast structure ref to generic. This is same as transmute but must be safe.
    fn as_generic(&self) -> &GenericVulkanStructure;

    /// Cast structure mutable ref to generic. This is same as transmute but must be safe.
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure;
}
/// Trait for Vulkan structures that have a known [`crate::vk::VkStructureType`] at compile time.
pub trait TypedVulkanStructure: VulkanStructure {
    /// sType of this structure
    const TYPE: crate::VkStructureType;

    /// Cast structure ref only if sType matches
    fn try_from_generic(g: &GenericVulkanStructure) -> Option<&Self>
    where
        Self: Sized,
    {
        if g.sType == Self::TYPE {
            Some(unsafe { g.cast_unchecked() })
        } else {
            None
        }
    }
}
unsafe impl<S: VulkanStructure + ?Sized> VulkanStructure for &'_ mut S {
    fn as_generic(&self) -> &GenericVulkanStructure {
        S::as_generic(*self)
    }

    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        S::as_generic_mut(*self)
    }
}
impl<S: TypedVulkanStructure + ?Sized> TypedVulkanStructure for &'_ mut S {
    const TYPE: crate::VkStructureType = S::TYPE;
}
unsafe impl<S: VulkanStructure + ?Sized> VulkanStructure for Box<S> {
    fn as_generic(&self) -> &GenericVulkanStructure {
        S::as_generic(&**self)
    }

    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        S::as_generic_mut(&mut **self)
    }
}
impl<S: TypedVulkanStructure + ?Sized> TypedVulkanStructure for Box<S> {
    const TYPE: crate::VkStructureType = S::TYPE;
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GenericVulkanSinkStructure {
    pub sType: crate::VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    _rest: [u8; 0],
}
impl GenericVulkanSinkStructure {
    /// # Safety
    ///
    /// self must be a valid Vulkan structure type of T.
    pub const unsafe fn cast_ref_unchecked<T>(&self) -> &T {
        unsafe { core::mem::transmute(self) }
    }

    /// # Safety
    ///
    /// self must be a valid Vulkan structure type of T.
    pub const unsafe fn cast_mut_unchecked<T>(&mut self) -> &mut T {
        unsafe { core::mem::transmute(self) }
    }
}

/// Trait for Vulkan structures that can be cast to [`GenericVulkanSinkStructure`] safely.
///
/// # Safety
///
/// self must be a valid Vulkan structure type.
pub unsafe trait VulkanSinkStructure {
    /// Cast this structure ref to generic one. This is same as transmute but must be safe.
    fn as_generic(&self) -> &GenericVulkanSinkStructure;

    /// Cast this structure mutable ref to generic one. This is same as transmute but must be safe.
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure;
}
/// Trait for Vulkan structures that have a known [`crate::vk::VkStructureType`] at compile time.
pub trait TypedVulkanSinkStructure: VulkanSinkStructure {
    /// `sType` constant for this structure.
    const TYPE: crate::VkStructureType;

    /// Constructs an uninitialized cell for this structure, that is ready to pass the api
    fn uninit_sink() -> core::mem::MaybeUninit<Self>
    where
        Self: Sized,
    {
        let mut p = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            let ptr = p.as_mut_ptr().cast::<GenericVulkanSinkStructure>();
            core::ptr::addr_of_mut!((*ptr).sType).write(Self::TYPE);
            core::ptr::addr_of_mut!((*ptr).pNext).write(core::ptr::null_mut());
        }

        p
    }

    /// Cast structure ref only if sType matches
    fn try_from_generic(g: &GenericVulkanSinkStructure) -> Option<&Self>
    where
        Self: Sized,
    {
        if g.sType == Self::TYPE {
            Some(unsafe { g.cast_ref_unchecked() })
        } else {
            None
        }
    }
}
unsafe impl<T> VulkanSinkStructure for &'_ mut T
where
    T: VulkanSinkStructure,
{
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanSinkStructure {
        T::as_generic(self)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure {
        T::as_generic_mut(self)
    }
}
impl<T> TypedVulkanSinkStructure for &'_ mut T
where
    T: TypedVulkanSinkStructure,
{
    const TYPE: crate::VkStructureType = T::TYPE;
}
unsafe impl<T> VulkanSinkStructure for Box<T>
where
    T: VulkanSinkStructure + ?Sized,
{
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanSinkStructure {
        T::as_generic(self)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure {
        T::as_generic_mut(self)
    }
}
impl<T> TypedVulkanSinkStructure for Box<T>
where
    T: TypedVulkanSinkStructure,
{
    const TYPE: crate::VkStructureType = T::TYPE;
}

pub trait VkRawHandle {
    const OBJECT_TYPE: VkObjectType;

    fn raw_handle_value(&self) -> u64;
}
impl<T: VkRawHandle> VkRawHandle for Option<T> {
    const OBJECT_TYPE: VkObjectType = T::OBJECT_TYPE;

    #[inline(always)]
    fn raw_handle_value(&self) -> u64 {
        match self {
            None => 0,
            Some(x) => x.raw_handle_value(),
        }
    }
}
