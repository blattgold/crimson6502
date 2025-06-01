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
    LDX,
    LDY,
    STA,
    STX,
    STY
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
            AddressingMode::Implied => 0,
            AddressingMode::Immediate => 1,
            AddressingMode::ZeroPage(_) => 1,
            AddressingMode::Absolute(_) => 2,
            AddressingMode::Indirect(_) => 2,
        }
    }

    pub fn get_operand(&self, state: &CPUState, memory: &Memory) -> (u8, bool) {
        let (addr, page_crossed) = self.effective_operand_address(state, memory);
        (memory.read_byte(addr), page_crossed)
    }

    pub fn write_value(&self, value: u8, state: &CPUState, memory: &mut Memory) {
        let (addr, _) = self.effective_operand_address(state, memory);
        memory.write_byte(addr, value);
    }

    fn effective_operand_address(&self, state: &CPUState, memory: &Memory) -> (u16, bool) {
        let pc_next: u16 = state.pc.wrapping_add(1);
        let mut page_crossed: bool = false;
        (match self {
            AddressingMode::Immediate => pc_next,
            AddressingMode::ZeroPage(indexed_by) => {
                let effective_addr: u8 = memory.read_byte(pc_next).wrapping_add(indexed_by.from_state(state));
                effective_addr as u16
            },
            AddressingMode::Absolute(indexed_by) => {
                let (addr_lo, addr_hi) = (memory.read_byte(pc_next), memory.read_byte(pc_next.wrapping_add(1)));
                let addr: u16 = ((addr_hi as u16) << 8) | (addr_lo as u16);
                let effective_addr: u16 = addr.wrapping_add(indexed_by.from_state(state) as u16);
                page_crossed = (addr & 0xFF00) != (effective_addr & 0xFF00);
                effective_addr
            },
            AddressingMode::Indirect(indexed_by) => {
                if *indexed_by == IndexedBy::X {
                    let indirect_addr: u8 = memory.read_byte(pc_next).wrapping_add(indexed_by.from_state(state));
                    let (addr_lo, addr_hi) = (memory.read_byte(indirect_addr as u16), memory.read_byte(indirect_addr.wrapping_add(1) as u16));
                    let effective_addr: u16 = ((addr_hi as u16) << 8) | (addr_lo as u16);
                    effective_addr
                } else if *indexed_by == IndexedBy::Y {
                    let indirect_addr: u8 = memory.read_byte(pc_next);
                    let (addr_lo, addr_hi) = (memory.read_byte(indirect_addr as u16), memory.read_byte(indirect_addr.wrapping_add(1) as u16));
                    let addr: u16 = ((addr_hi as u16) << 8) | (addr_lo as u16);
                    let effective_addr: u16 = addr.wrapping_add(indexed_by.from_state(state) as u16);
                    page_crossed = (addr & 0xFF00) != (effective_addr & 0xFF00);
                    effective_addr
                } else {
                    panic!();
                }
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
            0xA1 => Some(Self::new(Mnemonic::LDA, AddressingMode::Indirect(IndexedBy::X))),
            0xB1 => Some(Self::new(Mnemonic::LDA, AddressingMode::Indirect(IndexedBy::Y))),
            //LDX-----------------------------------------------------------------------------------
            0xA2 => Some(Self::new(Mnemonic::LDX, AddressingMode::Immediate)),
            0xA6 => Some(Self::new(Mnemonic::LDX, AddressingMode::ZeroPage(IndexedBy::None))),
            0xB6 => Some(Self::new(Mnemonic::LDX, AddressingMode::ZeroPage(IndexedBy::Y))),
            0xAE => Some(Self::new(Mnemonic::LDX, AddressingMode::Absolute(IndexedBy::None))),
            0xBE => Some(Self::new(Mnemonic::LDX, AddressingMode::Absolute(IndexedBy::Y))),
            //LDY-----------------------------------------------------------------------------------
            0xA0 => Some(Self::new(Mnemonic::LDY, AddressingMode::Immediate)),
            0xA4 => Some(Self::new(Mnemonic::LDY, AddressingMode::ZeroPage(IndexedBy::None))),
            0xB4 => Some(Self::new(Mnemonic::LDY, AddressingMode::ZeroPage(IndexedBy::X))),
            0xAC => Some(Self::new(Mnemonic::LDY, AddressingMode::Absolute(IndexedBy::None))),
            0xBC => Some(Self::new(Mnemonic::LDY, AddressingMode::Absolute(IndexedBy::X))),
            //STA-----------------------------------------------------------------------------------
            0x85 => Some(Self::new(Mnemonic::STA, AddressingMode::ZeroPage(IndexedBy::None))),
            0x95 => Some(Self::new(Mnemonic::STA, AddressingMode::ZeroPage(IndexedBy::X))),
            0x8D => Some(Self::new(Mnemonic::STA, AddressingMode::Absolute(IndexedBy::None))),
            0x9D => Some(Self::new(Mnemonic::STA, AddressingMode::Absolute(IndexedBy::X))),
            0x99 => Some(Self::new(Mnemonic::STA, AddressingMode::Absolute(IndexedBy::Y))),
            0x81 => Some(Self::new(Mnemonic::STA, AddressingMode::Indirect(IndexedBy::X))),
            0x91 => Some(Self::new(Mnemonic::STA, AddressingMode::Indirect(IndexedBy::Y))),
            //STX-----------------------------------------------------------------------------------
            0x86 => Some(Self::new(Mnemonic::STX, AddressingMode::ZeroPage(IndexedBy::None))),
            0x96 => Some(Self::new(Mnemonic::STX, AddressingMode::ZeroPage(IndexedBy::X))),
            0x8E => Some(Self::new(Mnemonic::STX, AddressingMode::Absolute(IndexedBy::None))),
            //STY-----------------------------------------------------------------------------------
            0x84 => Some(Self::new(Mnemonic::STY, AddressingMode::ZeroPage(IndexedBy::None))),
            0x94 => Some(Self::new(Mnemonic::STY, AddressingMode::ZeroPage(IndexedBy::X))),
            0x8C => Some(Self::new(Mnemonic::STY, AddressingMode::Absolute(IndexedBy::None))),
            //NOP-----------------------------------------------------------------------------------
            0xEA => Some(Self::new(Mnemonic::NOP, AddressingMode::Implied)),
            //--------------------------------------------------------------------------------------
            _ => None,
        }
    }
}