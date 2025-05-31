use crate::instruction_evaluation::types::InstructionResult;
use crate::cpu::CPUState;

pub fn evaluate_nop(state: &CPUState) -> InstructionResult {
    InstructionResult::new(
        CPUState {
            pc: state.pc + 1,
            ..*state
        },
        2u8,
        1u8,
    ) 
}