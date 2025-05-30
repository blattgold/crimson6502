use crimson6502::{CPU, Memory, CPUState};
use std::io;

fn main() {
    let mut cpu = CPU::new(CPUState::new());
    let mut memory: Memory = Memory::new();

    cpu.init_state();

    let mut input = String::new();
    loop {
        cpu.run(&mut memory);
        let _ = io::stdin().read_line(&mut input);
    }
}