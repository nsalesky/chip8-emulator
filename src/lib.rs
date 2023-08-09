mod constants;
mod errors;
mod instruction_parser;
mod virtual_computer;

use std::{
    collections::HashSet,
    fs::File,
    time::{Duration, Instant},
};

use anyhow::Result;
use constants::{BACKGROUND_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH};
use instruction_parser::parse_instruction;
use sdl2::{event::Event, keyboard::Keycode};
use virtual_computer::{KeyPress, VirtualComputer};

pub fn run(rom_file: File) -> Result<()> {
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

    let mut vc = VirtualComputer::from_rom_file(rom_file)?;

    let mut keys_pressed = HashSet::new();

    let mut last_ticked = Instant::now();

    'running: loop {
        // Check if it is time to tick the timers at 60hz
        let elapsed_time = last_ticked.elapsed();
        if elapsed_time.as_millis() > (1000.0 / 60.0) as u128 {
            last_ticked = Instant::now();
            vc.decrement_timers();
        }

        // 1. Input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = KeyPress::from_sdl_key(keycode) {
                        keys_pressed.insert(key);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = KeyPress::from_sdl_key(keycode) {
                        keys_pressed.take(&key);
                    }
                }
                _ => {}
            }
        }

        // 2. Update
        if let Some(instr_raw) = vc.fetch_instruction_and_increment_pc() {
            match parse_instruction(instr_raw) {
                Some(instr) => vc.execute_instruction(instr, &mut canvas, &keys_pressed),
                None => {} // None => eprintln!("Unknown raw instruction: {:#06x}", instr_raw),
            }
        }

        // 3. Render
        canvas.present();
        // std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
        std::thread::sleep(Duration::from_millis(7));
    }

    Ok(())
}
