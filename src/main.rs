use inputbot::{KeybdKey::*, MouseButton::*};
extern crate glium;

fn main() {
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    WKey.bind(|| {
        println!("Forward key has been pressed");
    });
    LeftButton.bind(|| println!("LMB has been pressed"));
    inputbot::handle_input_events();
    events_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
