use crate::{instruction::AddressingMode, CPUState, CPU};

/// input: AddressingMode::{ ZeroPageX, AbsoluteX, IndirectX, ZeroPageY, AbsoluteY, IndirectY, ZeroPage, Absolute }
pub fn maybe_apply_index_value(addressing_mode: AddressingMode, state: &mut CPUState, index_value: u8) {
    use AddressingMode::*;

    match addressing_mode {
        ZeroPageX |
        AbsoluteX |
        IndirectX
            => state.x = index_value,
        ZeroPageY |
        AbsoluteY |
        IndirectY
            => state.y = index_value,
        ZeroPage |
        Absolute
            => (),
        _
            => panic!("invalid addressing mode for {}: {:?}", stringify!(maybe_apply_index_value), addressing_mode),
    };
}

pub fn compute_effective_address(addressing_mode: AddressingMode, index_value: u8, addr: u16, cpu: &CPU) -> u16 {
    use AddressingMode::*;

    match addressing_mode {
        ZeroPageX |
        ZeroPageY
            => ((addr as u8).wrapping_add(index_value)) as u16,
        AbsoluteX |
        AbsoluteY
            => addr.wrapping_add(index_value as u16),
        IndirectX
            => cpu.read_word_zp((addr as u8).wrapping_add(index_value)),
        IndirectY
            => cpu.read_word_zp(addr as u8).wrapping_add(index_value as u16),
        ZeroPage |
        Absolute
            => addr,
        _
            => panic!("invalid addressing mode for {}: {:?}", stringify!(compute_effective_address), addressing_mode),
    }
}