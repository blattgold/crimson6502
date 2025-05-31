use crate::instruction_evaluation::types::InstructionResult;
use crate::cpu::CPUState;

pub trait NOPEvaluation {
    fn evaluate_nop(state: &CPUState) -> InstructionResult {
        InstructionResult::new(
            CPUState {
                pc: state.pc + 2,
                ..*state
            },
            2u8,
            1u8,
        ) 
    }
}