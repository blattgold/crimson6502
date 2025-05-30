const LENGTH_IMPLIED: u8 = 1u8;
const LENGTH_IMMEDIATE: u8 = 2u8;
const LENGTH_ZEROPAGE: u8 = 2u8;
const LENGTH_ABSOLUTE: u8 = 3u8;
const LENGTH_INDIRECT: u8 = 3u8;

#[derive(Clone, Copy)]
pub enum IndexedBy {
    None,
    X,
    Y,
}

pub enum Mnemonic {
    Nop,
}

#[derive(Clone, Copy)]
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
            AddressingMode::Implied => LENGTH_IMPLIED,
            AddressingMode::Immediate => LENGTH_IMMEDIATE,
            AddressingMode::ZeroPage(_) => LENGTH_ZEROPAGE,
            AddressingMode::Absolute(_) => LENGTH_ABSOLUTE,
            AddressingMode::Indirect(_) => LENGTH_INDIRECT,
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
}

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
            0x00 => Some(Self::new(Mnemonic::Nop, AddressingMode::Implied)),
            _ => None,
        }
    }
}