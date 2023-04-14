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
impl<T> VkHandle for &'_ mut T
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(*self)
    }
}
impl<T> VkHandleMut for &'_ mut T
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(*self)
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
impl<T> VkHandleMut for std::cell::RefCell<T>
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(self.get_mut())
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

impl<T> VkHandle for parking_lot::RwLock<T>
where
    T: VkHandle + ?Sized,
{
    type Handle = T::Handle;

    fn native_ptr(&self) -> Self::Handle {
        T::native_ptr(&self.read())
    }
}
impl<T> VkHandleMut for parking_lot::RwLock<T>
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr_mut(self.get_mut())
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
impl<T> VkHandleMut for parking_lot::Mutex<T>
where
    T: VkHandleMut + ?Sized,
{
    fn native_ptr_mut(&mut self) -> Self::Handle {
        T::native_ptr(self.get_mut())
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
