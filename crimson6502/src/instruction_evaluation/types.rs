use crate::cpu::CPUState;

#[derive(Debug)]
pub struct InstructionResult {
    pub state: CPUState,
    pub cycles: u8,
    pub bytes_read: u8,
}

impl InstructionResult {
    pub fn new(state: CPUState, cycles: u8, bytes_read: u8) -> InstructionResult {
        Self {
            state: state,
            cycles: cycles,
            bytes_read: bytes_read,
        }
    }
}