mod cpu;
mod instruction;
mod memory;
mod instruction_evaluation;

use crate::memory::Memory;
use crate::cpu::{CPU, CPUFlags, CPUState, CPUStats};

fn main() {
    let mut cpu = CPU::new(CPUState::new());
    let mut memory: Memory = Memory::new();
    cpu.init_state();
    cpu.run(&mut memory);
}
