use crate::instruction_evaluation::types::InstructionResult;
use crate::CPU;

pub fn evaluate_nop(cpu: &CPU) -> InstructionResult {
    InstructionResult::new(
        cpu.get_state().clone(),
        2,
        0,
    ) 
}