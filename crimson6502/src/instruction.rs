#[derive(Debug, Clone, Copy)]
pub enum Mnemonic {
    NOP,
    LDA, LDX, LDY,
    STA, STX, STY,
    TAX, TAY, TSX, TXA, TXS, TYA
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY
}

impl AddressingMode {
    pub fn instruction_length(&self) -> u8 {
        match self {
            AddressingMode::Implied => 0,
            AddressingMode::Immediate => 1,
            AddressingMode::ZeroPage | AddressingMode::ZeroPageX | AddressingMode::ZeroPageY => 1,
            AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 2,
            AddressingMode::IndirectX | AddressingMode::IndirectY => 2,
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
            //LDA-----------------------------------------------------------------------------------
            0xA9 => Some(Self::new(Mnemonic::LDA, AddressingMode::Immediate)),
            0xA5 => Some(Self::new(Mnemonic::LDA, AddressingMode::ZeroPage)),
            0xB5 => Some(Self::new(Mnemonic::LDA, AddressingMode::ZeroPageX)),
            0xAD => Some(Self::new(Mnemonic::LDA, AddressingMode::Absolute)),
            0xBD => Some(Self::new(Mnemonic::LDA, AddressingMode::AbsoluteX)),
            0xB9 => Some(Self::new(Mnemonic::LDA, AddressingMode::AbsoluteY)),
            0xA1 => Some(Self::new(Mnemonic::LDA, AddressingMode::IndirectX)),
            0xB1 => Some(Self::new(Mnemonic::LDA, AddressingMode::IndirectY)),
            //LDX-----------------------------------------------------------------------------------
            0xA2 => Some(Self::new(Mnemonic::LDX, AddressingMode::Immediate)),
            0xA6 => Some(Self::new(Mnemonic::LDX, AddressingMode::ZeroPage)),
            0xB6 => Some(Self::new(Mnemonic::LDX, AddressingMode::ZeroPageY)),
            0xAE => Some(Self::new(Mnemonic::LDX, AddressingMode::Absolute)),
            0xBE => Some(Self::new(Mnemonic::LDX, AddressingMode::AbsoluteY)),
            //LDY-----------------------------------------------------------------------------------
            0xA0 => Some(Self::new(Mnemonic::LDY, AddressingMode::Immediate)),
            0xA4 => Some(Self::new(Mnemonic::LDY, AddressingMode::ZeroPage)),
            0xB4 => Some(Self::new(Mnemonic::LDY, AddressingMode::ZeroPageX)),
            0xAC => Some(Self::new(Mnemonic::LDY, AddressingMode::Absolute)),
            0xBC => Some(Self::new(Mnemonic::LDY, AddressingMode::AbsoluteX)),
            //STA-----------------------------------------------------------------------------------
            0x85 => Some(Self::new(Mnemonic::STA, AddressingMode::ZeroPage)),
            0x95 => Some(Self::new(Mnemonic::STA, AddressingMode::ZeroPageX)),
            0x8D => Some(Self::new(Mnemonic::STA, AddressingMode::Absolute)),
            0x9D => Some(Self::new(Mnemonic::STA, AddressingMode::AbsoluteX)),
            0x99 => Some(Self::new(Mnemonic::STA, AddressingMode::AbsoluteY)),
            0x81 => Some(Self::new(Mnemonic::STA, AddressingMode::IndirectX)),
            0x91 => Some(Self::new(Mnemonic::STA, AddressingMode::IndirectY)),
            //STX-----------------------------------------------------------------------------------
            0x86 => Some(Self::new(Mnemonic::STX, AddressingMode::ZeroPage)),
            0x96 => Some(Self::new(Mnemonic::STX, AddressingMode::ZeroPageX)),
            0x8E => Some(Self::new(Mnemonic::STX, AddressingMode::Absolute)),
            //STY-----------------------------------------------------------------------------------
            0x84 => Some(Self::new(Mnemonic::STY, AddressingMode::ZeroPage)),
            0x94 => Some(Self::new(Mnemonic::STY, AddressingMode::ZeroPageX)),
            0x8C => Some(Self::new(Mnemonic::STY, AddressingMode::Absolute)),
            //TAX-TAY-TSX-TXA-TXS-TYA---------------------------------------------------------------
            0xAA => Some(Self::new(Mnemonic::TAX, AddressingMode::Implied)),
            0xA8 => Some(Self::new(Mnemonic::TAY, AddressingMode::Implied)),
            0xBA => Some(Self::new(Mnemonic::TSX, AddressingMode::Implied)),
            0x8A => Some(Self::new(Mnemonic::TXA, AddressingMode::Implied)),
            0x9A => Some(Self::new(Mnemonic::TXS, AddressingMode::Implied)),
            0x98 => Some(Self::new(Mnemonic::TYA, AddressingMode::Implied)),
            //NOP-----------------------------------------------------------------------------------
            0xEA => Some(Self::new(Mnemonic::NOP, AddressingMode::Implied)),
            //--------------------------------------------------------------------------------------
            _ => None,
        }
    }
}