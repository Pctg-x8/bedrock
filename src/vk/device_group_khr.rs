pub const VK_KHR_DEVICE_GROUP_SPEC_VERSION: usize = 4;
pub const VK_KHR_DEVICE_GROUP_EXTENSION_NAME: &str = "VK_KHR_device_group";

use derives::promote_1_1;

use super::*;
use crate::PFN;

#[promote_1_1]
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR: VkStructureType = ext_enum_value(61, 0) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: VkStructureType = ext_enum_value(61, 3) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: VkStructureType = ext_enum_value(61, 4) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR: VkStructureType = ext_enum_value(61, 5) as _;
#[promote_1_1]
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR: VkStructureType = ext_enum_value(61, 6) as _;

#[promote_1_1(suffix = "KHR")]
pub type VkPeerMemoryFeatureFlagsKHR = VkFlags;
#[promote_1_1(suffix = "KHR")]
pub type VkPeerMemoryFeatureFlagBitsKHR = VkFlags;
#[promote_1_1]
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBitsKHR = 1 << 0;
#[promote_1_1]
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR: VkPeerMemoryFeatureFlagBitsKHR = 1 << 1;
#[promote_1_1]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBitsKHR = 1 << 2;
#[promote_1_1]
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR: VkPeerMemoryFeatureFlagBitsKHR = 1 << 3;

#[promote_1_1(suffix = "KHR")]
pub type VkMemoryAllocateFlagsKHR = VkFlags;
#[promote_1_1(suffix = "KHR")]
pub type VkMemoryAllocateFlagBitsKHR = VkFlags;
#[promote_1_1]
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR: VkMemoryAllocateFlagBitsKHR = 1 << 0;

#[promote_1_1]
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: VkPipelineCreateFlagBits = 1 << 3;
#[promote_1_1]
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_KHR: VkPipelineCreateFlagBits = 1 << 4;

#[promote_1_1]
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR: VkDependencyFlagBits = 1 << 2;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkMemoryAllocateFlagsInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMemoryAllocateFlagsKHR,
    pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkDeviceGroupRenderPassBeginInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
    pub deviceRenderAreaCount: u32,
    pub pDeviceRenderAreas: *const VkRect2D,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkDeviceGroupCommandBufferBeginInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkDeviceGroupSubmitInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphoreDeviceIndices: *const u32,
    pub commandBufferCount: u32,
    pub pCommandBufferDeviceMasks: *const u32,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphoreDeviceIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
#[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR)]
#[promote_1_1(suffix = "KHR")]
pub struct VkDeviceGroupBindSparseInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub resourceDeviceIndex: u32,
    pub memoryDeviceIndex: u32,
}

#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkGetDeviceGroupPeerMemoryFeaturesKHR)]
pub struct PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR(
    pub  unsafe extern "system" fn(
        device: VkDevice,
        heapIndex: u32,
        localDeviceIndex: u32,
        remoteDeviceIndex: u32,
        pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHR,
    ),
);

#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdSetDeviceMaskKHR)]
pub struct PFN_vkCmdSetDeviceMaskKHR(pub unsafe extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: u32));
#[promote_1_1(suffix = "KHR")]
#[repr(transparent)]
#[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)]
#[pfn_of(vkCmdDispatchBaseKHR)]
pub struct PFN_vkCmdDispatchBaseKHR(
    pub  unsafe extern "system" fn(
        commandBuffer: VkCommandBuffer,
        baseGroupX: u32,
        baseGroupY: u32,
        baseGroupZ: u32,
        groupCountX: u32,
        groupCountY: u32,
        groupCountZ: u32,
    ),
);

#[cfg(feature = "Implements")]
#[cfg(not(feature = "DynamicLoaded"))]
extern "system" {
    #[promote_1_1(suffix = "KHR")]
    pub fn vkGetDeviceGroupPeerMemoryFeaturesKHR(
        device: VkDevice,
        heapIndex: u32,
        localDeviceIndex: u32,
        remoteDeviceIndex: u32,
        pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHR,
    );

    #[promote_1_1(suffix = "KHR")]
    pub fn vkCmdSetDeviceMaskKHR(commandBuffer: VkCommandBuffer, deviceMask: u32);
    #[promote_1_1(suffix = "KHR")]
    pub fn vkCmdDispatchBaseKHR(
        commandBuffer: VkCommandBuffer,
        baseGroupX: u32,
        baseGroupY: u32,
        baseGroupZ: u32,
        groupCountX: u32,
        groupCountY: u32,
        groupCountZ: u32,
    );
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_bind_memory2")] {
        #[promote_1_1]
        pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType = ext_enum_value(61, 13) as _;
        #[promote_1_1]
        pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType = ext_enum_value(61, 14) as _;

        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
        #[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR)]
        #[promote_1_1(suffix = "KHR")]
        pub struct VkBindBufferMemoryDeviceGroupInfoKHR {
            pub sType: VkStructureType,
            pub pNext: *const c_void,
            pub deviceIndexCount: u32,
            pub pDeviceIndices: *const u32
        }

        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
        #[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR)]
        #[promote_1_1(suffix = "KHR")]
        pub struct VkBindImageMemoryDeviceGroupInfoKHR {
            pub sType: VkStructureType,
            pub pNext: *const c_void,
            pub deviceIndexCount: u32,
            pub pDeviceIndices: *const u32,
            pub splitInstanceBindRegionCount: u32,
            pub pSplitInstanceBindRegions: *const VkRect2D
        }

        vk_bitmask! {
            extending enum VkImageCreateFlagBits {
                #[promote_1_1]
                pub VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: 6
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_surface")] {
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: VkStructureType = ext_enum_value(61, 7) as _;

        pub type VkDeviceGroupPresentModeFlagsKHR = VkFlags;
        vk_bitmask! { pub enum VkDeviceGroupPresentModeFlagBitsKHR {} }

        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
        #[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR)]
        pub struct VkDeviceGroupPresentCapabilitiesKHR {
            pub sType: VkStructureType,
            pub pNext: *mut c_void,
            pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE_KHR],
            pub modes: VkDeviceGroupPresentModeFlagsKHR
        }

        #[repr(transparent)] #[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)] #[pfn_of(vkGetDeviceGroupPresentCapabilitiesKHR)] pub struct PFN_vkGetDeviceGroupPresentCapabilitiesKHR(pub unsafe extern "system" fn(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR) -> VkResult);
        #[repr(transparent)] #[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)] #[pfn_of(vkGetDeviceGroupSurfacePresentModesKHR)] pub struct PFN_vkGetDeviceGroupSurfacePresentModesKHR(pub unsafe extern "system" fn(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult);
        #[repr(transparent)] #[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)] #[pfn_of(vkGetPhysicalDevicePresentRectanglesKHR)] pub struct PFN_vkGetPhysicalDevicePresentRectanglesKHR(pub unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult);

        #[cfg(feature = "Implements")]
        #[cfg(not(feature = "DynamicLoaded"))]
        extern "system" {
            pub fn vkGetDeviceGroupPresentCapabilitiesKHR(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR) -> VkResult;
            pub fn vkGetDeviceGroupSurfacePresentModesKHR(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;
            pub fn vkGetPhysicalDevicePresentRectanglesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult;
        }
    }
}

cfg_if! {
    if #[cfg(feature = "VK_KHR_swapchain")] {
        pub const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = ext_enum_value(61, 8) as _;
        pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: VkStructureType = ext_enum_value(61, 9) as _;
        pub const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR: VkStructureType = ext_enum_value(61, 10) as _;
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR: VkStructureType = ext_enum_value(61, 11) as _;
        pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = ext_enum_value(61, 12) as _;

        vk_bitmask! {
            extending enum VkSwapchainCreateFlagBitsKHR {
                pub VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: 0
            }
        }

        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
        #[VulkanStructure(type = VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR)]
        pub struct VkImageSwapchainCreateInfoKHR {
            pub sType: VkStructureType,
            pub pNext: *const c_void,
            pub swapchain: VkSwapchainKHR
        }

        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
        #[VulkanStructure(type = VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR)]
        pub struct VkBindImageMemorySwapchainInfoKHR {
            pub sType: VkStructureType,
            pub pNext: *const c_void,
            pub swapchain: VkSwapchainKHR,
            pub imageIndex: u32
        }

        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
        #[VulkanStructure(type = VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR)]
        pub struct VkAcquireNextImageInfoKHR {
            pub sType: VkStructureType,
            pub pNext: *const c_void,
            pub swapchain: VkSwapchainKHR,
            pub timeout: u64,
            pub semaphore: VkSemaphore,
            pub fence: VkFence,
            pub deviceMask: u32
        }

        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
        #[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR)]
        pub struct VkDeviceGroupPresentInfoKHR {
            pub sType: VkStructureType,
            pub pNext: *const c_void,
            pub swapchainCount: u32,
            pub pDeviceMasks: *const u32,
            pub mode: VkDeviceGroupPresentModeFlagBitsKHR
        }

        #[repr(C)]
        #[derive(Debug, Clone, PartialEq, Eq, VulkanStructure)]
        #[VulkanStructure(type = VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR)]
        pub struct VkDeviceGroupSwapchainCreateInfoKHR {
            pub sType: VkStructureType,
            pub pNext: *const c_void,
            pub modes: VkDeviceGroupPresentModeFlagsKHR
        }

        #[repr(transparent)] #[derive(PFN, Clone, Copy, Debug, PartialEq, Eq)] #[pfn_of(vkAcquireNextImage2KHR)] pub struct PFN_vkAcquireNextImage2KHR(pub unsafe extern "system" fn(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHR, pImageIndex: *mut u32) -> VkResult);

        #[cfg(feature = "Implements")]
        #[cfg(not(feature = "DynamicLoaded"))]
        extern "system" {
            pub fn vkAcquireNextImage2KHR(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHR, pImageIndex: *mut u32) -> VkResult;
        }
    }
}
