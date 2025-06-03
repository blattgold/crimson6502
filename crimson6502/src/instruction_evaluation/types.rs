use crate::cpu::CPUState;

#[derive(Debug, PartialEq)]
pub struct InstructionResult {
    pub state: CPUState,
    pub cycles: u8,
    pub instruction_length: u8,
}

impl InstructionResult {
    pub fn new(state: CPUState, cycles: u8, instruction_length: u8) -> InstructionResult {
        Self {
            state: state,
            cycles: cycles,
            instruction_length: instruction_length,
        }
    }
}