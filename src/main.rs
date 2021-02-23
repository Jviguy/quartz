use inputbot::{KeybdKey::*, MouseButton::*};
extern crate glium;

fn main() {
    let events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    WKey.bind(|| {
        println!("Forward Key has been pressed!");
    });
    RightButton.bind(|| {
        println!("Right mouse button has been clicked!");
    });
    inputbot::handle_input_events();
}
