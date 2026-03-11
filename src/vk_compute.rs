use crate::vk::load_vulkan_dyn_library;
use crate::vk::VulkanLib;
use crate::vk::VulkanInstance;
use crate::vk::VulkanDevice;
use crate::vk_handles::VkShaderModule;
use crate::vk::read_spv_shader;

pub struct VulkanComputeDevice {
    vk_device: VulkanDevice,
    shader_modules: Vec<VkShaderModule>,
    layout_sets: Vec<u32>,
}

impl VulkanComputeDevice {
    pub fn new(vk_device: VulkanDevice) -> VulkanComputeDevice {
        VulkanComputeDevice {
            vk_device,
            shader_modules: Vec::new(),
            layout_sets: Vec::new(),
        }
    }

    pub fn add_shader_from_code(&mut self, code: &Vec<u32>) {
        let shader_module = self.vk_device.create_shader(code);
        self.shader_modules.push(shader_module);
    }

    pub fn add_shader_from_file(&mut self, path: &str) {
        let code = read_spv_shader(path);
        self.add_shader_from_code(&code);
    }

    pub fn add_layout_set(&mut self, number_of_bindings: u32) {
        self.layout_sets.push(number_of_bindings);
    }

    pub fn prepare(&mut self) {
        
    }
}

impl Drop for VulkanComputeDevice {
    fn drop(&mut self) {
        for shader_module in self.shader_modules.clone() {
            self.vk_device.destroy_shader(shader_module);
        }
    }
}

pub fn make_vk_compute_device() -> Result<VulkanComputeDevice, String>{

    let lib = match load_vulkan_dyn_library() {
        Some(lib) => lib,
        None => return Err("Failed to load vulkan library".to_string()),
    };

    let vulkan_library = VulkanLib::new(lib);
    let instance = vulkan_library.create_instance();

    let vk_instance = VulkanInstance::new(vulkan_library, instance);
    let physical_devices = vk_instance.enumerate_physical_devices();
    if physical_devices.is_empty() {
        return Err("No physical devices found".to_string());
    }

    let (device, queue, queue_family_index) = match vk_instance.create_best_compute_device(&physical_devices) {
        Some((device, queue, queue_family_index)) => (device, queue, queue_family_index),
        None => return Err("No compute device found".to_string()),
    };

    let vk_device = VulkanDevice::new(vk_instance, device, queue, queue_family_index);

    Ok(VulkanComputeDevice::new(vk_device))

}
