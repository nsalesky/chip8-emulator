use sdl2::render::WindowCanvas;

use crate::{
    constants::{BACKGROUND_COLOR, DISPLAY_HEIGHT, DISPLAY_WIDTH},
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
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    stack: Vec<u16>,
    program_counter: u16,
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    variable_registers: [u8; 16],
    compatibility_mode: CompatibilityMode,
}

impl Default for VirtualComputer {
    fn default() -> Self {
        Self {
            memory: [0; 4096],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            stack: vec![],
            program_counter: 0,
            index_register: 0,
            delay_timer: 255, // TODO: check if this is right
            sound_timer: 255, // TODO: check if this is right
            variable_registers: [0; 16],
            compatibility_mode: CompatibilityMode::CosmicVIP,
        }
    }
}

impl VirtualComputer {
    pub fn fetch_instruction_and_increment_pc(&mut self) -> u16 {
        let instr = ((self.memory[self.program_counter as usize] as u16) << 8)
            | self.memory[self.program_counter as usize + 1] as u16;

        self.program_counter = self.program_counter.wrapping_add(2);
        instr
    }

    pub fn execute_instruction(&mut self, instr: InstructionType, canvas: &mut WindowCanvas) {
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
            InstructionType::SetIndexRegister(_) => todo!(),
            InstructionType::JumpWithOffset(_) => todo!(),
            InstructionType::GenerateRandomNumber { vx, bitmask } => todo!(),
            InstructionType::Display { vx, vy, n } => todo!(),
            InstructionType::SkipIfPressedVX(_) => todo!(),
            InstructionType::SkipIfNotPressedVX(_) => todo!(),
            InstructionType::FetchDelayTimerToVX(_) => todo!(),
            InstructionType::SetDelayTimerToVX(_) => todo!(),
            InstructionType::SetSoundTimerToVX(_) => todo!(),
            InstructionType::AddToIndexFromVX(_) => todo!(),
            InstructionType::WaitForKeyInVX(_) => todo!(),
            InstructionType::SetIndexToFontCharInVX(_) => todo!(),
            InstructionType::BinaryCodedDecimalConversionForVX(_) => todo!(),
            InstructionType::StoreVariableRegistersToMemoryUpToVX(_) => todo!(),
            InstructionType::LoadMemoryToVariableRegistersFromVXAddress(_) => todo!(),
        }
    }
}
