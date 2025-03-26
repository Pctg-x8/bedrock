use crate::vk::VkObjectType;

/// Wrapping a Vulkan Dispatchable/Nondispatchable Handler
pub trait VkHandle {
    type Handle;

    /// Retrieve an underlying handle
    fn native_ptr(&self) -> Self::Handle;

    #[inline(always)]
    fn as_transparent_ref(&self) -> VkHandleRef<Self::Handle> {
        VkHandleRef::new(self)
    }
}
/// Wrapping a Vulkan Dispatchable/Nondispatchable Mutable Handler
pub trait VkHandleMut: VkHandle {
    /// Retrieve an underlying mutable handle
    fn native_ptr_mut(&mut self) -> Self::Handle;

    #[inline(always)]
    fn as_transparent_ref_mut(&mut self) -> VkHandleRefMut<Self::Handle> {
        VkHandleRefMut::new(self)
    }
}

DerefContainerBracketImpl!(for VkHandle {
    type Handle = T::Handle;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle { T::native_ptr(self) }
});
DerefContainerBracketImpl!(for mut VkHandleMut {
    #[inline(always)]
    fn native_ptr_mut(&mut self) -> Self::Handle { T::native_ptr_mut(self) }
});

impl<T: VkHandle + ?Sized> VkHandle for std::cell::Ref<'_, T> {
    type Handle = T::Handle;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T: VkHandle + ?Sized> VkHandle for std::cell::RefMut<'_, T> {
    type Handle = T::Handle;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T: VkHandleMut + ?Sized> VkHandleMut for std::cell::RefMut<'_, T> {
    #[inline(always)]
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

impl<T: VkHandle + ?Sized> VkHandle for std::sync::MutexGuard<'_, T> {
    type Handle = T::Handle;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T: VkHandleMut + ?Sized> VkHandleMut for std::sync::MutexGuard<'_, T> {
    #[inline(always)]
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

impl<T: VkHandle + ?Sized> VkHandle for std::sync::RwLockReadGuard<'_, T> {
    type Handle = T::Handle;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}

impl<T: VkHandle + ?Sized> VkHandle for std::sync::RwLockWriteGuard<'_, T> {
    type Handle = T::Handle;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T: VkHandleMut + ?Sized> VkHandleMut for std::sync::RwLockWriteGuard<'_, T> {
    #[inline(always)]
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

impl<T: VkHandle + ?Sized> VkHandle for parking_lot::RwLockReadGuard<'_, T> {
    type Handle = T::Handle;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T: VkHandle + ?Sized> VkHandle for parking_lot::RwLockWriteGuard<'_, T> {
    type Handle = T::Handle;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T: VkHandleMut + ?Sized> VkHandleMut for parking_lot::RwLockWriteGuard<'_, T> {
    #[inline(always)]
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

impl<T: VkHandle + ?Sized> VkHandle for parking_lot::MutexGuard<'_, T> {
    type Handle = T::Handle;

    #[inline(always)]
    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T: VkHandleMut + ?Sized> VkHandleMut for parking_lot::MutexGuard<'_, T> {
    #[inline(always)]
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

pub trait VkRawHandle {
    const OBJECT_TYPE: VkObjectType;
    const NULL: Self;

    fn raw_handle_value(&self) -> u64;
}

pub trait VkDeviceChildNonExtDestroyable {
    unsafe fn destroy(self, device: crate::vk::VkDevice, allocator: *const crate::vk::VkAllocationCallbacks);
}

/// A smart handle to a Vulkan object that holds a source lifetime
/// (bitpattern as same as native handle type)
#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct VkHandleRef<'r, H>(pub(crate) H, core::marker::PhantomData<&'r dyn VkHandle<Handle = H>>);
impl<'r, H> VkHandleRef<'r, H> {
    pub fn new(r: &'r (impl VkHandle<Handle = H> + ?Sized)) -> Self {
        Self(r.native_ptr(), core::marker::PhantomData)
    }

    /// simple raw handle wrapper without any lifetime constraints.
    /// # Safety
    /// owner of the handle must be alive while the handle will be used.
    pub const unsafe fn dangling(h: H) -> Self {
        Self(h, core::marker::PhantomData)
    }
}
impl<H: Copy> VkHandle for VkHandleRef<'_, H> {
    type Handle = H;

    #[inline(always)]
    fn native_ptr(&self) -> H {
        self.0
    }
}

/// A smart handle to a Vulkan object that holds a source lifetime and mutable-borrowing
/// (bitpattern as same as native handle type)
#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct VkHandleRefMut<'r, H>(
    pub(crate) H,
    core::marker::PhantomData<&'r mut dyn VkHandleMut<Handle = H>>,
);
impl<'r, H> VkHandleRefMut<'r, H> {
    pub fn new(r: &'r mut (impl VkHandleMut<Handle = H> + ?Sized)) -> Self {
        Self(r.native_ptr(), core::marker::PhantomData)
    }

    /// simple raw handle wrapper without any lifetime constraints.
    /// # Safety
    /// owner of the handle must be alive while the handle will be used.
    pub const unsafe fn dangling(h: H) -> Self {
        Self(h, core::marker::PhantomData)
    }
}
impl<H: Copy> VkHandle for VkHandleRefMut<'_, H> {
    type Handle = H;

    #[inline(always)]
    fn native_ptr(&self) -> H {
        self.0
    }
}
impl<H: Copy> VkHandleMut for VkHandleRefMut<'_, H> {
    #[inline(always)]
    fn native_ptr_mut(&mut self) -> H {
        self.0
    }
}
