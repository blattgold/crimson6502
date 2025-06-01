use crate::Memory;
use crate::CPUState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexedBy {
    None,
    X,
    Y,
}

impl IndexedBy {
    fn from_state(&self, state: &CPUState) -> u8 {
        match self {
            IndexedBy::None => 0u8,
            IndexedBy::X => state.x,
            IndexedBy::Y => state.y,
        }
    }
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

    pub fn get_operand(&self, state: &CPUState, memory: &Memory) -> (u8, bool) {
        let (addr, page_crossed) = self.effective_operand_address(state, memory);
        (memory.read_byte(addr), page_crossed)
    }

    fn effective_operand_address(&self, state: &CPUState, memory: &Memory) -> (u16, bool) {
        let pc_next: u16 = state.pc.wrapping_add(1);
        let mut page_crossed: bool = false;
        (match self {
            AddressingMode::Immediate => pc_next,
            AddressingMode::ZeroPage(indexed_by) => {
                let addr: u8 = memory.read_byte(pc_next).wrapping_add(indexed_by.from_state(state));
                addr as u16
            },
            AddressingMode::Absolute(indexed_by) => {
                let (addr_lo, addr_hi) = (memory.read_byte(pc_next), memory.read_byte(pc_next.wrapping_add(1)));
                let addr: u16 = ((addr_hi as u16) << 8) | (addr_lo as u16);
                let effective_addr: u16 = addr.wrapping_add(indexed_by.from_state(state) as u16);
                page_crossed = (addr & 0xFF00) != (effective_addr & 0xFF00);
                effective_addr
            },
            _ => panic!("unimplemented AddressingMode handling for: {:?}", self),
        }, page_crossed)
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
            //LDA-----------------------------------------------------------------------------------
            0xA9 => Some(Self::new(Mnemonic::LDA, AddressingMode::Immediate)),
            0xA5 => Some(Self::new(Mnemonic::LDA, AddressingMode::ZeroPage(IndexedBy::None))),
            0xB5 => Some(Self::new(Mnemonic::LDA, AddressingMode::ZeroPage(IndexedBy::X))),
            0xAD => Some(Self::new(Mnemonic::LDA, AddressingMode::Absolute(IndexedBy::None))),
            0xBD => Some(Self::new(Mnemonic::LDA, AddressingMode::Absolute(IndexedBy::X))),
            0xB9 => Some(Self::new(Mnemonic::LDA, AddressingMode::Absolute(IndexedBy::Y))),
            //NOP-----------------------------------------------------------------------------------
            0xEA => Some(Self::new(Mnemonic::NOP, AddressingMode::Implied)),
            _ => None,
        }
    }
}