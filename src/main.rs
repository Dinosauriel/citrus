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

	let num_queues = 1;
	let (device, mut queue_group) = adapter
		.open_with::<_, Graphics>(num_queues, |family| surface.supports_queue_family(family))
		.unwrap();

	let max_buffers = 16;
	unsafe {
		let mut command_pool = device.create_command_pool_typed(
			&queue_group,
			CommandPoolCreateFlags::empty()
		);
	}

	let physical_device = &adapter.physical_device;

	let (caps, formats, _, _) = surface.compatibility(physical_device);

	let surface_color_format = {
		// We must pick a color format from the list of supported formats. If there
		// is no list, we default to Rgba8Srgb.
		match formats {
			Some(choices) => choices
				.into_iter()
				.find(|format| format.base_format().1 == ChannelType::Srgb)
				.unwrap(),
			None => Format::Rgba8Srgb,
		}
	};
let render_pass = {
		let color_attachment = Attachment {
			format: Some(surface_color_format),
			samples: 1,
			ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
			stencil_ops: AttachmentOps::DONT_CARE,
			layouts: Layout::Undefined..Layout::Present,
		};

		// A render pass could have multiple subpasses - but we're using one for now.
		let subpass = SubpassDesc {
			colors: &[(0, Layout::ColorAttachmentOptimal)],
			depth_stencil: None,
			inputs: &[],
			resolves: &[],
			preserves: &[],
		};

		// This expresses the dependencies between subpasses. Again, we only have
		// one subpass for now. Future tutorials may go into more detail.
		let dependency = SubpassDependency {
			passes: SubpassRef::External..SubpassRef::Pass(0),
			stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
			accesses: Access::empty()
				..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
		};

		unsafe {
			device.create_render_pass(&[color_attachment], &[subpass], &[dependency])
		}
	};
}