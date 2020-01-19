use super::address_mode::AddressMode;
use super::state::State;
use crate::math;

pub fn fetch_byte(state: &State, addr_mode: AddressMode) -> u8 {
    match addr_mode {
        AddressMode::Implicit => panic!(
            "address mode {:?} has no fetch_byte implementation",
            addr_mode
        ),
        AddressMode::Accumulator => state.regs.a,
        AddressMode::Immediate => state.mem.read(state.regs.pc + 1),

        // TODO: zero page wraparound
        _ => state.mem.read(fetch_address(state, addr_mode)),
    }
}

pub fn fetch_address(state: &State, addr_mode: AddressMode) -> u16 {
    match addr_mode {
        AddressMode::ZeroPage => state.mem.read(state.regs.pc + 1) as u16,
        AddressMode::ZeroPageX => state.mem.read(state.regs.pc + 1) as u16 + state.regs.x as u16,
        AddressMode::ZeroPageY => state.mem.read(state.regs.pc + 1) as u16 + state.regs.y as u16,

        AddressMode::Relative => {
            math::byte_addr_offset(state.regs.pc, state.mem.read(state.regs.pc + 1))
        }
        AddressMode::Absolute => state.mem.read16(state.regs.pc + 1),
        AddressMode::AbsoluteX => state.mem.read16(state.regs.pc + 1) + state.regs.x as u16,
        AddressMode::AbsoluteY => state.mem.read16(state.regs.pc + 1) + state.regs.y as u16,
        AddressMode::Indirect => state.mem.read16(state.mem.read16(state.regs.pc + 1)),

        // TODO: zero page wraparound
        AddressMode::IndirectX => state
            .mem
            .read16(state.mem.read(state.regs.pc + 1) as u16 + state.regs.x as u16),
        AddressMode::IndirectY => {
            state.mem.read16(state.mem.read(state.regs.pc + 1) as u16) + state.regs.y as u16
        }

        other => panic!(
            "address mode {:?} has no fetch_address implementation",
            other
        ),
    }
}

#[test]
fn test_fetch_accumulator() {
    let mut state = State::new();
    state.regs.a = 0xAB;
    assert_eq!(fetch_byte(&state, AddressMode::Accumulator), 0xAB);
}

#[test]
fn test_fetch_immediate() {
    let mut state = State::new();
    state.regs.pc = 0x200;
    state.mem.write(0x201, 0xAB);
    assert_eq!(fetch_byte(&state, AddressMode::Immediate), 0xAB);
}

#[test]
fn test_fetch_zero_page() {
    let mut state = State::new();
    state.regs.pc = 0x200;
    state.mem.write(0xAB, 0xCD);
    state.mem.write(0x201, 0xAB);
    assert_eq!(fetch_byte(&state, AddressMode::ZeroPage), 0xCD);
}

#[test]
fn test_fetch_zero_page_x() {
    let mut state = State::new();
    state.regs.pc = 0x200;
    state.regs.x = 0x01;
    state.mem.write(0xAC, 0xCD);
    state.mem.write(0x201, 0xAB);
    assert_eq!(fetch_byte(&state, AddressMode::ZeroPageX), 0xCD);
}

#[test]
fn test_fetch_zero_page_y() {
    let mut state = State::new();
    state.regs.pc = 0x200;
    state.regs.y = 0x01;
    state.mem.write(0xAC, 0xCD);
    state.mem.write(0x201, 0xAB);
    assert_eq!(fetch_byte(&state, AddressMode::ZeroPageY), 0xCD);
}

#[test]
fn test_fetch_relative() {
    let mut state = State::new();
    state.regs.pc = 0x200;
    state.mem.write(0x201, 0xF0);
    assert_eq!(fetch_address(&state, AddressMode::Relative), 0x1F0);
    state.mem.write(0x201, 0x0F);
    assert_eq!(fetch_address(&state, AddressMode::Relative), 0x20F);
}

#[test]
fn test_fetch_absolute() {
    let mut state = State::new();
    state.regs.pc = 0x200;
    state.mem.write(0x201, 0xCD);
    state.mem.write(0x202, 0xAB);
    assert_eq!(fetch_address(&state, AddressMode::Absolute), 0xABCD);
}

#[test]
fn test_fetch_absolute_x() {
    let mut state = State::new();
    state.regs.pc = 0x200;
    state.regs.x = 0x1;
    state.mem.write(0x201, 0xCD);
    state.mem.write(0x202, 0xAB);
    assert_eq!(fetch_address(&state, AddressMode::AbsoluteX), 0xABCE);
}

#[test]
fn test_fetch_absolute_y() {
    let mut state = State::new();
    state.regs.pc = 0x200;
    state.regs.y = 0x1;
    state.mem.write(0x201, 0xCD);
    state.mem.write(0x202, 0xAB);
    assert_eq!(fetch_address(&state, AddressMode::AbsoluteY), 0xABCE);
}

#[test]
fn test_fetch_indirect() {
    let mut state = State::new();
    state.regs.pc = 0x200;
    state.mem.write(0x201, 0xFF);
    state.mem.write(0x202, 0x04);
    assert_eq!(fetch_address(&state, AddressMode::AbsoluteY), 0x4FF);
}

#[test]
fn test_fetch_indirect_x() {
    let mut state = State::new();
    state.regs.x = 0x02;
    state.regs.pc = 0x200;
    state.mem.write(0x201, 0x01);
    state.mem.write(0x03, 0x22);
    state.mem.write(0x04, 0x11);
    assert_eq!(fetch_address(&state, AddressMode::IndirectX), 0x1122);
}

#[test]
fn test_fetch_indirect_y() {
    let mut state = State::new();
    state.regs.y = 0x01;
    state.regs.pc = 0x200;
    state.mem.write(0x201, 0x01);
    state.mem.write(0x01, 0x22);
    state.mem.write(0x02, 0x11);
    assert_eq!(fetch_address(&state, AddressMode::IndirectY), 0x1123);
}
