use rtlib::vk::load_vulkan_dyn_library;
use rtlib::vk::{VulkanLib, VulkanInstance, VulkanDevice, read_spv_shader, find_memory_type};
use rtlib::vk_const::{VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT, VK_MEMORY_PROPERTY_HOST_COHERENT_BIT};
use rtlib::vk_compute::make_vk_compute_device;


fn main() {
    simple_vulkan_shader();
}


fn simple_vulkan_shader() {
    // 1. create vulkan_compute_device
    let mut vulkan_compute_device = make_vk_compute_device().unwrap();
    // 2. add shaders to vulkan compute device
    vulkan_compute_device.add_shader_from_file("D:\\rust_projects\\rtlib\\shaders\\multiply.spv");
    // 3. add shader layout description
    // add_layout_description(&mut self, binding: u32,
    // descriptor_type: VkDescriptorType, stage_flags: VkShaderStageFlags)

    let lib = load_vulkan_dyn_library().unwrap();
    let vulkan_library = VulkanLib::new(lib);
    let instance = vulkan_library.create_instance();
    for extension in vulkan_library.enumerate_extensions() {
        println!("{}", extension);
    }
    println!("Created VkInstance = {:?}", instance);
    let vk_instance = VulkanInstance::new(vulkan_library, instance);
    let physical_devices = vk_instance.enumerate_physical_devices();
    let mem_properties = vk_instance.get_physical_device_memory_properties(physical_devices[0]);
    //println!("Memory properties = {:?}", mem_properties);
    let (device, queue, queue_family_index) = vk_instance.create_best_compute_device(&physical_devices).unwrap();
    println!("Logical device = {:?}", device);
    println!("Queue = {:?}", queue);
    let vk_device = VulkanDevice::new(vk_instance, device, queue, queue_family_index);
    let command_pool = vk_device.create_command_pool();
    println!("Command pool ={:#x}", command_pool);
    let command_buffer = vk_device.allocate_command_buffers(command_pool);
    println!("Command buffer = {:?}", command_buffer);

    let code = read_spv_shader("D:\\rust_projects\\rtlib\\shaders\\multiply.spv");
    let shader_module = vk_device.create_shader(&code);
    println!("Shader module {:#x}", shader_module);

    let descriptor_set_layout = vk_device.create_descriptor_set_layout();
    println!("Descriptor set layout {:#x}", descriptor_set_layout);

    let pipeline_layout = vk_device.create_pipeline_layout(descriptor_set_layout);
    println!("Pipeline layout {:#x}", pipeline_layout);

    let pipeline = vk_device.create_compute_pipline(shader_module, pipeline_layout);
    println!("Pipeline {:#x}", pipeline);

    let buffer = vk_device.create_buffer(1024);
    println!("Buffer {:#x}", buffer);

    let descriptor_pool = vk_device.create_descriptor_pool();
    println!("Descriptor pool {:#x}", descriptor_pool);

    let descriptor_set = vk_device.allocate_descriptor_set(descriptor_pool, descriptor_set_layout);
    println!("Descriptor set {:#x}", descriptor_set);

    let mem_requirements = vk_device.get_buffer_memory_requirements(buffer);
    println!("memory req {:?} memoryTypeBits 0x{:x}", mem_requirements, mem_requirements.memoryTypeBits);
    println!("Number of phsical devices {}", physical_devices.len());
    //let mem_properties = vk_instance.get_physical_device_memory_properties(physical_devices[0]);
    
    // Choose memory type (e.g. HOST_VISIBLE for CPU access)
    let memory_type_index = find_memory_type(
        mem_requirements.memoryTypeBits,
        VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
        &mem_properties, // queried earlier with vkGetPhysicalDeviceMemoryProperties
    ).expect("Failed to find suitable memory type");
    println!("Memory type index {}", memory_type_index);

    let buffer_memory = vk_device.allocate_memory(memory_type_index, mem_requirements);
    vk_device.bind_buffer_memory(buffer, buffer_memory);
    vk_device.fill_input_buffer(buffer_memory, mem_requirements);

    vk_device.update_descriptor_sets(buffer, descriptor_set);

    // record commands
    vk_device.begin_command_buffer(command_buffer);
    vk_device.cmd_bind_pipeline(command_buffer, pipeline);
    vk_device.cmd_bind_descriptor_sets(command_buffer, pipeline_layout, descriptor_set);
    vk_device.cmd_dispatch(command_buffer, 2, 1, 1);
    vk_device.end_command_buffer(command_buffer);

    //let fence = vulkan_library.create_fence(instance, device);
    //println!("Fence {:#x}", fence);
    //vulkan_library.queue_submit_with_fence(instance, device, queue, command_buffer, fence);

    vk_device.queue_submit(queue, command_buffer);
    vk_device.queue_wait_idle(queue);

    //vulkan_library.wait_for_fences(instance, device, fence);
    //vulkan_library.destroy_fence(instance, device, fence);

    vk_device.print_input_buffer(buffer_memory, mem_requirements);

    vk_device.free_memory(buffer_memory);
    vk_device.destroy_buffer(buffer);
    vk_device.destroy_descriptor_pool(descriptor_pool);
    vk_device.destroy_pipeline(pipeline);
    vk_device.destroy_pipeline_layout(pipeline_layout);
    vk_device.destroy_descriptor_set_layout(descriptor_set_layout);
    vk_device.destroy_shader(shader_module);
    vk_device.destroy_command_pool(command_pool);
    //vulkan_library.destroy_device(instance, device);
    //vulkan_library.destroy_instance(instance);
    println!("Gotovo");
}
