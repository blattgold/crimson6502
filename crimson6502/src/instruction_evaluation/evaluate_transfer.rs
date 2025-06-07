use crate::instruction_evaluation::types::InstructionResult;
use crate::instruction::{AddressingMode, IndexedBy, Mnemonic};
use crate::cpu::CPUState;
use crate::CPU;

fn cycles_transfer(addressing_mode: AddressingMode, page_crossed: bool) -> u8 {
    match addressing_mode {
        AddressingMode::Immediate => 2,
        AddressingMode::ZeroPage => 3,
        AddressingMode::ZeroPageX | AddressingMode::ZeroPageY => 4,
        AddressingMode::Absolute => 4,
        AddressingMode::AbsoluteY | AddressingMode::AbsoluteX if page_crossed => 5,
        AddressingMode::AbsoluteY | AddressingMode::AbsoluteX => 4,
        AddressingMode::IndirectX => 6,
        AddressingMode::IndirectY if page_crossed => 6,
        AddressingMode::IndirectY => 5,
        // invalid
        AddressingMode::Implied => panic!()
    }
}

fn set_flags_transfer(value: u8, state: &mut CPUState) {
    if value == 0 {
        state.sr |= flag!(zero);
    } else {
        state.sr &= !flag!(zero);
    }
    if value & 0x80 != 0 {
        state.sr |= flag!(negative);
    } else {
        state.sr &= !flag!(negative);
    }
}

pub fn evaluate_load(cpu: &mut CPU, mnemonic: Mnemonic, addressing_mode: AddressingMode) -> InstructionResult {
    let value: u8 = cpu.resolve_address_and_get_value(addressing_mode);

    let mut result = InstructionResult::new(
        CPUState {
            ..*cpu.get_state()
        },
        cycles_transfer(addressing_mode, cpu.crossed_page_boundary()),
        addressing_mode.instruction_length(),
    );

    set_flags_transfer(value, &mut result.state);

    match mnemonic {
        Mnemonic::LDA => result.state.a = value,
        Mnemonic::LDX => result.state.x = value,
        Mnemonic::LDY => result.state.y = value,
        _ => panic!("evaluate_transfer received invalid mnemonic: {:?}", mnemonic),
    };

    result
}

pub fn evaluate_store(cpu: &mut CPU, mnemonic: Mnemonic, addressing_mode: AddressingMode) -> InstructionResult {
    match mnemonic {
        Mnemonic::STA => cpu.resolve_address_and_set_value(addressing_mode, cpu.get_state().a),
        Mnemonic::STX => cpu.resolve_address_and_set_value(addressing_mode, cpu.get_state().x),
        Mnemonic::STY => cpu.resolve_address_and_set_value(addressing_mode, cpu.get_state().y),
        _ => panic!("evaluate_store received invalid mnemonic: {:?}", mnemonic),
    };

    InstructionResult::new(
        CPUState {
            ..*cpu.get_state()
        },
        cycles_transfer(addressing_mode, true),
        addressing_mode.instruction_length(),
    )
}

pub fn evaluate_transfer(cpu: &mut CPU, mnemonic: Mnemonic, addressing_mode: AddressingMode) -> InstructionResult {
    let mut new_state: CPUState = cpu.get_state().clone();

    match mnemonic {
        Mnemonic::TAX => {new_state.x = new_state.a; set_flags_transfer(new_state.a, &mut new_state)},
        Mnemonic::TAY => {new_state.y = new_state.a; set_flags_transfer(new_state.a, &mut new_state)},
        Mnemonic::TSX => {new_state.x = new_state.s; set_flags_transfer(new_state.s, &mut new_state)},
        Mnemonic::TXA => {new_state.a = new_state.x; set_flags_transfer(new_state.x, &mut new_state)},
        Mnemonic::TXS => {new_state.s = new_state.x},
        Mnemonic::TYA => {new_state.a = new_state.y; set_flags_transfer(new_state.y, &mut new_state)},
        _ => panic!("evaluate_transfer received invalid mnemonic: {:?}", mnemonic),
    };

    InstructionResult::new(
        new_state, 
        2, 
        addressing_mode.instruction_length()
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_evaluate_load() {
    }
}