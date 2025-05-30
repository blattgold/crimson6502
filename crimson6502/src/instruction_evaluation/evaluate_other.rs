use crate::instruction_evaluation::types::InstructionResult;
use crate::cpu::CPUState;

pub trait NOPEvaluation {
    fn evaluate_nop(state: &CPUState) -> InstructionResult {
        InstructionResult::new(
            state.clone(),
            2u8,
            1u8,
        ) 
    }
}