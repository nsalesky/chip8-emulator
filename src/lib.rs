mod constants;
mod errors;

use std::time::Duration;

use anyhow::Result;
use constants::{BACKGROUND_COLOR, CANVAS_HEIGHT, CANVAS_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::{event::Event, hint, keyboard::Keycode, pixels::Color};

pub fn run() -> Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("chip8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }

    Ok(())
}
