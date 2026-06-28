use derives::implements;

use crate::*;

#[derive(VkHandle, VkObject)]
#[VkObject(type = brvk::VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR)]
pub struct DescriptorUpdateTemplateObject<Device: crate::Device>(
    pub(crate) brvk::VkDescriptorUpdateTemplateKHR,
    pub(crate) Device,
);
#[implements]
impl<Device: crate::Device> Drop for DescriptorUpdateTemplateObject<Device> {
    fn drop(&mut self) {
        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            brvk::fns::destroy_descriptor_update_template(self.1.native_ptr(), self.0, core::ptr::null());
        }
        #[cfg(not(feature = "Allow1_1APIs"))]
        unsafe {
            self.1.destroy_descriptor_update_template_khr_fn().0(self.1.native_ptr(), self.0, core::ptr::null());
        }
    }
}
unsafe impl<Device: crate::Device + Sync> Sync for DescriptorUpdateTemplateObject<Device> {}
unsafe impl<Device: crate::Device + Send> Send for DescriptorUpdateTemplateObject<Device> {}
impl<Device: crate::Device> DeviceChildHandle for DescriptorUpdateTemplateObject<Device> {
    #[inline(always)]
    fn device_handle(&self) -> brvk::VkDevice {
        self.1.native_ptr()
    }
}
impl<Device: crate::Device> DeviceChild for DescriptorUpdateTemplateObject<Device> {
    type ConcreteDevice = Device;

    #[inline(always)]
    fn device(&self) -> &Self::ConcreteDevice {
        &self.1
    }
}
impl<Device: crate::Device> DescriptorUpdateTemplate for DescriptorUpdateTemplateObject<Device> {}
impl<Device: crate::Device> DescriptorUpdateTemplateObject<Device> {
    /// Constructs from raw values
    /// # Safety
    /// the resource must be created from the device and not freed anywhere
    pub const unsafe fn manage(handle: brvk::VkDescriptorUpdateTemplateKHR, parent: Device) -> Self {
        Self(handle, parent)
    }

    /// Purges the construct (Drop will not be called for this resource)
    pub const fn unmanage(self) -> (brvk::VkDescriptorUpdateTemplateKHR, Device) {
        let h = self.0;
        let p = unsafe { core::ptr::read(&self.1) };
        core::mem::forget(self);

        (h, p)
    }
}
impl<Device: crate::Device + Clone> DescriptorUpdateTemplateObject<&'_ Device> {
    /// Owning parent object by cloning it.
    #[inline(always)]
    pub fn clone_parent(self) -> DescriptorUpdateTemplateObject<Device> {
        let r = DescriptorUpdateTemplateObject(self.0, self.1.clone());
        core::mem::forget(self);

        r
    }
}

pub trait DescriptorUpdateTemplate: VkHandle<Handle = brvk::VkDescriptorUpdateTemplateKHR> + DeviceChild {
    #[implements]
    fn update_set<T>(&self, set: brvk::VkDescriptorSet, data: &T) {
        #[cfg(feature = "Allow1_1APIs")]
        unsafe {
            brvk::fns::update_descriptor_set_with_template(
                self.device().native_ptr(),
                set,
                self.native_ptr(),
                data as *const _ as _,
            );
        }
        #[cfg(not(feature = "Allow1_1APIs"))]
        unsafe {
            use crate::Device;

            self.device().update_descriptor_set_with_template_khr_fn().0(
                self.device().native_ptr(),
                set,
                self.native_ptr(),
                data as *const T as *const _,
            )
        }
    }
}
DerefContainerBracketImpl!(for DescriptorUpdateTemplate {});
GuardsImpl!(for DescriptorUpdateTemplate {});
