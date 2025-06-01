use bitflags::bitflags;
use crate::memory::Memory;
use crate::instruction::{Instruction, Mnemonic};
use crate::instruction_evaluation::{evaluate_load, evaluate_nop, evaluate_store, InstructionResult};

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct CPUFlags: u8 {
        const C = 0b00000001;
        const Z = 0b00000010;
        const I = 0b00000100;
        const D = 0b00001000;
        const B = 0b00010000;
        const V = 0b01000000;
        const N = 0b10000000;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CPUState {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub pc: u16,
    pub flags: CPUFlags,
}

#[derive(Debug)]
pub struct CPUStats {
    total_cycles: usize,
    instructions: usize,
}


#[derive(Debug)]
pub struct CPU {
    state: CPUState,
    stats: CPUStats,
}


impl CPUState {
    pub fn new() -> CPUState {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            pc: 0,
            flags: CPUFlags::empty(),
        }
    }
}

impl CPUStats {
    pub fn new() -> CPUStats {
        Self {
            total_cycles: 0usize,
            instructions: 0usize,
        }
    }
}

impl CPU {
    pub fn new(state: CPUState) -> CPU {
        Self {
            state: state,
            stats: CPUStats::new(),
        }
    }

    pub fn init_state(&mut self) {
        self.state.pc = 0x1000u16;
    }

    pub fn run(&mut self, memory: &mut Memory) {
        let instruction_byte: u8 = memory.read_byte(self.state.pc);
        let instruction_option: Option<Instruction> = Instruction::from_byte(instruction_byte);

        if let Some(instruction) = instruction_option {
            let result: InstructionResult = self.execute_instruction(instruction, memory);
            println!("{:?}, cycles: {:?}, instruction length: {:?}", result.state, result.cycles, result.instruction_length);
            self.state = result.state;
            self.stats.total_cycles += result.cycles as usize;
            self.stats.instructions += 1;
        } else {
            panic!("Could not convert byte to instruction: {:?} at address {:?}.", instruction_byte, self.state.pc);
        }
    }

    fn execute_instruction(&self, instruction: Instruction, memory: &mut Memory) -> InstructionResult {
        match instruction.mnemonic {
            Mnemonic::NOP => evaluate_nop(&self.state),
            Mnemonic::LDA | Mnemonic::LDX | Mnemonic::LDY => evaluate_load(&self.state, memory, instruction.mnemonic, instruction.addressing_mode),
            Mnemonic::STA | Mnemonic::STX | Mnemonic::STY => evaluate_store(&self.state, memory, instruction.mnemonic, instruction.addressing_mode),
        }
    }
}