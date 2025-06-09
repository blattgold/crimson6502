use rand::Rng;
use crate::CPUState;
use crate::Memory;
use crate::CPU;
use crate::instruction::*;
use crate::instruction_evaluation::*;
use super::helpers::*;

fn transfer_flags_helper(value: u8) -> u8 {
    let mut flags = 0;
    if value > 127 {
        flags |= flag!(negative);
    }
    if value == 0 {
        flags |= flag!(zero);
    }
    flags
}

pub fn test_load_immediate_helper(mnemonic: Mnemonic, expected_value: u8) {
    let mut cpu: CPU = CPU::new(CPUState::new(), Memory::new());
    cpu.write_byte(0, expected_value);

    let mut expected_state = CPUState::new();
    expected_state.pc = 1;
    
    match mnemonic {
        Mnemonic::LDA => expected_state.a = expected_value,
        Mnemonic::LDX => expected_state.x = expected_value,
        Mnemonic::LDY => expected_state.y = expected_value,
        _ => panic!("what the fuck is this mnemonic? {:?}", mnemonic),
    };

    expected_state.sr = transfer_flags_helper(expected_value);

    assert_eq!(
        evaluate_load(&mut cpu, mnemonic, AddressingMode::Immediate),
        InstructionResult {
            state: expected_state,
            cycles: 2,
            instruction_length: 1,
        }
    )
}

pub fn test_load_zeropage_helper(mnemonic: Mnemonic, expected_value: u8, addr: u8, addressing_mode: AddressingMode, index_value: u8) {
    let mut cpu_state = CPUState::new();
    maybe_apply_index_value(addressing_mode, &mut cpu_state, index_value);

    let mut cpu: CPU = CPU::new(cpu_state, Memory::new());
    let effective_addr: u16 = compute_effective_address(addressing_mode, index_value, addr as u16, &cpu);
    cpu.write_byte(0, addr);
    cpu.write_byte(effective_addr as u16, expected_value);

    let mut expected_state = cpu_state.clone();
    match mnemonic {
        Mnemonic::LDA => expected_state.a = expected_value,
        Mnemonic::LDX => expected_state.x = expected_value,
        Mnemonic::LDY => expected_state.y = expected_value,
        _ => panic!("what the fuck is this mnemonic? {:?}", mnemonic),
    };
    expected_state.sr = transfer_flags_helper(expected_value);
    expected_state.pc = 1;

    assert_eq!(
        evaluate_load(&mut cpu, mnemonic, addressing_mode),
        InstructionResult {
            state: expected_state,
            cycles: if addressing_mode == AddressingMode::ZeroPage {3} else {4},
            instruction_length: 1,
        }
    )
}

pub fn test_load_absolute_helper(mnemonic: Mnemonic, expected_value: u8, addr: u16, addressing_mode: AddressingMode, index_value: u8) {
    let mut cpu_state = CPUState::new();
    maybe_apply_index_value(addressing_mode, &mut cpu_state, index_value);

    let mut cpu: CPU = CPU::new(cpu_state, Memory::new());
    let mut expected_state = cpu_state.clone();
    let effective_addr: u16 = compute_effective_address(addressing_mode, index_value, addr, &cpu);
    cpu.write_byte(0, addr as u8);
    cpu.write_byte(1, (addr >> 8) as u8);
    cpu.write_byte(effective_addr as u16, expected_value);

    match mnemonic {
        Mnemonic::LDA => expected_state.a = expected_value,
        Mnemonic::LDX => expected_state.x = expected_value,
        Mnemonic::LDY => expected_state.y = expected_value,
        _ => panic!("what the fuck is this mnemonic? {:?}", mnemonic),
    };
    expected_state.sr = transfer_flags_helper(expected_value);
    expected_state.pc = 2;

    assert_eq!(
        evaluate_load(&mut cpu, mnemonic, addressing_mode),
        InstructionResult {
            state: expected_state,
            cycles: if addressing_mode == AddressingMode::Absolute {4} else {
                if (effective_addr &0xFF00) == (addr &0xFF00) {4} else {5}
            },
            instruction_length: addressing_mode.instruction_length(),
        }
    )
}

pub fn test_load_indirect_helper(mnemonic: Mnemonic, expected_value: u8, addr: u16, addressing_mode: AddressingMode, index_value: u8) {
    let mut cpu_state = CPUState::new();
    maybe_apply_index_value(addressing_mode, &mut cpu_state, index_value);

    let mut cpu: CPU = CPU::new(cpu_state, Memory::new());
    let mut expected_state = cpu_state.clone();
    let effective_addr: u16 = compute_effective_address(addressing_mode, index_value, addr, &cpu);
    cpu.write_byte(0, addr as u8);
    cpu.write_byte(1, (addr >> 8) as u8);
    cpu.write_byte(effective_addr as u16, expected_value);

    match mnemonic {
        Mnemonic::LDA => expected_state.a = expected_value,
        Mnemonic::LDX => expected_state.x = expected_value,
        Mnemonic::LDY => expected_state.y = expected_value,
        _ => panic!("what the fuck is this mnemonic? {:?}", mnemonic),
    };
    expected_state.sr = transfer_flags_helper(expected_value);
    expected_state.pc = 1;

    assert_eq!(
        evaluate_load(&mut cpu, mnemonic, addressing_mode),
        InstructionResult {
            state: expected_state,
            cycles: if addressing_mode == AddressingMode::IndirectX {6} else {
                if (effective_addr &0xFF00) == (cpu.read_word_zp(addr as u8) &0xFF00) {5} else {6}
            },
            instruction_length: addressing_mode.instruction_length(),
        }
    )
}

#[test]
pub fn test_load_immediate() {
    let mut rng = rand::rng();
    let mnemonics: [Mnemonic; 3] = [Mnemonic::LDA, Mnemonic::LDX, Mnemonic::LDY];
    for mnemonic in mnemonics.iter() {
        test_load_immediate_helper(*mnemonic, rng.random::<u8>());
    }
}

#[test]
pub fn test_load_zeropage() {
    let mut rng = rand::rng();
    let mnemonics: [Mnemonic; 3] = [Mnemonic::LDA, Mnemonic::LDX, Mnemonic::LDY];
    for mnemonic in mnemonics.iter() {
        test_load_zeropage_helper(*mnemonic, rng.random::<u8>(), rng.random::<u8>(), AddressingMode::ZeroPage, 0);
    }
}

#[test]
pub fn test_load_zeropage_indexed() {
    let mut rng = rand::rng();
    let rand_num = rng.random::<u8>();
    // LDA
    test_load_zeropage_helper(Mnemonic::LDA, rand_num, rand_num, AddressingMode::ZeroPageX, rand_num);
    test_load_zeropage_helper(Mnemonic::LDX, rand_num, rand_num, AddressingMode::ZeroPageY, rand_num);
    test_load_zeropage_helper(Mnemonic::LDY, rand_num, rand_num, AddressingMode::ZeroPageX, rand_num);
}

#[test]
pub fn test_load_absolute() {
    let mut rng = rand::rng();
    let mnemonics: [Mnemonic; 3] = [Mnemonic::LDA, Mnemonic::LDX, Mnemonic::LDY];
    for mnemonic in mnemonics.iter() {
        test_load_absolute_helper(*mnemonic, rng.random::<u8>(), rng.random::<u16>(), AddressingMode::Absolute, 0);
    }
}

#[test]
pub fn test_load_absolute_indexed() {
    let mut rng = rand::rng();
    let rand_num = rng.random::<u8>();
    let rand_addr = rng.random::<u16>();
    // LDA
    test_load_absolute_helper(Mnemonic::LDA, rand_num, rand_addr, AddressingMode::AbsoluteX, rand_num);
    test_load_absolute_helper(Mnemonic::LDX, rand_num, rand_addr, AddressingMode::AbsoluteY, rand_num);
    test_load_absolute_helper(Mnemonic::LDY, rand_num, rand_addr, AddressingMode::AbsoluteX, rand_num);
}

#[test]
pub fn test_load_indirect_indexed() {
    let mut rng = rand::rng();
    let rand_num = rng.random::<u8>();
    let rand_addr = rng.random::<u16>();
    // LDA
    test_load_indirect_helper(Mnemonic::LDA, rand_num, rand_addr, AddressingMode::IndirectX, rand_num);
    test_load_indirect_helper(Mnemonic::LDA, rand_num, rand_addr, AddressingMode::IndirectY, rand_num);
}