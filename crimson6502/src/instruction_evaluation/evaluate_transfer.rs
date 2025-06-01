use crate::instruction_evaluation::types::InstructionResult;
use crate::instruction::{AddressingMode, IndexedBy};
use crate::cpu::CPUState;
use crate::Memory;

fn cycles_transfer(addressing_mode: AddressingMode, page_crossed: bool) -> u8 {
    match addressing_mode {
        AddressingMode::Immediate => 2,
        AddressingMode::ZeroPage(IndexedBy::None) => 3,
        AddressingMode::ZeroPage(_) => 4,
        AddressingMode::Absolute(IndexedBy::None) => 4,
        AddressingMode::Absolute(_) if page_crossed => 5,
        AddressingMode::Absolute(_) => 4,
        AddressingMode::Indirect(IndexedBy::X) => 6,
        AddressingMode::Indirect(IndexedBy::Y) if page_crossed => 6,
        AddressingMode::Indirect(IndexedBy::Y) => 5, // TODO page cross penalty
        _ => panic!(),
    }
}

pub fn evaluate_lda(state: &CPUState, addressing_mode: AddressingMode, memory: &Memory) -> InstructionResult {
    // TODO flags
    let (value, page_crossed)= addressing_mode.get_operand(state, memory);

    InstructionResult::new(
        CPUState {
            a: value,
            pc: state.pc.wrapping_add((addressing_mode.instruction_length()) as u16),
            ..*state
        },
        cycles_transfer(addressing_mode, page_crossed),
        addressing_mode.instruction_length(),
    )
}