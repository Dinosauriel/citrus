#[cfg(windows)]
extern crate gfx_backend_dx12 as backend;
#[cfg(target_os = "macos")]
extern crate gfx_backend_metal as backend;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate gfx_backend_vulkan as backend;

extern crate noise;
extern crate gfx_hal;
extern crate winit;


use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowBuilder, WindowEvent};
use gfx_hal::{
    command::{ClearColor, ClearValue},
    format::{Aspects, ChannelType, Format, Swizzle},
    image::{Access, Layout, SubresourceRange, ViewKind},
    pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
        SubpassDesc, SubpassRef,
    },
    pool::CommandPoolCreateFlags,
    pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, Viewport,
    },
    queue::Submission,
    Backbuffer, Device, FrameSync, Graphics, Instance, Primitive, Surface, SwapImageIndex,
    Swapchain, SwapchainConfig,
};

//use noise::{NoiseFn, Perlin};

//static TICK_RATE: i64 = 64; //updates of the game logic per second
//static INGAME_TICK_RATE: i64 = 64; //units of ingame time per second. influences game speed

fn main() {
	let mut events_loop = EventsLoop::new();

	let window = WindowBuilder::new()
		.with_title("Part 00: Triangle")
		.with_dimensions((256, 256).into())
		.build(&events_loop)
		.unwrap();

	let instance = backend::Instance::create("Part 00: Triangle", 1);

	let mut surface = instance.create_surface(&window);

	let mut adapter = instance.enumerate_adapters().remove(0);
}