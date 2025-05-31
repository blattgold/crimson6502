use crate::instruction_evaluation::types::InstructionResult;
use crate::instruction::{AddressingMode, IndexedBy, MemoryReadResult};
use crate::cpu::CPUState;
use crate::Memory;

fn cycles_transfer(addressing_mode: AddressingMode) -> u8 {
    match addressing_mode {
        AddressingMode::Immediate => 2,
        AddressingMode::ZeroPage(IndexedBy::None) => 3,
        AddressingMode::ZeroPage(_) => 4,
        AddressingMode::Absolute(IndexedBy::None) => 4,
        AddressingMode::Absolute(_) => 4, // TODO page cross penalty
        AddressingMode::Indirect(IndexedBy::X) => 6,
        AddressingMode::Indirect(IndexedBy::Y) => 5, // TODO page cross penalty
        _ => panic!(),
    }
}

pub fn evaluate_lda(state: &CPUState, addressing_mode: AddressingMode, memory: &Memory) -> InstructionResult {
    let read_result: MemoryReadResult = addressing_mode.read_from_memory(memory, state.pc.wrapping_add(1u16));
    if let MemoryReadResult::u8(read_result) = read_result {
        return InstructionResult::new(
            CPUState {
                a: read_result,
                pc: state.pc.wrapping_add((addressing_mode.instruction_length()) as u16),
                ..*state
            },
            cycles_transfer(addressing_mode),
            addressing_mode.instruction_length(),
            
        );
    } else {
        panic!("");
    }
}