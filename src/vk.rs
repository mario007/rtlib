#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::Read;
use std::mem::transmute;
use std::{ffi::CString, ptr};
use libloading::{Library, Symbol};

use crate::vk_const::{VK_NULL_HANDLE, VK_SUCCESS, VK_STRUCTURE_TYPE_APPLICATION_INFO,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
    VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
    VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
    VK_COMMAND_BUFFER_LEVEL_PRIMARY, VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
    VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
    VK_SHADER_STAGE_COMPUTE_BIT,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
    VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
    VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
    VK_SHARING_MODE_EXCLUSIVE,
    VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
    VK_MAX_MEMORY_TYPES, VK_MAX_MEMORY_HEAPS,
    VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
    VK_PIPELINE_BIND_POINT_COMPUTE,
    VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
    VK_STRUCTURE_TYPE_SUBMIT_INFO};

use crate::vk_types::{PFN_vkAllocateCommandBuffers, PFN_vkCreateBuffer,
    PFN_vkCreateCommandPool, PFN_vkCreateComputePipelines,
    PFN_vkCreateDescriptorSetLayout,
    PFN_vkCreateDevice, PFN_vkCreateInstance, PFN_vkCreatePipelineLayout,
    PFN_vkCreateShaderModule, PFN_vkDestroyBuffer, PFN_vkDestroyCommandPool,
    PFN_vkDestroyDescriptorSetLayout, PFN_vkDestroyDevice, PFN_vkDestroyInstance,
    PFN_vkDestroyPipeline, PFN_vkDestroyPipelineLayout, PFN_vkDestroyShaderModule,
    PFN_vkEnumerateInstanceExtensionProperties, PFN_vkEnumeratePhysicalDevices,
    PFN_vkGetDeviceProcAddr, PFN_vkGetDeviceQueue, PFN_vkGetInstanceProcAddr,
    PFN_vkGetPhysicalDeviceFeatures, PFN_vkGetPhysicalDeviceProperties,
    PFN_vkGetPhysicalDeviceQueueFamilyProperties, VkApplicationInfo,
    VkBufferCreateInfo, VkCommandBufferAllocateInfo, VkCommandPoolCreateInfo,
    VkComputePipelineCreateInfo, VkDescriptorSetLayoutBinding,
    VkDescriptorSetLayoutCreateInfo, VkDeviceCreateInfo,
    VkDeviceQueueCreateInfo, VkDeviceSize, VkExtensionProperties,
    VkInstanceCreateInfo, VkPhysicalDeviceFeatures, VkPhysicalDeviceProperties,
    VkPhysicalDeviceType, VkPipelineLayoutCreateInfo, VkPipelineShaderStageCreateInfo,
    VkQueueFamilyProperties, VkShaderModuleCreateInfo,
    PFN_vkCreateDescriptorPool, PFN_vkDestroyDescriptorPool,
    VkDescriptorPoolCreateInfo, VkDescriptorPoolSize,
    PFN_vkAllocateDescriptorSets, PFN_vkUpdateDescriptorSets,
    VkDescriptorSetAllocateInfo, VkDescriptorBufferInfo, VkWriteDescriptorSet,
    PFN_vkGetPhysicalDeviceMemoryProperties, VkMemoryType, VkMemoryHeap, 
    VkPhysicalDeviceMemoryProperties, PFN_vkGetBufferMemoryRequirements, 
    VkMemoryRequirements, VkMemoryPropertyFlags, PFN_vkAllocateMemory,
    VkMemoryAllocateInfo, PFN_vkFreeMemory, PFN_vkBindBufferMemory,
    PFN_vkMapMemory, PFN_vkUnmapMemory, PFN_vkBeginCommandBuffer,
    PFN_vkEndCommandBuffer, VkCommandBufferBeginInfo,
    PFN_vkCmdBindPipeline, PFN_vkCmdBindDescriptorSets, PFN_vkCmdDispatch, 
    PFN_vkQueueSubmit, VkSubmitInfo, PFN_vkQueueWaitIdle,
    PFN_vkCreateFence, VkFenceCreateInfo, PFN_vkDestroyFence, PFN_vkWaitForFences};

use crate::vk_types::VK_QUEUE_COMPUTE_BIT;

use crate::vk_handles::{VkCommandBuffer, VkCommandPool, VkDevice, VkInstance,
    VkPhysicalDevice, VkPipelineLayout, VkQueue, VkShaderModule,
    VkDescriptorSetLayout, VkPipeline, VkBuffer, VkDescriptorPool,
    VkDescriptorSet, VkDeviceMemory, VkFence};

pub fn load_vulkan_dyn_library() -> Option<Library> {
    let lib = unsafe {
        #[cfg(target_os = "windows")]
        let lib = Library::new("vulkan-1.dll").ok();
        #[cfg(all(unix, not(target_os = "macos")))]
        let lib = Library::new("libvulkan.so.1").or_else(|_| Library::new("libvulkan.so")).ok();
        #[cfg(target_os = "macos")]
        let lib = Library::new("libvulkan.1.dylib").ok();
        lib
    };
    lib
}


fn vk_make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    ((variant & 0x7) << 29)
        | ((major & 0x3ff) << 22)
        | ((minor & 0x3f) << 12)
        | (patch & 0xfff)
}


pub fn find_memory_type(
    type_filter: u32,
    properties: VkMemoryPropertyFlags,
    mem_properties: &VkPhysicalDeviceMemoryProperties,
) -> Option<u32> {
    for i in 0..mem_properties.memoryTypeCount {
        if (type_filter & (1 << i)) != 0 &&
           (mem_properties.memoryTypes[i as usize].propertyFlags & properties) == properties {
            return Some(i);
        }
    }
    None
}


pub fn read_spv_shader(path: &str) -> Vec<u32> {
    let mut file = File::open(path).expect("Failed to open SPIR-V file");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    // Align to u32
    assert!(bytes.len() % 4 == 0);
    let mut code = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks(4) {
        code.push(u32::from_le_bytes(chunk.try_into().unwrap()));
    }
    code
}


pub struct VulkanLib {
    lib: Library,
    vkGetInstanceProcAddr: PFN_vkGetInstanceProcAddr,
    vkEnumerateInstanceExtensionProperties: PFN_vkEnumerateInstanceExtensionProperties,
    vkCreateInstance: PFN_vkCreateInstance
}

#[repr(C)]
pub struct VkInstance_T1 {
    _private: [u8; 0],
}
pub type VkInstance1 = *mut VkInstance_T1;


impl VulkanLib {
    pub fn new(lib: Library) -> VulkanLib {
        let vkGetInstanceProcAddr: Symbol<PFN_vkGetInstanceProcAddr> =
        unsafe {
            lib.get(b"vkGetInstanceProcAddr").expect("find vkGetInstanceProcAddr")
        };
        let vkGetInstanceProcAddr = *vkGetInstanceProcAddr;

        let name = CString::new("vkEnumerateInstanceExtensionProperties").unwrap();
        let vkEnumerateInstanceExtensionProperties: PFN_vkEnumerateInstanceExtensionProperties =
            unsafe { transmute(vkGetInstanceProcAddr(VK_NULL_HANDLE, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkCreateInstance").unwrap();
        let vkCreateInstance: PFN_vkCreateInstance = unsafe { transmute(
            vkGetInstanceProcAddr(VK_NULL_HANDLE, name.as_ptr()).unwrap())};

        VulkanLib { 
            lib, 
            vkGetInstanceProcAddr,
            vkEnumerateInstanceExtensionProperties,
            vkCreateInstance
        }
    }

    pub fn enumerate_extensions(&self) -> Vec<String> {
        let mut count: u32 = 0;
        let mut extensions = Vec::<String>::new();
        unsafe { (self.vkEnumerateInstanceExtensionProperties)(ptr::null(), &mut count, ptr::null_mut()); }
        let mut exts = Vec::<VkExtensionProperties>::with_capacity(count as usize);
        unsafe {(self.vkEnumerateInstanceExtensionProperties)(
            ptr::null(),
            &mut count,
            exts.as_mut_ptr(),
        );
        exts.set_len(count as usize);

        for ext in &exts {
            let cname = std::ffi::CStr::from_ptr(ext.extensionName.as_ptr());
            extensions.push(String::from(cname.to_string_lossy()));
        }};
        extensions
    }

    pub fn create_instance(&self) -> VkInstance {
        let app_name = CString::new("vk-dynamic").unwrap();
        let app_info = VkApplicationInfo {
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: ptr::null(),
            pApplicationName: app_name.as_ptr(),
            applicationVersion: 1,
            pEngineName: app_name.as_ptr(),
            engineVersion: 1,
            apiVersion: vk_make_api_version(0, 1, 3, 0),
        };

        let create_info = VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            pApplicationInfo: &app_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: ptr::null(),
        };

        let mut instance: VkInstance = VK_NULL_HANDLE;
        let res = unsafe { (self.vkCreateInstance)(&create_info, ptr::null(), &mut instance) };
        assert_eq!(res, VK_SUCCESS, "vkCreateInstance failed: {}", res);
        instance
    }

}


pub struct VulkanInstance {
    lib: VulkanLib,
    instance: VkInstance,
    vkDestroyInstance: PFN_vkDestroyInstance,
    vkEnumeratePhysicalDevices: PFN_vkEnumeratePhysicalDevices,
    vkGetPhysicalDeviceProperties: PFN_vkGetPhysicalDeviceProperties,
    vkGetPhysicalDeviceFeatures: PFN_vkGetPhysicalDeviceFeatures,
    vkGetPhysicalDeviceQueueFamilyProperties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    vkCreateDevice: PFN_vkCreateDevice,
    vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr,
    vkGetPhysicalDeviceMemoryProperties: PFN_vkGetPhysicalDeviceMemoryProperties
}

impl VulkanInstance {
    pub fn new(lib: VulkanLib, instance: VkInstance) -> VulkanInstance {
        let name = CString::new("vkDestroyInstance").unwrap();
        let vkDestroyInstance: PFN_vkDestroyInstance = unsafe { transmute(
             (lib.vkGetInstanceProcAddr)(instance, name.as_ptr()).unwrap())};

        let name = CString::new("vkEnumeratePhysicalDevices").unwrap();
        let vkEnumeratePhysicalDevices: PFN_vkEnumeratePhysicalDevices = unsafe { transmute(
            (lib.vkGetInstanceProcAddr)(instance, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkGetPhysicalDeviceProperties").unwrap();
        let vkGetPhysicalDeviceProperties: PFN_vkGetPhysicalDeviceProperties = unsafe { transmute(
            (lib.vkGetInstanceProcAddr)(instance, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkGetPhysicalDeviceFeatures").unwrap();
        let vkGetPhysicalDeviceFeatures: PFN_vkGetPhysicalDeviceFeatures = unsafe { transmute(
            (lib.vkGetInstanceProcAddr)(instance, name.as_ptr()).unwrap(),
        ) };

        let name = CString::new("vkGetPhysicalDeviceQueueFamilyProperties").unwrap();
        let vkGetPhysicalDeviceQueueFamilyProperties: PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe { transmute(
            (lib.vkGetInstanceProcAddr)(instance, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkCreateDevice").unwrap();
        let vkCreateDevice: PFN_vkCreateDevice = unsafe { transmute(
            (lib.vkGetInstanceProcAddr)(instance, name.as_ptr()).unwrap())};

        let name = CString::new("vkGetDeviceProcAddr").unwrap();
        let vkGetDeviceProcAddr: PFN_vkGetDeviceProcAddr = unsafe { transmute(
            (lib.vkGetInstanceProcAddr)(instance, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkGetPhysicalDeviceMemoryProperties").unwrap();
        let vkGetPhysicalDeviceMemoryProperties: PFN_vkGetPhysicalDeviceMemoryProperties =
        unsafe { transmute(
            (lib.vkGetInstanceProcAddr)(instance, name.as_ptr()).unwrap())};

        VulkanInstance { 
            lib, 
            instance,
            vkDestroyInstance,
            vkEnumeratePhysicalDevices,
            vkGetPhysicalDeviceProperties,
            vkGetPhysicalDeviceFeatures,
            vkGetPhysicalDeviceQueueFamilyProperties,
            vkCreateDevice,
            vkGetDeviceProcAddr,
            vkGetPhysicalDeviceMemoryProperties
        }
    }

    pub fn enumerate_physical_devices(&self) -> Vec<VkPhysicalDevice> {
        let mut count: u32 = 0;
        let _result = unsafe { (self.vkEnumeratePhysicalDevices)(self.instance, &mut count, ptr::null_mut()) };
        let mut exts = Vec::<VkPhysicalDevice>::with_capacity(count as usize);
        unsafe {
            (self.vkEnumeratePhysicalDevices)(
            self.instance,
            &mut count,
            exts.as_mut_ptr(),
            );
            exts.set_len(count as usize);
        }
        exts
    }

    pub fn get_physical_device_properties(&self, device: VkPhysicalDevice) -> VkPhysicalDeviceProperties {
        let mut properties: VkPhysicalDeviceProperties = unsafe { std::mem::zeroed() };
        unsafe { (self.vkGetPhysicalDeviceProperties)(device, &mut properties) };
        properties
    }

    pub fn get_physical_device_features(&self, device: VkPhysicalDevice) -> VkPhysicalDeviceFeatures {
        let mut features: VkPhysicalDeviceFeatures = unsafe { std::mem::zeroed() };
        unsafe { (self.vkGetPhysicalDeviceFeatures)(device, &mut features) };
        features
    }

    pub fn get_physical_device_queue_family_properties(&self, device: VkPhysicalDevice) -> Vec<VkQueueFamilyProperties> {
        let mut count: u32 = 0;
        unsafe { (self.vkGetPhysicalDeviceQueueFamilyProperties)(device, &mut count, ptr::null_mut()) };
        let mut queue_family_properties = Vec::<VkQueueFamilyProperties>::with_capacity(count as usize);
        
        unsafe {
            (self.vkGetPhysicalDeviceQueueFamilyProperties)(
            device,
            &mut count,
            queue_family_properties.as_mut_ptr(),
            );
            queue_family_properties.set_len(count as usize);
        }
        queue_family_properties
    }

    pub fn find_compute_queueFamilyIndex(&self, physical_device: VkPhysicalDevice) -> Option<u32> {
        let queues = self.get_physical_device_queue_family_properties(physical_device);
        for (i, queue) in queues.iter().enumerate() {
            if (queue.queueFlags & VK_QUEUE_COMPUTE_BIT) != 0 {
                return Some(i as u32);
            } 
        }
        None
    }

    pub fn create_device(&self, physical_device: VkPhysicalDevice, queue_family_index: u32) -> (VkDevice, VkQueue, u32) {
        let queue_priority: f32 = 1.0;
        let queue_info = VkDeviceQueueCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            queueFamilyIndex: queue_family_index,
            queueCount: 1,
            pQueuePriorities: &queue_priority,
        };
        let device_info = VkDeviceCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            queueCreateInfoCount: 1,
            pQueueCreateInfos: &queue_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: ptr::null(),
            pEnabledFeatures: ptr::null(),
        };

        let mut device: VkDevice = ptr::null_mut();
        let res = unsafe { (self.vkCreateDevice)(physical_device, &device_info, ptr::null(), &mut device) };
        assert_eq!(res, VK_SUCCESS, "vkCreateDevice failed: {}", res);

        let name = CString::new("vkGetDeviceQueue").unwrap();
        let vkGetDeviceQueue: PFN_vkGetDeviceQueue = unsafe { std::mem::transmute(
            (self.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap(),
        ) };

        let mut queue: VkQueue = ptr::null_mut();
        unsafe {vkGetDeviceQueue(device, queue_family_index, 0, &mut queue)}
        (device, queue, queue_family_index)
    }

    pub fn create_best_compute_device(&self, devices: &Vec<VkPhysicalDevice>) -> Option<(VkDevice, VkQueue, u32)> {
        // first try to create discrete gpu compute device
        for device in devices {
            let p = self.get_physical_device_properties(*device);
            match p.deviceType {
                VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => {
                    match self.find_compute_queueFamilyIndex(*device) {
                        Some(index) => {
                            return Some(self.create_device(*device, index));
                        }
                        _ => ()
                    }
                },
                _ => ()
            };
        }
        // try to create integrated gpu compute device
        for device in devices {
            let p = self.get_physical_device_properties(*device);
            match p.deviceType {
                VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => {
                    match self.find_compute_queueFamilyIndex(*device) {
                        Some(index) => {
                            return Some(self.create_device(*device, index));
                        }
                        _ => ()
                    }
                },
                _ => ()
            };
        }
        None
    }

    pub fn get_physical_device_memory_properties(&self, device: VkPhysicalDevice) -> VkPhysicalDeviceMemoryProperties {
        let mut mem_properties = VkPhysicalDeviceMemoryProperties {
            memoryTypeCount: 0,
            memoryTypes: [VkMemoryType { propertyFlags: 0, heapIndex: 0 }; VK_MAX_MEMORY_TYPES],
            memoryHeapCount: 0,
            memoryHeaps: [VkMemoryHeap { size: 0, flags: 0 }; VK_MAX_MEMORY_HEAPS],
        };

        unsafe {(self.vkGetPhysicalDeviceMemoryProperties)(device, &mut mem_properties);}
        mem_properties
    }

}

impl Drop for VulkanInstance {
    fn drop(&mut self) {
        unsafe { (self.vkDestroyInstance)(self.instance, ptr::null()) };
    }
}

pub struct VulkanDevice {
    vk_instance: VulkanInstance,
    device: VkDevice,
    queue: VkQueue,
    queue_family_index: u32,
    vkDestroyDevice: PFN_vkDestroyDevice,
    vkCreateCommandPool: PFN_vkCreateCommandPool,
    vkDestroyCommandPool: PFN_vkDestroyCommandPool,
    vkAllocateCommandBuffers: PFN_vkAllocateCommandBuffers,
    vkCreateShaderModule: PFN_vkCreateShaderModule,
    vkDestroyShaderModule: PFN_vkDestroyShaderModule,
    vkCreateDescriptorSetLayout: PFN_vkCreateDescriptorSetLayout,
    vkDestroyDescriptorSetLayout: PFN_vkDestroyDescriptorSetLayout,
    vkCreatePipelineLayout: PFN_vkCreatePipelineLayout,
    vkDestroyPipelineLayout: PFN_vkDestroyPipelineLayout,
    vkCreateComputePipelines: PFN_vkCreateComputePipelines,
    vkDestroyPipeline: PFN_vkDestroyPipeline,
    vkCreateBuffer: PFN_vkCreateBuffer,
    vkDestroyBuffer: PFN_vkDestroyBuffer,
    vkCreateDescriptorPool: PFN_vkCreateDescriptorPool,
    vkDestroyDescriptorPool: PFN_vkDestroyDescriptorPool,
    vkAllocateDescriptorSets: PFN_vkAllocateDescriptorSets,
    vkUpdateDescriptorSets: PFN_vkUpdateDescriptorSets,
    vkGetBufferMemoryRequirements: PFN_vkGetBufferMemoryRequirements,
    vkAllocateMemory: PFN_vkAllocateMemory,
    vkFreeMemory: PFN_vkFreeMemory,
    vkBindBufferMemory: PFN_vkBindBufferMemory,
    vkMapMemory: PFN_vkMapMemory,
    vkUnmapMemory: PFN_vkUnmapMemory,
    vkBeginCommandBuffer: PFN_vkBeginCommandBuffer,
    vkEndCommandBuffer: PFN_vkEndCommandBuffer,
    vkCmdBindPipeline: PFN_vkCmdBindPipeline,
    vkCmdBindDescriptorSets: PFN_vkCmdBindDescriptorSets,
    vkCmdDispatch: PFN_vkCmdDispatch,
    vkQueueSubmit: PFN_vkQueueSubmit,
    vkQueueWaitIdle: PFN_vkQueueWaitIdle,
    vkCreateFence: PFN_vkCreateFence,
    vkDestroyFence: PFN_vkDestroyFence,
    vkWaitForFences: PFN_vkWaitForFences
}

impl VulkanDevice {
    pub fn new(vk_instance: VulkanInstance, device: VkDevice, queue: VkQueue, queue_family_index: u32) -> VulkanDevice {
        let name = CString::new("vkDestroyDevice").unwrap();
        let vkDestroyDevice: PFN_vkDestroyDevice = unsafe {transmute(
            (vk_instance.lib.vkGetInstanceProcAddr)(vk_instance.instance, name.as_ptr()).unwrap())};

        let name = CString::new("vkCreateCommandPool").unwrap();
        let vkCreateCommandPool: PFN_vkCreateCommandPool = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkDestroyCommandPool").unwrap();
        let vkDestroyCommandPool: PFN_vkDestroyCommandPool = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkAllocateCommandBuffers").unwrap();
        let vkAllocateCommandBuffers: PFN_vkAllocateCommandBuffers = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};

        let name = CString::new("vkCreateShaderModule").unwrap();
        let vkCreateShaderModule: PFN_vkCreateShaderModule = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkDestroyShaderModule").unwrap();
        let vkDestroyShaderModule: PFN_vkDestroyShaderModule = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkCreateDescriptorSetLayout").unwrap();
        let vkCreateDescriptorSetLayout: PFN_vkCreateDescriptorSetLayout = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkDestroyDescriptorSetLayout").unwrap();
        let vkDestroyDescriptorSetLayout: PFN_vkDestroyDescriptorSetLayout = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkCreatePipelineLayout").unwrap();
        let vkCreatePipelineLayout: PFN_vkCreatePipelineLayout = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkDestroyPipelineLayout").unwrap();
        let vkDestroyPipelineLayout: PFN_vkDestroyPipelineLayout = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkCreateComputePipelines").unwrap();
        let vkCreateComputePipelines: PFN_vkCreateComputePipelines = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkDestroyPipeline").unwrap();
        let vkDestroyPipeline: PFN_vkDestroyPipeline = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkCreateBuffer").unwrap();
        let vkCreateBuffer: PFN_vkCreateBuffer = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkDestroyBuffer").unwrap();
        let vkDestroyBuffer: PFN_vkDestroyBuffer = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkCreateDescriptorPool").unwrap();
        let vkCreateDescriptorPool: PFN_vkCreateDescriptorPool = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};

        let name = CString::new("vkDestroyDescriptorPool").unwrap();
        let vkDestroyDescriptorPool: PFN_vkDestroyDescriptorPool = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};

         let name = CString::new("vkAllocateDescriptorSets").unwrap();
         let vkAllocateDescriptorSets: PFN_vkAllocateDescriptorSets = unsafe { transmute(
             (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkUpdateDescriptorSets").unwrap();
        let vkUpdateDescriptorSets: PFN_vkUpdateDescriptorSets = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkGetBufferMemoryRequirements").unwrap();
        let vkGetBufferMemoryRequirements: PFN_vkGetBufferMemoryRequirements = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkAllocateMemory").unwrap();
        let vkAllocateMemory: PFN_vkAllocateMemory = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkFreeMemory").unwrap();
        let vkFreeMemory: PFN_vkFreeMemory = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};

        let name = CString::new("vkBindBufferMemory").unwrap();
        let vkBindBufferMemory: PFN_vkBindBufferMemory = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkMapMemory").unwrap();
        let vkMapMemory: PFN_vkMapMemory = unsafe {transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};

        let name = CString::new("vkUnmapMemory").unwrap();
        let vkUnmapMemory: PFN_vkUnmapMemory = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkBeginCommandBuffer").unwrap();
        let vkBeginCommandBuffer: PFN_vkBeginCommandBuffer = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkEndCommandBuffer").unwrap();
        let vkEndCommandBuffer: PFN_vkEndCommandBuffer = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkCmdBindPipeline").unwrap();
        let vkCmdBindPipeline: PFN_vkCmdBindPipeline = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkCmdBindDescriptorSets").unwrap();
        let vkCmdBindDescriptorSets: PFN_vkCmdBindDescriptorSets = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
    
        let name = CString::new("vkCmdDispatch").unwrap();
        let vkCmdDispatch: PFN_vkCmdDispatch = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkQueueSubmit").unwrap();
        let vkQueueSubmit: PFN_vkQueueSubmit = unsafe {transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};

        let name = CString::new("vkQueueWaitIdle").unwrap();
        let vkQueueWaitIdle: PFN_vkQueueWaitIdle = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};

        let name = CString::new("vkCreateFence").unwrap();
        let vkCreateFence: PFN_vkCreateFence = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};

        let name = CString::new("vkDestroyFence").unwrap();
        let vkDestroyFence: PFN_vkDestroyFence = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};
        
        let name = CString::new("vkWaitForFences").unwrap();
        let vkWaitForFences: PFN_vkWaitForFences = unsafe { transmute(
            (vk_instance.vkGetDeviceProcAddr)(device, name.as_ptr()).unwrap())};

        VulkanDevice {  
            vk_instance,
            device,
            queue,
            queue_family_index,
            vkDestroyDevice,
            vkCreateCommandPool,
            vkDestroyCommandPool,
            vkAllocateCommandBuffers,
            vkCreateShaderModule,
            vkDestroyShaderModule,
            vkCreateDescriptorSetLayout,
            vkDestroyDescriptorSetLayout,
            vkCreatePipelineLayout,
            vkDestroyPipelineLayout,
            vkCreateComputePipelines,
            vkDestroyPipeline,
            vkCreateBuffer,
            vkDestroyBuffer,
            vkCreateDescriptorPool,
            vkDestroyDescriptorPool,
            vkAllocateDescriptorSets,
            vkUpdateDescriptorSets,
            vkGetBufferMemoryRequirements,
            vkAllocateMemory,
            vkFreeMemory,
            vkBindBufferMemory,
            vkMapMemory,
            vkUnmapMemory,
            vkBeginCommandBuffer,
            vkEndCommandBuffer,
            vkCmdBindPipeline,
            vkCmdBindDescriptorSets,
            vkCmdDispatch,
            vkQueueSubmit,
            vkQueueWaitIdle,
            vkCreateFence,
            vkDestroyFence,
            vkWaitForFences
        }

    }

    pub fn create_command_pool(&self) -> VkCommandPool {
        let pool_info = VkCommandPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            queueFamilyIndex: self.queue_family_index,
        };

        let mut command_pool: VkCommandPool = 0;
        let res = unsafe {(self.vkCreateCommandPool)(self.device, &pool_info, ptr::null(), &mut command_pool)};
        assert_eq!(res, VK_SUCCESS, "vkCreateCommandPool failed: {}", res);
        command_pool
    }

    pub fn destroy_command_pool(&self, command_pool: VkCommandPool) {
         unsafe {(self.vkDestroyCommandPool)(self.device, command_pool, ptr::null()) };
    }
    
    pub fn allocate_command_buffers(&self, command_pool: VkCommandPool) -> VkCommandBuffer {
        let buffer_alocate_info = VkCommandBufferAllocateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: ptr::null(),
            commandPool: command_pool,
            level: VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
        };

        let mut cmd_buf: VkCommandBuffer = ptr::null_mut();
        let result = unsafe {(self.vkAllocateCommandBuffers)(self.device, &buffer_alocate_info, &mut cmd_buf)};
        assert_eq!(result, VK_SUCCESS, "Failed to allocate command buffer");
        cmd_buf
    }

    pub fn create_shader(&self, code: &Vec<u32> ) -> VkShaderModule {
         let create_info = VkShaderModuleCreateInfo {
            sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            codeSize: code.len() * 4,
            pCode: code.as_ptr(),
        };

        let mut shader_module: VkShaderModule = 0;
        let result = unsafe {(self.vkCreateShaderModule)(self.device, &create_info, ptr::null(), &mut shader_module)};
        assert_eq!(result, VK_SUCCESS, "Failed to create shader module");
        shader_module
    }

    pub fn destroy_shader(&self, shader_module: VkShaderModule) {
         unsafe {(self.vkDestroyShaderModule)(self.device, shader_module, ptr::null()) };
    }

    pub fn create_descriptor_set_layout(&self) -> VkDescriptorSetLayout {
         // Binding for our storage buffer (set=0, binding=0)
        let binding = VkDescriptorSetLayoutBinding {
            binding: 0,
            descriptorType: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            descriptorCount: 1,
            stageFlags: VK_SHADER_STAGE_COMPUTE_BIT as u32,
            pImmutableSamplers: ptr::null(),
        };

        let set_layout_info = VkDescriptorSetLayoutCreateInfo {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            bindingCount: 1,
            pBindings: &binding,
        };

         let mut descriptor_set_layout: VkDescriptorSetLayout = 0;
         let result = unsafe {
            (self.vkCreateDescriptorSetLayout)(self.device, &set_layout_info, ptr::null(), &mut descriptor_set_layout)
        };
        assert_eq!(result, VK_SUCCESS, "Failed to create descriptor set layout");
        descriptor_set_layout
    }

    pub fn destroy_descriptor_set_layout(&self, descriptor_set_layout: VkDescriptorSetLayout) {
         unsafe {(self.vkDestroyDescriptorSetLayout)(self.device, descriptor_set_layout, ptr::null()) };
    }

    pub fn create_pipeline_layout(&self, descriptor_set_layout: VkDescriptorSetLayout) -> VkPipelineLayout {
        let pipeline_layout_info = VkPipelineLayoutCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            setLayoutCount: 1,
            pSetLayouts: &descriptor_set_layout,
            pushConstantRangeCount: 0,
            pPushConstantRanges: ptr::null(),
        };

        let mut pipeline_layout: VkPipelineLayout = 0;
        let result = unsafe {
            (self.vkCreatePipelineLayout)(self.device, &pipeline_layout_info, ptr::null(), &mut pipeline_layout)
        };
        assert_eq!(result, VK_SUCCESS, "Failed to create pipeline layout");
        pipeline_layout
    }

    pub fn destroy_pipeline_layout(&self, pipeline_layout: VkPipelineLayout) {
         unsafe {(self.vkDestroyPipelineLayout)(self.device, pipeline_layout, ptr::null()) };
    }

    pub fn create_compute_pipline(&self, shader_module: VkShaderModule, pipeline_layout: VkPipelineLayout) -> VkPipeline {
        let entry_name = CString::new("main").unwrap();
        let shader_stage = VkPipelineShaderStageCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            stage: VK_SHADER_STAGE_COMPUTE_BIT,
            module: shader_module,
            pName: entry_name.as_ptr(),
            pSpecializationInfo: ptr::null(),
        };
        
        let pipeline_info = VkComputePipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            stage: shader_stage,
            layout: pipeline_layout,
            basePipelineHandle: 0,
            basePipelineIndex: -1,
        };

        let mut pipeline: VkPipeline = 0;
        let result = unsafe {
            (self.vkCreateComputePipelines)(self.device, 0, 1, &pipeline_info, ptr::null(), &mut pipeline)
        };
        assert_eq!(result, VK_SUCCESS, "Failed to create pipeline layout");
        pipeline

    }

    pub fn destroy_pipeline(&self, pipeline: VkPipeline) {
         unsafe {(self.vkDestroyPipeline)(self.device, pipeline, ptr::null()) };
    }

    pub fn create_buffer(&self, size: VkDeviceSize) -> VkBuffer {
        // required for compute shader
        const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: u32 = 0x00000020;
        
        let buffer_info = VkBufferCreateInfo {
            sType: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            size: size,
            usage: VK_BUFFER_USAGE_STORAGE_BUFFER_BIT, 
            sharingMode: VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: ptr::null(),
        };

        let mut buffer: VkBuffer = 0;
        let result = unsafe {
            (self.vkCreateBuffer)(self.device, &buffer_info, ptr::null(), &mut buffer)
        };
        assert_eq!(result, VK_SUCCESS, "Failed to create buffer");
        buffer
    }

    pub fn destroy_buffer(&self, buffer: VkBuffer) {
         unsafe { (self.vkDestroyBuffer)(self.device, buffer, ptr::null()) };
    }

    pub fn create_descriptor_pool(&self) -> VkDescriptorPool {
        let pool_size = VkDescriptorPoolSize {
            type_: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            descriptorCount: 1,
        };
        
        let pool_info = VkDescriptorPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            maxSets: 1,
            poolSizeCount: 1,
            pPoolSizes: &pool_size,
        };

        let mut descriptor_pool: VkDescriptorPool = 0;
        unsafe {
            (self.vkCreateDescriptorPool)(self.device, &pool_info, ptr::null(), &mut descriptor_pool);
        }
        descriptor_pool
    }

    pub fn destroy_descriptor_pool(&self, descriptor_pool: VkDescriptorPool) {
         unsafe { (self.vkDestroyDescriptorPool)(self.device, descriptor_pool, ptr::null()) };
    }

    pub fn allocate_descriptor_set(&self, descriptor_pool: VkDescriptorPool, descriptor_set_layout: VkDescriptorSetLayout) -> VkDescriptorSet {
        let alloc_info = VkDescriptorSetAllocateInfo {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
            pNext: ptr::null(),
            descriptorPool: descriptor_pool,
            descriptorSetCount: 1,
            pSetLayouts: &descriptor_set_layout,
        };

        let mut descriptor_set: VkDescriptorSet = 0;
        unsafe {(self.vkAllocateDescriptorSets)(self.device, &alloc_info, &mut descriptor_set);}
        descriptor_set
    }

    pub fn update_descriptor_sets(&self, buffer: VkBuffer, descriptor_set: VkDescriptorSet) {
        
        const VK_WHOLE_SIZE: u64 = u64::MAX;
    
        let buffer_info = VkDescriptorBufferInfo {
            buffer: buffer, // VkBuffer from vkCreateBuffer
            offset: 0,
            range: VK_WHOLE_SIZE, // or size of your buffer
        };

        let write = VkWriteDescriptorSet {
            sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            pNext: ptr::null(),
            dstSet: descriptor_set,
            dstBinding: 0,
            dstArrayElement: 0,
            descriptorCount: 1,
            descriptorType: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            pImageInfo: ptr::null(),
            pBufferInfo: &buffer_info,
            pTexelBufferView: ptr::null(),
        };

        unsafe {(self.vkUpdateDescriptorSets)(self.device, 1, &write, 0, ptr::null());}
    }

    pub fn get_buffer_memory_requirements(&self, buffer: VkBuffer) -> VkMemoryRequirements {
        let mut mem_requirements = VkMemoryRequirements {
            size: 0,
            alignment: 0,
            memoryTypeBits: 0,
        };

        unsafe {(self.vkGetBufferMemoryRequirements)(self.device, buffer, &mut mem_requirements);}
        mem_requirements
    }

    pub fn allocate_memory(&self, memory_type_index: u32, mem_requirements: VkMemoryRequirements) -> VkDeviceMemory {        
        let alloc_info = VkMemoryAllocateInfo {
            sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            pNext: ptr::null(),
            allocationSize: mem_requirements.size,
            memoryTypeIndex: memory_type_index,
        };

        let mut buffer_memory: VkDeviceMemory = 0;
        let result = unsafe {
            (self.vkAllocateMemory)(self.device, &alloc_info, ptr::null(), &mut buffer_memory)
         };
         assert_eq!(result, VK_SUCCESS, "Failed to allocate buffer memory");
         buffer_memory
    }

    pub fn free_memory(&self, device_memory: VkDeviceMemory) {
        unsafe { (self.vkFreeMemory)(self.device, device_memory, ptr::null()) };
    }

    pub fn bind_buffer_memory(&self, buffer: VkBuffer, device_memory: VkDeviceMemory) {
        let result = unsafe {
            (self.vkBindBufferMemory)(self.device, buffer, device_memory, 0)
        };
        assert_eq!(result, VK_SUCCESS, "Failed to bind buffer memory");
    }

    pub fn fill_input_buffer(&self, buffer_memory: VkDeviceMemory, mem_requirements: VkMemoryRequirements) {
        let mut data: *mut std::ffi::c_void = std::ptr::null_mut();
        unsafe {
            (self.vkMapMemory)(self.device, buffer_memory, 0, mem_requirements.size, 0, &mut data);
            let slice = std::slice::from_raw_parts_mut(data as *mut u32, 256);
            for i in 0..256 {
                slice[i] = i as u32; // fill input buffer
            }
            (self.vkUnmapMemory)(self.device, buffer_memory);
        }
    }
    pub fn begin_command_buffer(&self, cmd_buf: VkCommandBuffer) {
        let begin_info = VkCommandBufferBeginInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: ptr::null(),
            flags: 0,
            pInheritanceInfo: ptr::null(),
        };

        let result = unsafe {(self.vkBeginCommandBuffer)(cmd_buf, &begin_info)};
        assert_eq!(result, VK_SUCCESS, "Failed to begin command buffer");
    }

    pub fn end_command_buffer(&self, cmd_buf: VkCommandBuffer) {
        let result = unsafe {(self.vkEndCommandBuffer)(cmd_buf)};
        assert_eq!(result, VK_SUCCESS, "Failed to end command buffer");
    }

    pub fn cmd_bind_pipeline(&self, cmd_buf: VkCommandBuffer, compute_pipeline: VkPipeline) {
        unsafe {(self.vkCmdBindPipeline)(cmd_buf, VK_PIPELINE_BIND_POINT_COMPUTE, compute_pipeline)};
    }

    pub fn cmd_bind_descriptor_sets(&self, cmd_buf: VkCommandBuffer, pipeline_layout: VkPipelineLayout, descriptor_set: VkDescriptorSet) {

        let desc_sets: [VkDescriptorSet; 1] = [descriptor_set];

        unsafe {(self.vkCmdBindDescriptorSets)(
            cmd_buf,
            VK_PIPELINE_BIND_POINT_COMPUTE,
            pipeline_layout,
            0, // firstSet
            1, // descriptorSetCount
            //&descriptor_set,
            desc_sets.as_ptr(),
            0, // dynamicOffsetCount
            std::ptr::null(),
        )};
    }

    pub fn cmd_dispatch(&self, cmd_buf: VkCommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {(self.vkCmdDispatch)(cmd_buf, group_count_x, group_count_y, group_count_z)};
    }

    pub fn queue_submit(&self, queue: VkQueue, cmd_buf: VkCommandBuffer) {
        let submit_info = VkSubmitInfo {
            sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: ptr::null(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: ptr::null(),
            pWaitDstStageMask: ptr::null(),
            commandBufferCount: 1,
            pCommandBuffers: &cmd_buf,
            signalSemaphoreCount: 0,
            pSignalSemaphores: ptr::null(),
        };

        let fence: VkFence = 0;
        let res = unsafe {(self.vkQueueSubmit)(queue, 1, &submit_info, fence)};
        assert_eq!(res, VK_SUCCESS, "vkQueueSubmit failed");
    }

    pub fn queue_submit_with_fence(&self, queue: VkQueue, cmd_buf: VkCommandBuffer, fence: VkFence) {
        let submit_info = VkSubmitInfo {
            sType: VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: ptr::null(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: ptr::null(),
            pWaitDstStageMask: ptr::null(),
            commandBufferCount: 1,
            pCommandBuffers: &cmd_buf,
            signalSemaphoreCount: 0,
            pSignalSemaphores: ptr::null(),
        };

        let res = unsafe {(self.vkQueueSubmit)(queue, 1, &submit_info, fence)};
        assert_eq!(res, VK_SUCCESS, "vkQueueSubmit failed");
    }

    pub fn queue_wait_idle(&self, queue: VkQueue) {
        let res = unsafe {(self.vkQueueWaitIdle)(queue)};
        assert_eq!(res, VK_SUCCESS, "vkQueueWaitIdle failed");
    }

    pub fn create_fence(&self) -> VkFence{
        let fence_info = VkFenceCreateInfo {
            sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0, // 0 = unsignaled, VK_FENCE_CREATE_SIGNALED_BIT if you want signaled initially
        };

        let mut fence: VkFence = 0;
        let r = unsafe {(self.vkCreateFence)(self.device, &fence_info, std::ptr::null(), &mut fence)};
        assert_eq!(r, VK_SUCCESS, "vkCreateFence failed");
        fence
    }

    pub fn destroy_fence(&self, fence: VkFence) {
        unsafe {(self.vkDestroyFence)(self.device, fence, std::ptr::null())};
    }

    pub fn wait_for_fences(&self, fence: VkFence) {
        // timeout
        let result = unsafe {(self.vkWaitForFences)(self.device, 1, &fence, 1, 10000000)};
        assert_eq!(result, VK_SUCCESS, "vkWaitForFences failed or timed out");
    }

    pub fn print_input_buffer(&self, buffer_memory: VkDeviceMemory, mem_requirements: VkMemoryRequirements) {
        let mut data: *mut std::ffi::c_void = std::ptr::null_mut();
        unsafe {
            (self.vkMapMemory)(self.device, buffer_memory, 0, mem_requirements.size, 0, &mut data);
            let slice = std::slice::from_raw_parts_mut(data as *mut u32, 256);
            for i in 0..32 {
                println!("{}", slice[i]);
                // slice[i] = i as u32; // fill input buffer
            }
            // let data1 = data as *const u32;
            // for i in 0..256 {
            //     println!("Result[{}] = {}", i, *data1.add(i));
            // }
            (self.vkUnmapMemory)(self.device, buffer_memory);
        }
    }

}

impl Drop for VulkanDevice {
    fn drop(&mut self) {
        unsafe {(self.vkDestroyDevice)(self.device, ptr::null()) };
    }
}
