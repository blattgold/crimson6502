use crate::memory::Memory;
use crate::instruction::{AddressingMode, Instruction, Mnemonic};
use crate::instruction_evaluation::{evaluate_load, evaluate_nop, evaluate_store, evaluate_transfer, InstructionResult};
use crate::util::bytes_to_word;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPUState {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub pc: u16,
    pub sr: u8,
}

#[derive(Debug, PartialEq)]
pub struct CPUStats {
    total_cycles: usize,
    instructions: usize,
}


#[derive(Debug, PartialEq)]
pub struct CPU {
    state: CPUState,
    stats: CPUStats,
    memory: Memory,
    /// stores whether last address resolution crossed a page boundary
    page_crossed: bool,
}

impl CPUState {
    pub fn new() -> CPUState {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            pc: 0,
            sr: 0,
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
    pub fn new(state: CPUState, memory: Memory) -> CPU {
        Self {
            state: state,
            stats: CPUStats::new(),
            memory: memory,
            page_crossed: false,
        }
    }

    /// emulates 6502 reset procedure
    pub fn init_state(&mut self) {
        self.state.pc = 0x1000u16;
        // TODO set stack pointer
    }

    /// get immutable reference to CPUState
    pub fn get_state(&self) -> &CPUState {
        &self.state
    }

    /// updates whether a page cross occured during last address resolution.
    /// Takes pre- and post-index memory address
    fn update_page_cross(&mut self, addr_unindexed: u16, addr_indexed: u16) {
        let page_crossed: bool = (addr_unindexed & 0xFF00) != (addr_indexed & 0xFF00);
        self.page_crossed = page_crossed;
    }

    /// whether last address resolution crossed page boundary
    pub fn crossed_page_boundary(&self) -> bool {
        self.page_crossed
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.memory.write_byte(addr, value);
    }

    /// reads one byte from memory at given address
    pub fn read_byte(&self, addr: u16) -> u8 {
        let read_byte: u8 = self.memory.read_byte(addr);
        read_byte
    }

    /// reads one word from memory (little endian) starting at addr.
    pub fn read_word(&self, addr: u16) -> u16 {
        let lo: u8 = self.memory.read_byte(addr);
        let hi: u8 = self.memory.read_byte(addr.wrapping_add(1));
        let read_word: u16 = bytes_to_word(lo, hi);
        read_word
    }

    /// fetches pc and then increments it
    pub fn fetch_operand_address(&mut self) -> u16 {
        let fetched_operand_address: u16 = self.state.pc;
        self.state.pc = self.state.pc.wrapping_add(1);
        fetched_operand_address
    }

    /// fetches byte pointed to by pc and then increments pc
    pub fn fetch_byte(&mut self) -> u8 {
        let operand_address: u16 = self.fetch_operand_address();
        let fetched_byte: u8 = self.memory.read_byte(operand_address);
        fetched_byte
    }

    /// fetches byte pointed to by pc, and pc+1, increments pc twice.
    /// combines both bytes into a word (little endian).
    pub fn fetch_word(&mut self) -> u16 {
        let lo: u8 = self.fetch_byte();
        let hi: u8 = self.fetch_byte();
        let fetched_word: u16 = bytes_to_word(lo, hi);
        fetched_word
    }

    pub fn run(&mut self) {
        let instruction_byte: u8 = self.fetch_byte();
        let instruction_option: Option<Instruction> = Instruction::from_byte(instruction_byte);

        if let Some(instruction) = instruction_option {
            let result: InstructionResult = self.execute_instruction(instruction);
            println!("{:?}, cycles: {:?}, instruction length: {:?}", result.state, result.cycles, result.instruction_length);
            self.state = result.state;
            self.stats.total_cycles += result.cycles as usize;
            self.stats.instructions += 1;
        } else {
            panic!("Could not convert byte to instruction: {:?} at address {:?}.", instruction_byte, self.state.pc);
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> InstructionResult {
        match instruction.mnemonic {
            Mnemonic::NOP 
                => evaluate_nop(self),
            Mnemonic::LDA | Mnemonic::LDX | Mnemonic::LDY 
                => evaluate_load(self, instruction.mnemonic, instruction.addressing_mode),
            Mnemonic::STA | Mnemonic::STX | Mnemonic::STY 
                => evaluate_store(self, instruction.mnemonic, instruction.addressing_mode),
            Mnemonic::TAX | Mnemonic::TAY | Mnemonic::TSX | Mnemonic::TXA | Mnemonic::TXS | Mnemonic::TYA 
                => evaluate_transfer(self, instruction.mnemonic, instruction.addressing_mode) 
        }
    }

    pub fn resolve_address_and_set_value(&mut self, addressing_mode: AddressingMode, value: u8) {
        let addr: u16 = self.resolve_address(addressing_mode);
        self.memory.write_byte(addr, value);
    }

    pub fn resolve_address_and_get_value(&mut self, addressing_mode: AddressingMode) -> u8 {
        let addr: u16 = self.resolve_address(addressing_mode);
        let value: u8 = self.read_byte(addr);
        value
    }

    pub fn resolve_address(&mut self, addressing_mode: AddressingMode) -> u16 {
            match addressing_mode {
                AddressingMode::Immediate => self.fetch_operand_address(),
                AddressingMode::ZeroPage | AddressingMode::ZeroPageX | AddressingMode::ZeroPageY => {
                    let addr: u8 = self.fetch_byte();
                    let effective_addr: u8 = match addressing_mode {
                        AddressingMode::ZeroPageX => addr.wrapping_add(self.get_state().x),
                        AddressingMode::ZeroPageY => addr.wrapping_add(self.get_state().y),
                        // cannot match with ZeroPage directly because compiler is stupid and can't infer ZeroPage being the only other possibility.
                        _ => addr,
                    };
                    effective_addr as u16
                },
                AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => {
                    let addr: u16 = self.fetch_word();
                    let effective_addr: u16 = match addressing_mode {
                        AddressingMode::AbsoluteX => addr.wrapping_add(self.get_state().x as u16),
                        AddressingMode::AbsoluteY => addr.wrapping_add(self.get_state().y as u16),
                        // cannot match with Absolute directly because compiler is stupid and can't infer Absolute being the only other possibility.
                        _ => addr,
                    };
                    self.update_page_cross(addr, effective_addr);
                    effective_addr
                },
                AddressingMode::IndirectX => {
                        let indirect_addr: u8 = self.fetch_byte();
                        let indirect_addr_indexed: u8 = indirect_addr.wrapping_add(self.get_state().x);
                        let effective_addr: u16 = self.read_word(indirect_addr_indexed as u16);
                        effective_addr
                },
                AddressingMode::IndirectY => {
                        let indirect_addr: u8 = self.fetch_byte();
                        let effective_addr_unindexed: u16 = self.read_word(indirect_addr as u16);
                        let effective_addr: u16 = effective_addr_unindexed.wrapping_add(self.get_state().y as u16);
                        self.update_page_cross(effective_addr_unindexed, effective_addr);
                        effective_addr
                },
                _ => panic!("unimplemented AddressingMode handling for: {:?}", self),
            }
    }
}