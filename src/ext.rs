//! Extension Helper

use std::iter::FusedIterator;

#[repr(C)]
#[allow(non_snake_case)]
pub struct GenericVulkanStructure {
    pub sType: crate::vk::VkStructureType,
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
    const TYPE: crate::vk::VkStructureType;

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
    const TYPE: crate::vk::VkStructureType = S::TYPE;
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
    const TYPE: crate::vk::VkStructureType = S::TYPE;
}

#[inline(always)]
pub const fn uninit_sink<T: TypedVulkanSinkStructure>() -> core::mem::MaybeUninit<T> {
    let mut p = core::mem::MaybeUninit::<T>::uninit();
    unsafe {
        core::ptr::addr_of_mut!((*p.as_mut_ptr().cast::<GenericVulkanSinkStructure>()).sType).write(T::TYPE);
        core::ptr::addr_of_mut!((*p.as_mut_ptr().cast::<GenericVulkanSinkStructure>()).pNext)
            .write(core::ptr::null_mut());
    }

    p
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GenericVulkanSinkStructure {
    pub sType: crate::vk::VkStructureType,
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
    const TYPE: crate::vk::VkStructureType;

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
    const TYPE: crate::vk::VkStructureType = T::TYPE;
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
    const TYPE: crate::vk::VkStructureType = T::TYPE;
}

pub struct StructureChainIterator<'a> {
    pub(crate) current: *const GenericVulkanStructure,
    pub(crate) marker: std::marker::PhantomData<&'a GenericVulkanStructure>,
}
impl<'a> Iterator for StructureChainIterator<'a> {
    type Item = &'a GenericVulkanStructure;
    fn next(&mut self) -> Option<&'a GenericVulkanStructure> {
        if let Some(r) = unsafe { self.current.as_ref() } {
            self.current = r.pNext as _;
            unsafe { self.current.as_ref() }
        } else {
            None
        }
    }
}
impl FusedIterator for StructureChainIterator<'_> {}

pub struct SinkStructureChainIterator<'a> {
    pub(crate) current: *const GenericVulkanSinkStructure,
    pub(crate) marker: core::marker::PhantomData<&'a GenericVulkanSinkStructure>,
}
impl<'a> Iterator for SinkStructureChainIterator<'a> {
    type Item = &'a GenericVulkanSinkStructure;

    fn next(&mut self) -> Option<&'a GenericVulkanSinkStructure> {
        self.current = unsafe { self.current.as_ref() }?.pNext.cast();
        unsafe { self.current.as_ref() }
    }
}
impl FusedIterator for SinkStructureChainIterator<'_> {}

pub trait VulkanStructureProvider {
    type RootStructure;

    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut Self::RootStructure) -> &'r mut GenericVulkanStructure;
}
pub struct Extends<Parent: VulkanStructureProvider, T>(pub(crate) Parent, pub(crate) T);
impl<Parent: VulkanStructureProvider, T> VulkanStructureProvider for Extends<Parent, T>
where
    T: TypedVulkanStructure,
{
    type RootStructure = Parent::RootStructure;

    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut Self::RootStructure) -> &'r mut GenericVulkanStructure {
        let parent = self.0.build(root);
        parent.pNext = &self.1 as *const _ as _;
        self.1.as_generic_mut()
    }
}
pub trait Extendable<T>: Sized + VulkanStructureProvider {
    #[inline]
    fn extends(self, next: T) -> Extends<Self, T> {
        Extends(self, next)
    }
}

// pub unsafe trait UninitVulkanStructureOps {
//     fn set_next(&mut self, next: &(impl TypedVulkanStructure + ?Sized));
// }
// unsafe impl<T> UninitVulkanStructureOps for core::mem::MaybeUninit<T>
// where
//     T: TypedVulkanStructure,
// {
//     #[inline(always)]
//     fn set_next(&mut self, next: &(impl TypedVulkanStructure + ?Sized)) {
//         let p = self.as_mut_ptr() as *mut GenericVulkanStructure;
//         unsafe {
//             core::ptr::addr_of_mut!((*p).pNext).write(next.as_generic() as *const _ as _);
//         }
//     }
// }

pub trait Chainable<'d, T> {
    fn chain(&mut self, next: &'d T) -> &mut Self;
}

pub trait StructureChainQuery {
    /// Iterate pNext chain
    fn iter_chain<'a>(&'a self) -> StructureChainIterator<'a>;

    fn query_structure_type(&self, ty: crate::vk::VkStructureType) -> Option<&GenericVulkanStructure> {
        self.iter_chain().find(|s| s.sType == ty)
    }
    fn query_structure<S: TypedVulkanStructure>(&self) -> Option<&S> {
        self.query_structure_type(S::TYPE)
            .map(|r| unsafe { r.cast_unchecked() })
    }
}
impl<S: TypedVulkanStructure> StructureChainQuery for S {
    #[inline(always)]
    fn iter_chain<'a>(&'a self) -> StructureChainIterator<'a> {
        StructureChainIterator {
            current: self.as_generic() as _,
            marker: std::marker::PhantomData,
        }
    }
}

pub trait SinkStructureChainQuery {
    /// Iterate pNext chain
    fn iter_chain<'a>(&'a self) -> SinkStructureChainIterator<'a>;

    #[inline(always)]
    fn query_structure_type(&self, ty: crate::vk::VkStructureType) -> Option<&GenericVulkanSinkStructure> {
        self.iter_chain().find(|s| s.sType == ty)
    }

    #[inline(always)]
    fn query_structure<S: TypedVulkanSinkStructure>(&self) -> Option<&S> {
        self.query_structure_type(S::TYPE)
            .map(|r| unsafe { r.cast_ref_unchecked() })
    }
}
impl<S: TypedVulkanSinkStructure> SinkStructureChainQuery for S {
    #[inline(always)]
    fn iter_chain<'a>(&'a self) -> SinkStructureChainIterator<'a> {
        SinkStructureChainIterator {
            current: self.as_generic() as _,
            marker: core::marker::PhantomData,
        }
    }
}

/// chains a list of vulkan structures
pub fn chain_structures<'x>(mut xs: impl Iterator<Item = &'x mut GenericVulkanStructure>) {
    let Some(mut p) = xs.next() else {
        // nothing to be chained
        return;
    };

    for q in xs {
        p.pNext = core::ptr::from_mut(q).cast();
        p = q;
    }
}
