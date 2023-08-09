use anyhow::{anyhow, Result};
use bitmatch::bitmatch;
use sdl2::{
    keyboard::Keycode,
    rect::{Point, Rect},
    render::WindowCanvas,
};
use std::{collections::HashSet, fs::File, io::Read};

use crate::{
    constants::{
        BACKGROUND_COLOR, DISPLAY_HEIGHT, DISPLAY_WIDTH, FONT_DATA, FONT_STARTING_MEMORY_ADDRESS,
        PIXEL_COLOR, PIXEL_HEIGHT, PIXEL_WIDTH,
    },
    instruction_parser::InstructionType,
};

#[derive(PartialEq)]
pub enum CompatibilityMode {
    /// The original CHIP-8 interpreter
    CosmicVIP,

    /// A newer version
    SuperChip,
}

#[derive(Hash, PartialEq, Eq)]
pub enum KeyPress {
    Key0 = 0,
    Key1 = 1,
    Key2 = 2,
    Key3 = 3,
    Key4 = 4,
    Key5 = 5,
    Key6 = 6,
    Key7 = 7,
    Key8 = 8,
    Key9 = 9,
    KeyA = 10,
    KeyB = 11,
    KeyC = 12,
    KeyD = 13,
    KeyE = 14,
    KeyF = 15,
}

impl KeyPress {
    pub fn from_sdl_key(key: Keycode) -> Option<Self> {
        match key {
            Keycode::X => Some(KeyPress::Key0),
            Keycode::Num1 => Some(KeyPress::Key1),
            Keycode::Num2 => Some(KeyPress::Key2),
            Keycode::Num3 => Some(KeyPress::Key3),
            Keycode::Q => Some(KeyPress::Key4),
            Keycode::W => Some(KeyPress::Key5),
            Keycode::E => Some(KeyPress::Key6),
            Keycode::A => Some(KeyPress::Key7),
            Keycode::S => Some(KeyPress::Key8),
            Keycode::D => Some(KeyPress::Key9),
            Keycode::Z => Some(KeyPress::KeyA),
            Keycode::C => Some(KeyPress::KeyB),
            Keycode::Num4 => Some(KeyPress::KeyC),
            Keycode::R => Some(KeyPress::KeyD),
            Keycode::F => Some(KeyPress::KeyE),
            Keycode::V => Some(KeyPress::KeyF),
            _ => None,
        }
    }
}

impl std::convert::From<u8> for KeyPress {
    fn from(value: u8) -> Self {
        match value {
            0 => KeyPress::Key0,
            1 => KeyPress::Key1,
            2 => KeyPress::Key2,
            3 => KeyPress::Key3,
            4 => KeyPress::Key4,
            5 => KeyPress::Key5,
            6 => KeyPress::Key6,
            7 => KeyPress::Key7,
            8 => KeyPress::Key8,
            9 => KeyPress::Key9,
            10 => KeyPress::KeyA,
            11 => KeyPress::KeyB,
            12 => KeyPress::KeyC,
            13 => KeyPress::KeyD,
            14 => KeyPress::KeyE,
            15 => KeyPress::KeyF,
            _ => panic!("cannot convert {} to a KeyPress", value),
        }
    }
}

pub struct VirtualComputer {
    memory: [u8; 4096],
    display: [[bool; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
    stack: Vec<u16>,
    program_counter: u16,
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    variable_registers: [u8; 16],
    compatibility_mode: CompatibilityMode,
}

impl VirtualComputer {
    pub fn from_rom_file(mut rom_file: File) -> Result<Self> {
        let mut memory_buf = vec![];

        let allowed_rom_size = 4096 - 0x200; // First 200 bytes reserved for the "interpreter"
        if rom_file.metadata()?.len() > allowed_rom_size {
            return Err(anyhow!(
                "Rom file is greater than {} bytes!",
                allowed_rom_size
            ));
        }

        rom_file.read_to_end(&mut memory_buf)?;

        let mut vc = VirtualComputer::default();

        // FIXME: this is messy but I wanted to just get something working
        for (i, data) in memory_buf.iter().enumerate() {
            if i < allowed_rom_size as usize {
                vc.memory[0x200 + i] = *data;
            }
        }

        Ok(vc)
    }
}

impl Default for VirtualComputer {
    fn default() -> Self {
        let mut memory = [0; 4096];

        // Fill the font characters in memory
        for (i, font_byte) in FONT_DATA.into_iter().enumerate() {
            memory[*FONT_STARTING_MEMORY_ADDRESS as usize + i] = *font_byte;
        }

        Self {
            memory,
            display: [[false; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
            stack: vec![],
            program_counter: 0x200,
            index_register: 0,
            delay_timer: 255, // TODO: check if this is right
            sound_timer: 255, // TODO: check if this is right
            variable_registers: [0; 16],
            compatibility_mode: CompatibilityMode::CosmicVIP,
        }
    }
}

impl VirtualComputer {
    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn fetch_instruction_and_increment_pc(&mut self) -> Option<u16> {
        if self.program_counter >= 4096 {
            return None;
        }

        let instr = ((self.memory[self.program_counter as usize] as u16) << 8)
            | self.memory[self.program_counter as usize + 1] as u16;
        self.program_counter = self.program_counter + 2;
        Some(instr)
    }

    #[bitmatch]
    pub fn execute_instruction(
        &mut self,
        instr: InstructionType,
        canvas: &mut WindowCanvas,
        keys_pressed: &HashSet<KeyPress>,
    ) {
        println!("Executing instruction: {:?}", instr);

        match instr {
            InstructionType::ClearScreen => {
                canvas.set_draw_color(*BACKGROUND_COLOR);
                canvas.clear();
            }
            InstructionType::JumpToMemoryLocation(nnn) => self.program_counter = nnn,
            InstructionType::CallSubroutine(nnn) => {
                self.stack.push(self.program_counter);
                self.program_counter = nnn;
            }
            InstructionType::ReturnFromSubroutine => match self.stack.pop() {
                Some(pc) => self.program_counter = pc,
                None => eprintln!("Attempted to return from a subroutine with an empty stack"),
            },
            InstructionType::SkipIfRegisterEqValue { vx, value } => {
                if self.variable_registers[vx as usize] == value {
                    self.program_counter += 2;
                }
            }
            InstructionType::SkipIfRegisterNeqValue { vx, value } => {
                if self.variable_registers[vx as usize] != value {
                    self.program_counter += 2;
                }
            }
            InstructionType::SkipIfRegistersEq { vx, vy } => {
                if self.variable_registers[vx as usize] == self.variable_registers[vy as usize] {
                    self.program_counter += 2;
                }
            }
            InstructionType::SkipIfRegistersNeq { vx, vy } => {
                if self.variable_registers[vx as usize] != self.variable_registers[vy as usize] {
                    self.program_counter += 2;
                }
            }
            InstructionType::UpdateRegister { vx, value } => {
                self.variable_registers[vx as usize] = value;
            }
            InstructionType::AddValueToRegister { vx, value } => {
                self.variable_registers[vx as usize] =
                    self.variable_registers[vx as usize].wrapping_add(value);
            }
            InstructionType::CopyRegister { vx, vy } => {
                self.variable_registers[vx as usize] = self.variable_registers[vy as usize];
            }
            InstructionType::BitwiseOR { vx, vy } => {
                self.variable_registers[vx as usize] |= self.variable_registers[vy as usize];
            }
            InstructionType::BitwiseAND { vx, vy } => {
                self.variable_registers[vx as usize] &= self.variable_registers[vy as usize];
            }
            InstructionType::BitwiseXOR { vx, vy } => {
                self.variable_registers[vx as usize] ^= self.variable_registers[vy as usize];
            }
            InstructionType::AddRegisterToRegister { vx, vy } => {
                match self.variable_registers[vx as usize]
                    .overflowing_add(self.variable_registers[vy as usize])
                {
                    (sum, false) => {
                        self.variable_registers[vx as usize] = sum;
                        self.variable_registers[0xF] = 0;
                    }
                    (sum, true) => {
                        self.variable_registers[vx as usize] = sum;
                        self.variable_registers[0xF] = 1;
                    }
                }
            }
            InstructionType::SubtractXY { vx, vy } => {
                let minuend = self.variable_registers[vx as usize];
                let subtrahend = self.variable_registers[vy as usize];

                if minuend > subtrahend {
                    self.variable_registers[0xF] = 1;
                } else {
                    self.variable_registers[0xF] = 0;
                }

                self.variable_registers[vx as usize] = minuend.wrapping_sub(subtrahend);
            }
            InstructionType::SubtractYX { vx, vy } => {
                let minuend = self.variable_registers[vy as usize];
                let subtrahend = self.variable_registers[vx as usize];

                if minuend > subtrahend {
                    self.variable_registers[0xF] = 1;
                } else {
                    self.variable_registers[0xF] = 0;
                }

                self.variable_registers[vx as usize] = minuend.wrapping_sub(subtrahend);
            }
            InstructionType::ShiftLeft { vx, vy } => {
                let x = self.variable_registers[vy as usize];

                self.variable_registers[vx as usize] = (((x as u16) << 1) & 0xFF) as u8;

                if self.compatibility_mode == CompatibilityMode::CosmicVIP {
                    self.variable_registers[0xF] = (x & 0x40) >> 7;
                }
            }
            InstructionType::ShiftRight { vx, vy } => {
                let x = self.variable_registers[vy as usize];

                self.variable_registers[vx as usize] = x >> 1;

                if self.compatibility_mode == CompatibilityMode::CosmicVIP {
                    self.variable_registers[0xF] = x & 1;
                }
            }
            InstructionType::SetIndexRegister(nnn) => self.index_register = nnn,
            InstructionType::JumpWithOffset(nnn) => match self.compatibility_mode {
                CompatibilityMode::CosmicVIP => {
                    self.program_counter = nnn + self.variable_registers[0] as u16;
                }
                CompatibilityMode::SuperChip => {
                    #[bitmatch]
                    let "????xxxx????????" = nnn;

                    self.program_counter = nnn + self.variable_registers[x as usize] as u16;
                }
            },
            InstructionType::GenerateRandomNumber { vx, bitmask } => {
                self.variable_registers[vx as usize] = rand::random::<u8>() & bitmask;
            }
            InstructionType::Display { vx, vy, n } => {
                let x = self.variable_registers[vx as usize] % DISPLAY_WIDTH;
                let y = self.variable_registers[vy as usize] % DISPLAY_HEIGHT;

                self.variable_registers[0xF] = 0;
                let mut was_toggled_off = false;

                for i in 0..n {
                    let sprite_data = self.memory[(self.index_register + i as u16) as usize];

                    let py = y + i;

                    for j in 0..8 {
                        let px = x + j;

                        if px < DISPLAY_WIDTH && py < DISPLAY_HEIGHT {
                            let pixel_bit = (sprite_data >> (7 - j)) & 1;

                            if pixel_bit == 1 {
                                // Flip the display pixel

                                if !was_toggled_off && self.display[py as usize][px as usize] {
                                    was_toggled_off = true;
                                    self.variable_registers[0xF] = 1;
                                }

                                if self.display[py as usize][px as usize] {
                                    canvas.set_draw_color(*BACKGROUND_COLOR);
                                } else {
                                    canvas.set_draw_color(*PIXEL_COLOR);
                                }
                                canvas
                                    .fill_rect(Rect::new(
                                        (px as u32 * PIXEL_WIDTH as u32) as i32,
                                        (py as u32 * PIXEL_HEIGHT as u32) as i32,
                                        PIXEL_WIDTH as u32,
                                        PIXEL_HEIGHT as u32,
                                    ))
                                    .unwrap();
                                // canvas.draw_point(Point::new(px as i32, py as i32)).unwrap();

                                self.display[py as usize][px as usize] =
                                    !self.display[py as usize][px as usize];
                            }
                        }
                    }
                }
            }
            InstructionType::SkipIfPressedVX(vx) => {
                let x = self.variable_registers[vx as usize];
                if x > 15 {
                    eprintln!("Key {} is out of the range [0, 15]", x);
                    return;
                }

                if keys_pressed.contains(&KeyPress::from(x)) {
                    self.program_counter += 2;
                }
            }
            InstructionType::SkipIfNotPressedVX(vx) => {
                let x = self.variable_registers[vx as usize];
                if x > 15 {
                    eprintln!("Key {} is out of the range [0, 15]", x);
                    return;
                }

                if !keys_pressed.contains(&KeyPress::from(x)) {
                    self.program_counter += 2;
                }
            }
            InstructionType::FetchDelayTimerToVX(vx) => {
                self.variable_registers[vx as usize] = self.delay_timer;
            }
            InstructionType::SetDelayTimerToVX(vx) => {
                self.delay_timer = self.variable_registers[vx as usize];
            }
            InstructionType::SetSoundTimerToVX(vx) => {
                self.sound_timer = self.variable_registers[vx as usize];
            }
            InstructionType::AddToIndexFromVX(vx) => {
                let low_before = self.index_register <= 0xFFF;
                self.index_register = self
                    .index_register
                    .wrapping_add(self.variable_registers[vx as usize] as u16);

                // Weird behavior necessary for at least one game
                if low_before && self.index_register >= 0x1000 {
                    self.variable_registers[0xF] = 1;
                }
            }
            InstructionType::WaitForKeyInVX(vx) => {
                let x = self.variable_registers[vx as usize];
                if x > 15 {
                    eprintln!("Key {} is out of the range [0, 15]", x);
                    return;
                }

                if !keys_pressed.contains(&KeyPress::from(x)) {
                    self.program_counter -= 2;
                }
            }
            InstructionType::SetIndexToFontCharInVX(vx) => {
                let x = 0xF & self.variable_registers[vx as usize];

                // Characters are 5 bytes
                self.index_register = (*FONT_STARTING_MEMORY_ADDRESS + (x * 5)) as u16;
            }
            InstructionType::BinaryCodedDecimalConversionForVX(vx) => {
                let x = self.variable_registers[vx as usize];
                self.memory[self.index_register as usize] = (x as f64 / 100.0).floor() as u8;
                self.memory[(self.index_register + 1) as usize] =
                    ((x % 100) as f64 / 10.0).floor() as u8;
                self.memory[(self.index_register + 2) as usize] = x % 10;
            }
            InstructionType::StoreVariableRegistersToMemoryUpToVX(vx) => {
                for i in 0..=vx {
                    self.memory[(self.index_register + i as u16) as usize] =
                        self.variable_registers[i as usize];
                }

                if self.compatibility_mode == CompatibilityMode::CosmicVIP {
                    self.index_register += vx as u16;
                }
            }
            InstructionType::LoadMemoryToVariableRegistersFromVXAddress(vx) => {
                for i in 0..=vx {
                    self.variable_registers[i as usize] =
                        self.memory[(self.index_register + i as u16) as usize];
                }

                if self.compatibility_mode == CompatibilityMode::CosmicVIP {
                    self.index_register += vx as u16;
                }
            }
        }
    }
}
