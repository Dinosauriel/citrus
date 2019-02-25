extern crate glfw;

use glfw::{Action, Context, Key};
use noise::{NoiseFn, Perlin};

static TICK_RATE: i64 = 64; //updates of the game logic per second
static INGAME_TICK_RATE: i64 = 64; //units of ingame time per second. influences game speed

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();


    let perlin = Perlin::new();
    let val = perlin.get([42.4, 37.7, 2.8]);


    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}