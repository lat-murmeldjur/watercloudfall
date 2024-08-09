//! Simple winit application.
#![allow(warnings)]

mod model;
use crate::model::{Normal, Position, INDICES, NORMALS, POSITIONS};

use glam::{
    f32::{Mat3, Vec3},
    Mat4,
};
use std::{error::Error, sync::Arc, time::Instant};
use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
        Buffer, BufferCreateInfo, BufferUsage,
    },
    command_buffer::{
        allocator::StandardCommandBufferAllocator, CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsage, RecordingCommandBuffer, RenderPassBeginInfo,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, DescriptorSet, WriteDescriptorSet,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, DeviceOwned,
        QueueCreateInfo, QueueFlags,
    },
    format::Format,
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            depth_stencil::{DepthState, DepthStencilState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::EntryPoint,
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};

use std::thread;
#[cfg(not(web_platform))]
use std::time;

use ::tracing::{info, warn};
#[cfg(web_platform)]
use web_time as time;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Fullscreen, Window, WindowId};

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();

    event_loop.run_app(Flow::default())
}

#[derive(Default)]
struct Flow {
    close_requested: bool,
    window: Option<Window>,
}

impl ApplicationHandler for Flow {
    fn new_events(&mut self, _event_loop: &dyn ActiveEventLoop, cause: StartCause) {
        println!("new_events: {cause:?}");
    }

    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("watercloudfall")
            .with_fullscreen(Some(Fullscreen::Borderless(None)));
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        _event_loop: &dyn ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        println!("{event:?}");

        match event {
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key.as_ref() {
                // WARNING: Consider using `key_without_modifiers()` if available on your platform.
                // See the `key_binding` example
                Key::Character("1") => {
                    // self.mode = Mode::Wait;
                    println!("pressed: 1");
                }
                Key::Named(NamedKey::Escape) => {
                    self.close_requested = true;
                }
                _ => (),
            },
            WindowEvent::RedrawRequested => {
                if self.close_requested {
                    _event_loop.exit();
                }

                let window = self.window.as_ref().unwrap();
                window.pre_present_notify();
            }
            _ => (),
        }

        self.window.as_ref().unwrap().request_redraw();
    }
}
