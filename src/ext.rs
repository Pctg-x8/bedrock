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
pub unsafe trait VulkanStructure: Sized {
    /// sType of this structure
    const TYPE: crate::vk::VkStructureType;

    /// Cast structure ref to generic. This is same as transmute but must be safe.
    fn as_generic(&self) -> &GenericVulkanStructure {
        unsafe { std::mem::transmute(self) }
    }
    /// Cast structure mutable ref to generic. This is same as transmute but must be safe.
    fn as_generic_mut(&mut self) -> &mut GenericVulkanStructure {
        unsafe { std::mem::transmute(self) }
    }

    /// Cast structure ref only if sType matches
    fn try_from_generic(g: &GenericVulkanStructure) -> Option<&Self> {
        if g.sType == Self::TYPE {
            Some(unsafe { g.cast_unchecked() })
        } else {
            None
        }
    }
}
impl<S: VulkanStructure> StructureChainQuery for S {
    fn iter_chain(&self) -> StructureChainIterator {
        StructureChainIterator {
            current: self.as_generic() as _,
            marker: std::marker::PhantomData,
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GenericVulkanStructure {
    pub sType: crate::vk::VkStructureType,
    pub pNext: *const libc::c_void,
}
impl GenericVulkanStructure {
    pub unsafe fn cast_unchecked<T>(&self) -> &T {
        std::mem::transmute(self)
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
