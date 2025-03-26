//! Extension Helper

use std::iter::FusedIterator;

pub trait Chainable<'d, T> {
    fn chain(&mut self, next: &'d T) -> &mut Self;
}

pub trait StructureChainQuery {
    /// Iterate pNext chain
    fn iter_chain(&self) -> StructureChainIterator;

    fn query_structure_type(&self, ty: crate::vk::VkStructureType) -> Option<&GenericVulkanStructure> {
        self.iter_chain().find(|s| s.sType == ty)
    }
    fn query_structure<S: VulkanStructure>(&self) -> Option<&S> {
        self.query_structure_type(S::TYPE)
            .map(|r| unsafe { r.cast_unchecked() })
    }
}
impl<S: VulkanStructure> StructureChainQuery for S {
    #[inline(always)]
    fn iter_chain(&self) -> StructureChainIterator {
        StructureChainIterator {
            current: self.as_generic() as _,
            marker: std::marker::PhantomData,
        }
    }
}

pub trait SinkStructureChainQuery {
    /// Iterate pNext chain
    fn iter_chain(&self) -> SinkStructureChainIterator;

    #[inline(always)]
    fn query_structure_type(&self, ty: crate::vk::VkStructureType) -> Option<&GenericVulkanSinkStructure> {
        self.iter_chain().find(|s| s.sType == ty)
    }

    #[inline(always)]
    fn query_structure<S: VulkanSinkStructure>(&self) -> Option<&S> {
        self.query_structure_type(S::TYPE)
            .map(|r| unsafe { r.cast_ref_unchecked() })
    }
}
impl<S: VulkanSinkStructure> SinkStructureChainQuery for S {
    #[inline(always)]
    fn iter_chain(&self) -> SinkStructureChainIterator {
        SinkStructureChainIterator {
            current: self.as_generic() as _,
            marker: core::marker::PhantomData,
        }
    }
}

pub unsafe trait VulkanStructureAsRef {
    /// Cast structure ref to generic. This is same as transmute but must be safe.
    fn as_generic(&self) -> &GenericVulkanStructure;

    /// Cast structure mutable ref to generic. This is same as transmute but must be safe.
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure;
}
pub unsafe trait VulkanStructure: VulkanStructureAsRef + Sized {
    /// sType of this structure
    const TYPE: crate::vk::VkStructureType;

    /// Cast structure ref only if sType matches
    fn try_from_generic(g: &GenericVulkanStructure) -> Option<&Self> {
        if g.sType == Self::TYPE {
            Some(unsafe { g.cast_unchecked() })
        } else {
            None
        }
    }
}
unsafe impl<S: VulkanStructureAsRef + ?Sized> VulkanStructureAsRef for &'_ mut S {
    fn as_generic(&self) -> &GenericVulkanStructure {
        S::as_generic(*self)
    }

    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        S::as_generic_mut(*self)
    }
}
unsafe impl<S: VulkanStructure + ?Sized> VulkanStructure for &'_ mut S {
    const TYPE: crate::vk::VkStructureType = S::TYPE;
}
unsafe impl<S: VulkanStructureAsRef + ?Sized> VulkanStructureAsRef for Box<S> {
    fn as_generic(&self) -> &GenericVulkanStructure {
        S::as_generic(&**self)
    }

    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        S::as_generic_mut(&mut **self)
    }
}
unsafe impl<S: VulkanStructure + ?Sized> VulkanStructure for Box<S> {
    const TYPE: crate::vk::VkStructureType = S::TYPE;
}

pub unsafe trait VulkanSinkStructureAsRef {
    /// Cast this structure ref to generic one. This is same as transmute but must be safe.
    fn as_generic(&self) -> &GenericVulkanSinkStructure;

    /// Cast this structure mutable ref to generic one. This is same as transmute but must be safe.
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure;
}
pub unsafe trait VulkanSinkStructure: VulkanSinkStructureAsRef + Sized {
    /// `sType` constant for this structure.
    const TYPE: crate::vk::VkStructureType;

    /// Cast structure ref only if sType matches
    fn try_from_generic(g: &GenericVulkanSinkStructure) -> Option<&Self> {
        if g.sType == Self::TYPE {
            Some(unsafe { g.cast_ref_unchecked() })
        } else {
            None
        }
    }
}
unsafe impl<T> VulkanSinkStructureAsRef for &'_ mut T
where
    T: VulkanSinkStructureAsRef,
{
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanSinkStructure {
        T::as_generic(*self)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure {
        T::as_generic_mut(*self)
    }
}
unsafe impl<T> VulkanSinkStructure for &'_ mut T
where
    T: VulkanSinkStructure,
{
    const TYPE: crate::vk::VkStructureType = T::TYPE;
}
unsafe impl<T> VulkanSinkStructureAsRef for Box<T>
where
    T: VulkanSinkStructureAsRef + ?Sized,
{
    #[inline(always)]
    fn as_generic(&self) -> &GenericVulkanSinkStructure {
        T::as_generic(&*self)
    }

    #[inline(always)]
    fn as_generic_mut(&mut self) -> &mut GenericVulkanSinkStructure {
        T::as_generic_mut(&mut *self)
    }
}
unsafe impl<T> VulkanSinkStructure for Box<T>
where
    T: VulkanSinkStructure,
{
    const TYPE: crate::vk::VkStructureType = T::TYPE;
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GenericVulkanStructure {
    pub sType: crate::vk::VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub _rest: [u8; 0],
}
impl GenericVulkanStructure {
    pub const unsafe fn cast_unchecked<T>(&self) -> &T {
        unsafe { core::mem::transmute(self) }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GenericVulkanSinkStructure {
    pub sType: crate::vk::VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    _rest: [u8; 0],
}
impl GenericVulkanSinkStructure {
    pub const unsafe fn cast_ref_unchecked<T>(&self) -> &T {
        unsafe { core::mem::transmute(self) }
    }

    pub const unsafe fn cast_mut_unchecked<T>(&mut self) -> &mut T {
        unsafe { core::mem::transmute(self) }
    }
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
        let Some(r) = (unsafe { self.current.as_ref() }) else {
            return None;
        };

        self.current = r.pNext as _;
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
    T: VulkanStructure,
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

pub unsafe trait UninitVulkanStructureOps {
    fn set_next(&mut self, next: &(impl VulkanStructure + ?Sized));
}
unsafe impl<T> UninitVulkanStructureOps for core::mem::MaybeUninit<T>
where
    T: VulkanStructure,
{
    #[inline(always)]
    fn set_next(&mut self, next: &(impl VulkanStructure + ?Sized)) {
        let p = self.as_mut_ptr() as *mut GenericVulkanStructure;
        unsafe {
            core::ptr::addr_of_mut!((*p).pNext).write(next.as_generic() as *const _ as _);
        }
    }
}
