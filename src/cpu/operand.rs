use super::address_mode::AddressMode;
use super::state::State;
use crate::math;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operand {
    None,
    Accumulator,
    Immediate(u8),
    Memory(u16),
}

impl Operand {
    pub fn read(&self, state: &State) -> u8 {
        match self {
            Self::Accumulator => state.regs.a,
            Self::Immediate(val) => *val,
            Self::Memory(addr) => state.mem.read(*addr),
            other => panic!("no readable value for {:?} operand", other),
        }
    }

    pub fn write(&self, state: &mut State, val: u8) {
        match self {
            Self::Accumulator => state.regs.a = val,
            Self::Memory(addr) => state.mem.write(*addr, val),
            other => panic!("no writable value for {:?} operand", other),
        }
    }

    pub fn decode(state: &mut State, addr_mode: AddressMode) -> Operand {
        match addr_mode {
            AddressMode::Implicit => Self::None,
            AddressMode::Accumulator => Self::Accumulator,
            AddressMode::Immediate => Self::Immediate(state.consume_instruction_byte()),
            AddressMode::ZeroPage => Self::Memory(state.consume_instruction_byte() as u16),
            AddressMode::ZeroPageX => {
                Self::Memory(state.consume_instruction_byte() as u16 + state.regs.x as u16)
            }
            AddressMode::ZeroPageY => {
                Self::Memory(state.consume_instruction_byte() as u16 + state.regs.y as u16)
            }
            AddressMode::Relative => Self::Immediate(state.consume_instruction_byte()),
            AddressMode::Absolute => Self::Memory(math::bytes_to_u16_le([
                state.consume_instruction_byte(),
                state.consume_instruction_byte(),
            ])),
            AddressMode::AbsoluteX => Self::Memory(
                math::bytes_to_u16_le([
                    state.consume_instruction_byte(),
                    state.consume_instruction_byte(),
                ]) + state.regs.x as u16,
            ),
            AddressMode::AbsoluteY => Self::Memory(
                math::bytes_to_u16_le([
                    state.consume_instruction_byte(),
                    state.consume_instruction_byte(),
                ]) + state.regs.y as u16,
            ),
            AddressMode::Indirect => {
                let bytes = [
                    state.consume_instruction_byte(),
                    state.consume_instruction_byte(),
                ];
                Self::Memory(state.mem.read16(math::bytes_to_u16_le(bytes)))
            }
            AddressMode::IndirectX => {
                let base = state.consume_instruction_byte() as u16;
                Self::Memory(state.mem.read16(base + state.regs.x as u16))
            }
            AddressMode::IndirectY => {
                let base = state.consume_instruction_byte() as u16;
                Self::Memory(state.mem.read16(base) + state.regs.y as u16)
            }
        }
    }
}

#[test]
fn test_operand_accumulator() {
    let mut state = State::new();
    state.regs.a = 0xAB;

    let op = Operand::Accumulator;
    assert_eq!(op.read(&state), 0xAB);

    op.write(&mut state, 0xCD);
    assert_eq!(state.regs.a, 0xCD);
}

#[test]
fn test_operand_immediate() {
    let op = Operand::Immediate(0xAB);
    assert_eq!(op.read(&State::new()), 0xAB);
}

#[test]
fn test_operand_memory() {
    let mut state = State::new();
    state.mem.write(0x1F, 0xAB);

    let op = Operand::Memory(0x1F);
    assert_eq!(op.read(&state), 0xAB);

    op.write(&mut state, 0xCD);
    assert_eq!(state.mem.read(0x1F), 0xCD);
}

#[test]
fn test_decode_implicit() {
    let mut state = State::new();
    assert_eq!(
        Operand::decode(&mut state, AddressMode::Implicit),
        Operand::None
    );
    assert_eq!(state.regs.pc, 0);
}

#[test]
fn test_decode_accumulator() {
    let mut state = State::new();
    assert_eq!(
        Operand::decode(&mut state, AddressMode::Accumulator),
        Operand::Accumulator
    );
    assert_eq!(state.regs.pc, 0);
}

#[test]
fn test_decode_immediate() {
    let mut state = State::new();
    state.mem.write(0, 0xAB);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::Immediate),
        Operand::Immediate(0xAB),
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_zero_page() {
    let mut state = State::new();
    state.mem.write(0, 0x1F);
    state.mem.write(0x1F, 0xAB);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::ZeroPage),
        Operand::Memory(0x1F)
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_zero_page_x() {
    let mut state = State::new();
    state.regs.x = 1;
    state.mem.write(0, 0x10);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::ZeroPageX),
        Operand::Memory(0x11)
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_zero_page_y() {
    let mut state = State::new();
    state.regs.y = 1;
    state.mem.write(0, 0x10);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::ZeroPageY),
        Operand::Memory(0x11)
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_relative() {
    let mut state = State::new();
    state.mem.write(0, 0xAB);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::Relative),
        Operand::Immediate(0xAB),
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_absolute() {
    let mut state = State::new();
    state.mem.write(0, 0xCD);
    state.mem.write(1, 0xAB);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::Absolute),
        Operand::Memory(0xABCD)
    );
    assert_eq!(state.regs.pc, 2);
}

#[test]
fn test_decode_absolute_x() {
    let mut state = State::new();
    state.regs.x = 0x1;
    state.mem.write(0, 0xCD);
    state.mem.write(1, 0xAB);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::AbsoluteX),
        Operand::Memory(0xABCE)
    );
}

#[test]
fn test_decode_absolute_y() {
    let mut state = State::new();
    state.regs.y = 0x1;
    state.mem.write(0, 0xCD);
    state.mem.write(1, 0xAB);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::AbsoluteY),
        Operand::Memory(0xABCE)
    );
}

#[test]
fn test_decode_indirect() {
    let mut state = State::new();
    state.regs.x = 1;
    state.mem.write(0, 0xFF);
    state.mem.write(1, 1);
    state.mem.write(0x1FF, 0xCD);
    state.mem.write(0x200, 0xAB);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::Indirect),
        Operand::Memory(0xABCD)
    );
}

#[test]
fn test_decode_indirect_x() {
    let mut state = State::new();
    state.regs.x = 1;
    state.mem.write(0, 0xF);
    state.mem.write(0x10, 0xCD);
    state.mem.write(0x11, 0xAB);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::IndirectX),
        Operand::Memory(0xABCD)
    );
}

#[test]
fn test_decode_indirect_y() {
    let mut state = State::new();
    state.regs.y = 1;
    state.mem.write(0, 0xF);
    state.mem.write(0xF, 0xCD);
    state.mem.write(0x10, 0xAB);
    assert_eq!(
        Operand::decode(&mut state, AddressMode::IndirectY),
        Operand::Memory(0xABCE)
    );
}
