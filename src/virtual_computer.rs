use anyhow::{anyhow, Result};
use bitmatch::bitmatch;
use sdl2::{
    rect::{Point, Rect},
    render::WindowCanvas,
};
use std::{fs::File, io::Read};

use crate::{
    constants::{
        BACKGROUND_COLOR, DISPLAY_HEIGHT, DISPLAY_WIDTH, PIXEL_COLOR, PIXEL_HEIGHT, PIXEL_WIDTH,
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

        // FIXME: this is messy but I wanted to just get something working
        let mut memory = [0; 4096];
        for (i, data) in memory_buf.iter().enumerate() {
            if i < allowed_rom_size as usize {
                memory[0x200 + i] = *data;
            }
        }

        Ok(Self {
            memory,
            display: [[false; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
            stack: vec![],
            program_counter: 0x200,
            index_register: 0,
            delay_timer: 255, // TODO: check if this is right
            sound_timer: 255, // TODO: check if this is right
            variable_registers: [0; 16],
            compatibility_mode: CompatibilityMode::CosmicVIP,
        })
    }
}

impl Default for VirtualComputer {
    fn default() -> Self {
        Self {
            memory: [0; 4096],
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
    pub fn execute_instruction(&mut self, instr: InstructionType, canvas: &mut WindowCanvas) {
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
                                    .draw_rect(Rect::new(
                                        (px * PIXEL_WIDTH) as i32,
                                        (py * PIXEL_HEIGHT) as i32,
                                        PIXEL_WIDTH as u32,
                                        PIXEL_HEIGHT as u32,
                                    ))
                                    .unwrap();

                                self.display[py as usize][px as usize] =
                                    !self.display[py as usize][px as usize];
                            }
                        }
                    }
                }
            }
            InstructionType::SkipIfPressedVX(_) => todo!(),
            InstructionType::SkipIfNotPressedVX(_) => todo!(),
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
            InstructionType::WaitForKeyInVX(_) => todo!(),
            InstructionType::SetIndexToFontCharInVX(_) => todo!(),
            InstructionType::BinaryCodedDecimalConversionForVX(vx) => {
                let x = self.variable_registers[vx as usize];
                self.memory[self.index_register as usize] = x / 100;
                self.memory[(self.index_register + 1) as usize] = x / 10;
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
