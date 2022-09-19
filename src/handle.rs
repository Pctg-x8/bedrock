/// Wrapping a Vulkan Dispatchable/Nondispatchable Handler
pub trait VkHandle {
    type Handle;

    /// Retrieve an underlying handle
    fn native_ptr(&self) -> Self::Handle;
}
DerefContainerBracketImpl!(for VkHandle {
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle { T::native_ptr(self) }
});
impl<T> VkHandle for &'_ mut T
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(*self)
    }
}
impl<T> VkHandle for std::cell::RefCell<T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&self.borrow())
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
impl<T> VkHandle for parking_lot::RwLock<T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&self.read())
    }
}
impl<T> VkHandle for parking_lot::Mutex<T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&self.lock())
    }
}

/// Unwrapping Option-ed Reference to VkHandles.  
/// Returns "Empty Handle" when the value is `None`.
impl<'h, H: VkHandle + ?Sized + 'h> VkHandle for Option<&'h H> {
    type Handle = <H as VkHandle>::Handle;

    fn native_ptr(&self) -> Self::Handle {
        self.map_or(unsafe { std::mem::zeroed() }, |x| x.native_ptr())
    }
}
