use std::num::NonZeroU32;

use pong::{
    display::{Color, Screen},
    geometry::{base::Position, shapes::Rectangle},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Theme, WindowBuilder},
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pong")
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)
        .expect("Failed to build window");
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                println!("Redraw requested.");
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                if width == 0 || height == 0 {
                    return;
                };

                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let buffer = surface.buffer_mut().unwrap();
                let mut screen = Screen::new(width as usize, height as usize, buffer);
                redraw(&mut screen);
                screen.present();
            }
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::CloseRequested,
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        };
    })
}

fn redraw(screen: &mut Screen) {
    let rect = Rectangle::from_corners(Position::new(100.0, 150.0), Position::new(200.0, 500.0));

    screen.draw(rect, Color::from_rgb(200, 240, 100))
}
