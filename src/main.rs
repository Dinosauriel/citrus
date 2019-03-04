extern crate noise;
extern crate gfx;
extern crate winit;


use winit::EventsLoop;
use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

//use noise::{NoiseFn, Perlin};

//static TICK_RATE: i64 = 64; //updates of the game logic per second
//static INGAME_TICK_RATE: i64 = 64; //units of ingame time per second. influences game speed

fn main() {
	let mut events_loop = winit::EventsLoop::new();
	let window = winit::Window::new(&events_loop).unwrap();

	let mut running = true;
	while running {
		

	}
}