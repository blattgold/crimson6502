use bitflags::bitflags;
use crate::memory::Memory;
use crate::instruction::{Instruction, Mnemonic, AddressingMode, IndexedBy};
use crate::instruction_evaluation::{NOPEvaluation, InstructionResult};

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

#[derive(Clone, Debug)]
pub struct CPUState {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    pc: u16,
    flags: CPUFlags,
}

#[derive(Debug)]
pub struct CPUStats {
    total_cycles: usize,
    instructions: usize,
}


#[derive(
    Debug, 
)]
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

impl NOPEvaluation for CPU {}

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
        let instruction_byte: u8 = memory.read(self.state.pc);
        let instruction_option: Option<Instruction> = Instruction::from_byte(instruction_byte);

        if let Some(instruction) = instruction_option {
            let result: InstructionResult = self.execute_instruction(instruction);
            println!("{:?}, {:?}", result.state, result.cycles);
            self.state = result.state;
            self.stats.total_cycles += result.cycles as usize;
        } else {
            println!("Couldn't execute instruction.");
        }
    }

    fn execute_instruction(&self, instruction: Instruction) -> InstructionResult {
        match instruction {
            Instruction{mnemonic: Mnemonic::Nop, addressing_mode: AddressingMode::Implied} => CPU::evaluate_nop(&self.state),
            _ => CPU::evaluate_nop(&self.state),
        }
    }
}