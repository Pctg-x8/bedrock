use crate::vk::VkObjectType;

/// Wrapping a Vulkan Dispatchable/Nondispatchable Handler
pub trait VkHandle {
    type Handle;

    /// Retrieve an underlying handle
    fn native_ptr(&self) -> Self::Handle;
}
/// Wrapping a Vulkan Dispatchable/Nondispatchable Mutable Handler
pub trait VkHandleMut: VkHandle {
    /// Retrieve an underlying mutable handle
    fn native_ptr_mut(&mut self) -> Self::Handle;
}

DerefContainerBracketImpl!(for VkHandle {
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle { T::native_ptr(self) }
});
impl<T> VkHandleMut for &'_ mut T
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(*self)
    }
}

impl<T> VkHandle for std::cell::Ref<'_, T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T> VkHandle for std::cell::RefMut<'_, T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T> VkHandleMut for std::cell::RefMut<'_, T>
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

impl<T> VkHandle for std::sync::MutexGuard<'_, T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T> VkHandleMut for std::sync::MutexGuard<'_, T>
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

impl<T> VkHandle for std::sync::RwLockReadGuard<'_, T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}

impl<T> VkHandle for std::sync::RwLockWriteGuard<'_, T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T> VkHandleMut for std::sync::RwLockWriteGuard<'_, T>
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

impl<T> VkHandle for parking_lot::RwLockReadGuard<'_, T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T> VkHandle for parking_lot::RwLockWriteGuard<'_, T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T> VkHandleMut for parking_lot::RwLockWriteGuard<'_, T>
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

impl<T> VkHandle for parking_lot::MutexGuard<'_, T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&**self)
    }
}
impl<T> VkHandleMut for parking_lot::MutexGuard<'_, T>
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

impl<T> VkHandleMut for Box<T>
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(&mut **self)
    }
}

/// Unwrapping Option-ed Reference to VkHandles.  
/// Returns "Empty Handle" when the value is `None`.
impl<'h, H: VkHandle + ?Sized + 'h> VkHandle for Option<&'h H> {
    type Handle = <H as VkHandle>::Handle;

    fn native_ptr(&self) -> Self::Handle {
        self.map_or_else(
            || unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            |x| x.native_ptr(),
        )
    }
}

/// Unwrapping Option-ed Reference to VkHandles.  
/// Returns "Empty Handle" when the value is `None`.
impl<'h, H: VkHandle + ?Sized + 'h> VkHandle for Option<&'h mut H> {
    type Handle = <H as VkHandle>::Handle;

    fn native_ptr(&self) -> Self::Handle {
        self.as_ref().map_or_else(
            || unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            |x| x.native_ptr(),
        )
    }
}
/// Unwrapping Option-ed reference to VkHandles.
/// Returns "Empty Handle" when trhe value is `None`.
impl<'h, H: VkHandleMut + ?Sized + 'h> VkHandleMut for Option<&'h mut H> {
    fn native_ptr_mut(&mut self) -> Self::Handle {
        self.as_mut().map_or_else(
            || unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            |x| x.native_ptr_mut(),
        )
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

#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct VkHandleRef<'r, H>(H, core::marker::PhantomData<&'r dyn VkHandle<Handle = H>>);
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

    fn native_ptr(&self) -> H {
        self.0
    }
}
impl<H: Copy> VkHandleMut for VkHandleRef<'_, H> {
    fn native_ptr_mut(&mut self) -> H {
        self.0
    }
}
