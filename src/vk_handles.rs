#![allow(non_camel_case_types)]

// === Dispatchable Handles ===
// These are always opaque pointers to incomplete types.

#[repr(C)]
pub struct VkInstance_T {
    _private: [u8; 0],
}
pub type VkInstance = *mut VkInstance_T;

#[repr(C)]
pub struct VkPhysicalDevice_T {
    _private: [u8; 0],
}
pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;

#[repr(C)]
pub struct VkDevice_T {
    _private: [u8; 0],
}
pub type VkDevice = *mut VkDevice_T;

#[repr(C)]
pub struct VkQueue_T {
    _private: [u8; 0],
}
pub type VkQueue = *mut VkQueue_T;

#[repr(C)]
pub struct VkCommandBuffer_T {
    _private: [u8; 0],
}
pub type VkCommandBuffer = *mut VkCommandBuffer_T;

// === Non-dispatchable Handles ===
// These are just integers (64-bit handles).
pub type VkBuffer = u64;
pub type VkImage = u64;
pub type VkShaderModule = u64;
pub type VkPipeline = u64;
pub type VkPipelineLayout = u64;
pub type VkRenderPass = u64;
pub type VkFramebuffer = u64;
pub type VkDescriptorSetLayout = u64;
pub type VkDescriptorPool = u64;
pub type VkDescriptorSet = u64;
pub type VkSampler = u64;
pub type VkEvent = u64;
pub type VkFence = u64;
pub type VkSemaphore = u64;
pub type VkQueryPool = u64;
pub type VkCommandPool = u64;
pub type VkPipelineCache = u64;
pub type VkDeviceMemory = u64;
