mod constants;
mod errors;
mod renderer;

use anyhow::Result;
use constants::{BACKGROUND_COLOR, CANVAS_HEIGHT, CANVAS_WIDTH};
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::{Display, Surface};
use renderer::{CanvasRenderer, GLCanvas};

#[macro_use]
extern crate glium;

#[macro_use]
extern crate anyhow;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub fn run() -> Result<()> {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(640.0, 480.0))
        .with_title("Chip 8");

    let context_builder = ContextBuilder::new();

    let display = Display::new(window_builder, context_builder, &event_loop)?;

    let mut canvas = GLCanvas::new(&display)?;

    for row in 0..CANVAS_HEIGHT {
        for col in 0..CANVAS_WIDTH {
            // if row % 2 == 0 && col % 2 == 0 {
            canvas.set_pixel(row, col, true);
            // }
        }
    }

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            _ => (),
        },
        Event::RedrawEventsCleared => {
            let mut frame = display.draw();
            frame.clear_color(
                BACKGROUND_COLOR[0],
                BACKGROUND_COLOR[1],
                BACKGROUND_COLOR[2],
                1.0,
            );

            canvas.draw(&mut frame).unwrap();

            frame.finish().unwrap();
        }
        _ => (),
    });
}
