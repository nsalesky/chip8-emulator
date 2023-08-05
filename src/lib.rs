mod constants;
mod errors;
mod instruction_parser;
mod virtual_computer;

use std::time::Duration;

use anyhow::Result;
use constants::{BACKGROUND_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::{event::Event, keyboard::Keycode};
use virtual_computer::VirtualComputer;

pub fn run() -> Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("chip8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(*BACKGROUND_COLOR);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut vc = VirtualComputer::default();

    'running: loop {
        // 1. Input
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

        // 2. Update
        let instr = vc.fetch_instruction_and_increment_pc();

        // 3. Render
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }

    Ok(())
}
