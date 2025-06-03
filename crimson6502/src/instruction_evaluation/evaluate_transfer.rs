use crate::instruction_evaluation::types::InstructionResult;
use crate::instruction::{AddressingMode, IndexedBy, Mnemonic};
use crate::cpu::CPUState;
use crate::{CPUFlags, Memory};

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
        AddressingMode::Indirect(IndexedBy::Y) => 5,
        _ => panic!(),
    }
}

fn set_flags_transfer(value: u8, flags: &mut CPUFlags) {
    if value == 0 {
        flags.insert(CPUFlags::Z);
    } else {
        flags.remove(CPUFlags::Z)
    }
    if value & 0x80 != 0 {
        flags.insert(CPUFlags::N);
    } else {
        flags.remove(CPUFlags::N);
    }
}

pub fn evaluate_load(state: &CPUState, memory: &Memory, mnemonic: Mnemonic, addressing_mode: AddressingMode) -> InstructionResult {
    let (value, page_crossed)= addressing_mode.get_operand(state, memory);

    let mut result = InstructionResult::new(
        CPUState {
            pc: state.pc.wrapping_add((addressing_mode.instruction_length() + 1) as u16),
            ..*state
        },
        cycles_transfer(addressing_mode, page_crossed),
        addressing_mode.instruction_length(),
    );

    set_flags_transfer(value, &mut result.state.flags);

    match mnemonic {
        Mnemonic::LDA => result.state.a = value,
        Mnemonic::LDX => result.state.x = value,
        Mnemonic::LDY => result.state.y = value,
        _ => panic!("evaluate_transfer received invalid mnemonic: {:?}", mnemonic),
    };

    result
}

pub fn evaluate_store(state: &CPUState, memory: &mut Memory, mnemonic: Mnemonic, addressing_mode: AddressingMode) -> InstructionResult {
    match mnemonic {
        Mnemonic::STA => addressing_mode.write_value(state.a, state, memory),
        Mnemonic::STX => addressing_mode.write_value(state.x, state, memory),
        Mnemonic::STY => addressing_mode.write_value(state.y, state, memory),
        _ => panic!("evaluate_store received invalid mnemonic: {:?}", mnemonic),
    };

    InstructionResult::new(
        CPUState {
            pc: state.pc.wrapping_add((addressing_mode.instruction_length() + 1) as u16),
            ..*state
        },
        cycles_transfer(addressing_mode, true),
        addressing_mode.instruction_length(),
    )
}

pub fn evaluate_transfer(state: &CPUState, memory: &mut Memory, mnemonic: Mnemonic, addressing_mode: AddressingMode) -> InstructionResult {
    let mut new_state: CPUState = state.clone();

    match mnemonic {
        Mnemonic::TAX => {new_state.x = state.a; set_flags_transfer(state.a, &mut new_state.flags)},
        Mnemonic::TAY => {new_state.y = state.a; set_flags_transfer(state.a, &mut new_state.flags)},
        Mnemonic::TSX => {new_state.x = state.s; set_flags_transfer(state.s, &mut new_state.flags)},
        Mnemonic::TXA => {new_state.a = state.x; set_flags_transfer(state.x, &mut new_state.flags)},
        Mnemonic::TXS => {new_state.s = state.x},
        Mnemonic::TYA => {new_state.a = state.y; set_flags_transfer(state.y, &mut new_state.flags)},
        _ => panic!("evaluate_transfer received invalid mnemonic: {:?}", mnemonic),
    };

    new_state.pc = state.pc.wrapping_add((addressing_mode.instruction_length() + 1) as u16);

    InstructionResult::new(
        new_state, 
        2, 
        addressing_mode.instruction_length()
    )
}