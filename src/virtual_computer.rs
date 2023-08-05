use crate::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct VirtualComputer {
    memory: [u8; 4096],
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    stack: Vec<u16>,
    program_counter: u8,
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    variable_registers: [u8; 16],
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

    // aaaaaaaa00000000

    pub fn parse_instruction(&mut self, instr: u16) {}
}
