use crate::Memory;
use crate::CPUState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexedBy {
    None,
    X,
    Y,
}

#[derive(Debug, Clone, Copy)]
pub enum Mnemonic {
    NOP,
    LDA,
}

#[derive(Clone, Copy, Debug)]
pub enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage(IndexedBy),
    Absolute(IndexedBy),
    Indirect(IndexedBy),
}

pub enum MemoryReadResult {
    Err,
    u8(u8),
    u16(u16),
}

pub enum MemoryWriteValue {
    u8(u8),
    u16(u16),
}

impl AddressingMode {
    pub fn instruction_length(&self) -> u8 {
        match self {
            AddressingMode::Implied => 1u8,
            AddressingMode::Immediate => 2u8,
            AddressingMode::ZeroPage(_) => 2u8,
            AddressingMode::Absolute(_) => 3u8,
            AddressingMode::Indirect(_) => 3u8,
        }
    }

    pub fn can_cross_page(&self) -> bool {
        matches!(
            self,
            AddressingMode::Absolute(IndexedBy::X) |
            AddressingMode::Absolute(IndexedBy::Y) |
            AddressingMode::Indirect(IndexedBy::Y)
        )
    }

    pub fn read_from_memory(&self, memory: &Memory, addr: u16) -> MemoryReadResult {
        match self {
            AddressingMode::Immediate => MemoryReadResult::u8(memory.read_byte(addr)),
            _ => MemoryReadResult::Err,
        }
    }

    pub fn write_to_memory(&self, memory: &mut Memory, addr: u16, value: MemoryWriteValue) {
        match (self, value)  {
            (AddressingMode::Immediate, MemoryWriteValue::u8(value)) 
                => memory.write_byte(addr, value),
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic, 
    pub addressing_mode: AddressingMode,
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic, addressing_mode: AddressingMode) -> Self {
        Self {
            mnemonic,
            addressing_mode,
        }
    }
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0xA9 => Some(Self::new(Mnemonic::LDA, AddressingMode::Immediate)),
            0xEA => Some(Self::new(Mnemonic::NOP, AddressingMode::Implied)),
            _ => None,
        }
    }
}