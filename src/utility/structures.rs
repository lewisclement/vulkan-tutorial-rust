
use ash;
use ash::vk;
use cgmath::Matrix4;

pub struct DeviceExtension {
    pub names: [&'static str; 1],
//    pub raw_names: [*const i8; 1],
}

impl DeviceExtension {
    pub fn get_raw_names(&self) -> Vec<*const i8> {
        self.names.iter()
            .map(|name| super::tools::vk_to_raw_string(*name).as_ptr())
            .collect()
    }
}

pub struct SurfaceStuff {
    pub surface_loader: ash::extensions::Surface,
    pub surface: vk::SurfaceKHR,

    pub screen_width: u32,
    pub screen_height: u32,
}
pub struct SwapChainStuff {
    pub swapchain_loader: ash::extensions::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
}

pub struct SwapChainSupportDetail {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

pub struct QueueFamilyIndices {
    pub graphics_family: i32,
    pub present_family:  i32,
}

impl QueueFamilyIndices {

    pub fn new() -> QueueFamilyIndices {
        QueueFamilyIndices {
            graphics_family: -1,
            present_family:  -1,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family >= 0 && self.present_family >= 0
    }
}

pub struct SyncObjects {
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub inflight_fences: Vec<vk::Fence>,
}

#[derive(Clone, Debug, Copy)]
pub struct UniformBufferObject {
    pub model: Matrix4<f32>,
    pub view:  Matrix4<f32>,
    pub proj:  Matrix4<f32>,
}

#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}
impl Vertex {

    pub fn get_binding_description() -> [vk::VertexInputBindingDescription; 1] {
        [
            vk::VertexInputBindingDescription {
                binding: 0,
                stride: ::std::mem::size_of::<Vertex>() as u32,
                input_rate: vk::VertexInputRate::Vertex,
            },
        ]
    }

    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                binding:  0,
                location: 0,
                format: vk::Format::R32g32Sfloat,
                offset: offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                binding:  0,
                location: 1,
                format: vk::Format::R32g32b32a32Sfloat,
                offset: offset_of!(Vertex, color) as u32,
            }
        ]
    }
}

pub const RECT_VERTICES_DATA: [Vertex; 4] = [
    Vertex { pos: [-0.5, -0.5], color: [1.0, 0.0, 0.0, 1.0], },
    Vertex { pos: [ 0.5, -0.5], color: [0.0, 1.0, 0.0, 1.0], },
    Vertex { pos: [ 0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0], },
    Vertex { pos: [-0.5,  0.5], color: [1.0, 1.0, 1.0, 1.0], },
];
pub const RECT_INDICES_DATA: [vk::types::uint32_t; 6] = [
    0, 1, 2, 2, 3, 0
];
