extern crate glfw;
extern crate noise;

mod util;

use glfw::{Action, Context, Key};
//use noise::{NoiseFn, Perlin};

//static TICK_RATE: i64 = 64; //updates of the game logic per second
//static INGAME_TICK_RATE: i64 = 64; //units of ingame time per second. influences game speed

fn main() {

	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	let (mut window, events) = glfw.create_window(1500, 950, "Hello this is window", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window.");

	window.set_key_polling(true);
	window.make_current();

	while !window.should_close() {
		glfw.poll_events();

		window.swap_buffers();
	}
}