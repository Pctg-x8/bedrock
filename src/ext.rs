//! Extension Helper
use bedrock_vk as brvk;

use std::iter::FusedIterator;

#[inline(always)]
pub const fn uninit_sink<T: brvk::TypedVulkanSinkStructure>() -> core::mem::MaybeUninit<T> {
    let mut p = core::mem::MaybeUninit::<T>::uninit();
    unsafe {
        core::ptr::addr_of_mut!((*p.as_mut_ptr().cast::<brvk::GenericVulkanSinkStructure>()).sType).write(T::TYPE);
        core::ptr::addr_of_mut!((*p.as_mut_ptr().cast::<brvk::GenericVulkanSinkStructure>()).pNext)
            .write(core::ptr::null_mut());
    }

    p
}

pub struct StructureChainIterator<'a> {
    pub(crate) current: *const brvk::GenericVulkanStructure,
    pub(crate) marker: std::marker::PhantomData<&'a brvk::GenericVulkanStructure>,
}
impl<'a> Iterator for StructureChainIterator<'a> {
    type Item = &'a brvk::GenericVulkanStructure;
    fn next(&mut self) -> Option<&'a brvk::GenericVulkanStructure> {
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
    pub(crate) current: *const brvk::GenericVulkanSinkStructure,
    pub(crate) marker: core::marker::PhantomData<&'a brvk::GenericVulkanSinkStructure>,
}
impl<'a> Iterator for SinkStructureChainIterator<'a> {
    type Item = &'a brvk::GenericVulkanSinkStructure;

    fn next(&mut self) -> Option<&'a brvk::GenericVulkanSinkStructure> {
        self.current = unsafe { self.current.as_ref() }?.pNext.cast();
        unsafe { self.current.as_ref() }
    }
}
impl FusedIterator for SinkStructureChainIterator<'_> {}

pub trait VulkanStructureProvider {
    type RootStructure;

    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut Self::RootStructure) -> &'r mut brvk::GenericVulkanStructure;
}
pub struct Extends<Parent: VulkanStructureProvider, T>(pub(crate) Parent, pub(crate) T);
impl<Parent: VulkanStructureProvider, T> VulkanStructureProvider for Extends<Parent, T>
where
    T: brvk::TypedVulkanStructure,
{
    type RootStructure = Parent::RootStructure;

    fn build<'r, 's: 'r>(&'s mut self, root: &'s mut Self::RootStructure) -> &'r mut brvk::GenericVulkanStructure {
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

// pub unsafe trait Uninitbrvk::VulkanStructureOps {
//     fn set_next(&mut self, next: &(impl brvk::TypedVulkanStructure + ?Sized));
// }
// unsafe impl<T> Uninitbrvk::VulkanStructureOps for core::mem::MaybeUninit<T>
// where
//     T: brvk::TypedVulkanStructure,
// {
//     #[inline(always)]
//     fn set_next(&mut self, next: &(impl brvk::TypedVulkanStructure + ?Sized)) {
//         let p = self.as_mut_ptr() as *mut brvk::GenericVulkanStructure;
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

    fn query_structure_type(&self, ty: brvk::VkStructureType) -> Option<&brvk::GenericVulkanStructure> {
        self.iter_chain().find(|s| s.sType == ty)
    }
    fn query_structure<S: brvk::TypedVulkanStructure>(&self) -> Option<&S> {
        self.query_structure_type(S::TYPE)
            .map(|r| unsafe { r.cast_unchecked() })
    }
}
impl<S: brvk::TypedVulkanStructure> StructureChainQuery for S {
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
    fn query_structure_type(&self, ty: brvk::VkStructureType) -> Option<&brvk::GenericVulkanSinkStructure> {
        self.iter_chain().find(|s| s.sType == ty)
    }

    #[inline(always)]
    fn query_structure<S: brvk::TypedVulkanSinkStructure>(&self) -> Option<&S> {
        self.query_structure_type(S::TYPE)
            .map(|r| unsafe { r.cast_ref_unchecked() })
    }
}
impl<S: brvk::TypedVulkanSinkStructure> SinkStructureChainQuery for S {
    #[inline(always)]
    fn iter_chain<'a>(&'a self) -> SinkStructureChainIterator<'a> {
        SinkStructureChainIterator {
            current: self.as_generic() as _,
            marker: core::marker::PhantomData,
        }
    }
}

/// chains a list of vulkan structures
pub fn chain_structures<'x>(mut xs: impl Iterator<Item = &'x mut brvk::GenericVulkanStructure>) {
    let Some(mut p) = xs.next() else {
        // nothing to be chained
        return;
    };

    for q in xs {
        p.pNext = core::ptr::from_mut(q).cast();
        p = q;
    }
}
