use std::ptr;
use crate::vk_types::VkResult;
use crate::vk_handles::VkInstance;

pub type VkStructureType = i32;
pub const VK_SUCCESS: VkResult = 0;
pub const VK_NULL_HANDLE: VkInstance = ptr::null_mut();

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;

pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = 0;
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = 1;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = 2;
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = 3;
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = 4;
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = 5;
pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = 8;
pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = 12;
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = 16;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType = 18;
pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = 29;
pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = 30;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType = 32;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = 33;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = 34;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = 35;
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = 39;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = 40;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 42;


pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: u32 = 1;
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: u32 = 2;
pub const VK_COMMAND_POOL_CREATE_PROTECTED_BIT: u32 = 4;

pub type VkCommandBufferLevel = i32;
pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = 0;
pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: VkCommandBufferLevel = 1;


pub type VkShaderStageFlagBits = i32;
pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlagBits = 0x00000001;
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlagBits = 0x00000002;
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlagBits = 0x00000004;
pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlagBits  = 0x00000008;
pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlagBits = 0x00000010;
pub const VK_SHADER_STAGE_COMPUTE_BIT:  VkShaderStageFlagBits = 0x00000020;
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlagBits = 0x0000001F;
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlagBits = 0x7FFFFFFF;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_RAYGEN_BIT_KHR: VkShaderStageFlagBits = 0x00000100;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_ANY_HIT_BIT_KHR: VkShaderStageFlagBits = 0x00000200;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR: VkShaderStageFlagBits = 0x00000400;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_MISS_BIT_KHR: VkShaderStageFlagBits = 0x00000800;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_INTERSECTION_BIT_KHR:  VkShaderStageFlagBits = 0x00001000;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_SHADER_STAGE_CALLABLE_BIT_KHR: VkShaderStageFlagBits = 0x00002000;
// Provided by VK_EXT_mesh_shader
pub const VK_SHADER_STAGE_TASK_BIT_EXT: VkShaderStageFlagBits = 0x00000040;
// Provided by VK_EXT_mesh_shader
pub const VK_SHADER_STAGE_MESH_BIT_EXT: VkShaderStageFlagBits = 0x00000080;
// Provided by VK_HUAWEI_subpass_shading
pub const VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI: VkShaderStageFlagBits = 0x00004000;
// Provided by VK_HUAWEI_cluster_culling_shader
pub const VK_SHADER_STAGE_CLUSTER_CULLING_BIT_HUAWEI: VkShaderStageFlagBits = 0x00080000;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_RAYGEN_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_RAYGEN_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_ANY_HIT_BIT_NV: VkShaderStageFlagBits  = VK_SHADER_STAGE_ANY_HIT_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_MISS_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_MISS_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_INTERSECTION_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_INTERSECTION_BIT_KHR;
// Provided by VK_NV_ray_tracing
pub const VK_SHADER_STAGE_CALLABLE_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_CALLABLE_BIT_KHR;
// Provided by VK_NV_mesh_shader
pub const VK_SHADER_STAGE_TASK_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_TASK_BIT_EXT;
// Provided by VK_NV_mesh_shader
pub const VK_SHADER_STAGE_MESH_BIT_NV: VkShaderStageFlagBits = VK_SHADER_STAGE_MESH_BIT_EXT;


pub type VkDescriptorType = i32;
pub const VK_DESCRIPTOR_TYPE_SAMPLER: VkDescriptorType = 0;
pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: VkDescriptorType = 1;
pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: VkDescriptorType = 2;
pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: VkDescriptorType = 3;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: VkDescriptorType = 4;
pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: VkDescriptorType = 5;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: VkDescriptorType = 6;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: VkDescriptorType = 7;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: VkDescriptorType = 8;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: VkDescriptorType = 9;
pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: VkDescriptorType = 10;
// Provided by VK_VERSION_1_3
pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK: VkDescriptorType = 1000138000;
// Provided by VK_KHR_acceleration_structure
pub const VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR: VkDescriptorType = 1000150000;
// Provided by VK_NV_ray_tracing
pub const VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV: VkDescriptorType = 1000165000;
// Provided by VK_QCOM_image_processing
pub const VK_DESCRIPTOR_TYPE_SAMPLE_WEIGHT_IMAGE_QCOM: VkDescriptorType = 1000440000;
// Provided by VK_QCOM_image_processing
pub const VK_DESCRIPTOR_TYPE_BLOCK_MATCH_IMAGE_QCOM: VkDescriptorType = 1000440001;
// Provided by VK_ARM_tensors
pub const VK_DESCRIPTOR_TYPE_TENSOR_ARM: VkDescriptorType = 1000460000;
// Provided by VK_EXT_mutable_descriptor_type
pub const VK_DESCRIPTOR_TYPE_MUTABLE_EXT: VkDescriptorType = 1000351000;
// Provided by VK_NV_partitioned_acceleration_structure
pub const VK_DESCRIPTOR_TYPE_PARTITIONED_ACCELERATION_STRUCTURE_NV: VkDescriptorType = 1000570000;
// Provided by VK_EXT_inline_uniform_block
pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT: VkDescriptorType = VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK;
// Provided by VK_VALVE_mutable_descriptor_type
pub const VK_DESCRIPTOR_TYPE_MUTABLE_VALVE: VkDescriptorType = VK_DESCRIPTOR_TYPE_MUTABLE_EXT;

pub type VkPipelineLayoutCreateFlagBits = i32;
pub const VK_PIPELINE_LAYOUT_CREATE_INDEPENDENT_SETS_BIT_EXT: VkPipelineLayoutCreateFlagBits  = 0x00000002;

pub type VkSharingMode = i32;
pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = 0;
pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = 1;


pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;


pub type VkMemoryPropertyFlags = u32;

pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlags = 0x00000001;
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlags = 0x00000002;
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlags = 0x00000004;
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlags = 0x00000008;
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlags = 0x00000010;
pub const VK_MEMORY_PROPERTY_PROTECTED_BIT: VkMemoryPropertyFlags = 0x00000020;

// Vendor-specific (AMD, NVIDIA) - optional
pub const VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD: VkMemoryPropertyFlags = 0x00000040;
pub const VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD: VkMemoryPropertyFlags = 0x00000080;
pub const VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV: VkMemoryPropertyFlags = 0x00000100;


pub type VkPipelineBindPoint = i32;
pub const VK_PIPELINE_BIND_POINT_GRAPHICS: VkPipelineBindPoint = 0;
pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint  = 1;
// Provided by VK_KHR_ray_tracing_pipeline
pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR: VkPipelineBindPoint = 1000165000;
// Provided by VK_HUAWEI_subpass_shading
pub const VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI: VkPipelineBindPoint = 1000369003;
// Provided by VK_ARM_data_graph
pub const VK_PIPELINE_BIND_POINT_DATA_GRAPH_ARM: VkPipelineBindPoint = 1000507000;
// Provided by VK_NV_ray_tracing
pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_NV: VkPipelineBindPoint = VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR;
