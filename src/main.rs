extern crate glfw;
extern crate gl;

use gl::types::*;
use glfw::{Action, Context, Key};
use noise::{NoiseFn, Perlin};

static TICK_RATE: i64 = 64; //updates of the game logic per second
static INGAME_TICK_RATE: i64 = 64; //units of ingame time per second. influences game speed

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(1500, 950, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();


    let perlin = Perlin::new();
    let val = perlin.get([42.4, 37.7, 2.8]);

    let gl = gl::load_with(|s| window.get_proc_address(s));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    while !window.should_close() {
        glfw.poll_events();

        unsafe {
            gl::Viewport(0, 0, 900, 700); // set viewport
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        window.swap_buffers();
    }
}