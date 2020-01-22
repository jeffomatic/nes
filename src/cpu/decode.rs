use super::opcode::Opcode;
use super::operand::Operand;
use super::state::State;
use crate::math;

// Reference: http://obelisk.me.uk/6502/addressing.html
#[derive(Clone, Copy, Debug)]
enum AddressMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX, // aka "indexed indirect"
    IndirectY, // aka "indirect indexed"
}

pub fn decode(state: &mut State) -> Option<(Opcode, Operand)> {
    let (opcode, addr_mode) = decode_raw_opcode(state.consume_instruction_byte())?;
    let operand = decode_operand(state, addr_mode);
    Some((opcode, operand))
}

// Reference: obelisk.me.uk/6502/reference.html
fn decode_raw_opcode(raw_opcode: u8) -> Option<(Opcode, AddressMode)> {
    match raw_opcode {
        0x00 => Some((Opcode::Brk, AddressMode::Implicit)),
        0x01 => Some((Opcode::Ora, AddressMode::IndirectX)),
        0x05 => Some((Opcode::Ora, AddressMode::ZeroPage)),
        0x06 => Some((Opcode::Asl, AddressMode::ZeroPage)),
        0x08 => Some((Opcode::Php, AddressMode::Implicit)),
        0x09 => Some((Opcode::Ora, AddressMode::Immediate)),
        0x0A => Some((Opcode::Asl, AddressMode::Accumulator)),
        0x0D => Some((Opcode::Ora, AddressMode::Absolute)),
        0x0E => Some((Opcode::Asl, AddressMode::Absolute)),
        0x10 => Some((Opcode::Bpl, AddressMode::Relative)),
        0x11 => Some((Opcode::Ora, AddressMode::IndirectY)),
        0x15 => Some((Opcode::Ora, AddressMode::ZeroPageX)),
        0x16 => Some((Opcode::Asl, AddressMode::ZeroPageX)),
        0x18 => Some((Opcode::Clc, AddressMode::Implicit)),
        0x19 => Some((Opcode::Ora, AddressMode::AbsoluteY)),
        0x1D => Some((Opcode::Ora, AddressMode::AbsoluteX)),
        0x1E => Some((Opcode::Asl, AddressMode::AbsoluteX)),
        0x20 => Some((Opcode::Jsr, AddressMode::Absolute)),
        0x21 => Some((Opcode::And, AddressMode::IndirectX)),
        0x24 => Some((Opcode::Bit, AddressMode::ZeroPage)),
        0x25 => Some((Opcode::And, AddressMode::ZeroPage)),
        0x26 => Some((Opcode::Rol, AddressMode::ZeroPage)),
        0x28 => Some((Opcode::Plp, AddressMode::Implicit)),
        0x29 => Some((Opcode::And, AddressMode::Immediate)),
        0x2A => Some((Opcode::Rol, AddressMode::Accumulator)),
        0x2C => Some((Opcode::Bit, AddressMode::Absolute)),
        0x2D => Some((Opcode::And, AddressMode::Absolute)),
        0x2E => Some((Opcode::Rol, AddressMode::Absolute)),
        0x30 => Some((Opcode::Bmi, AddressMode::Relative)),
        0x31 => Some((Opcode::And, AddressMode::IndirectY)),
        0x35 => Some((Opcode::And, AddressMode::ZeroPageX)),
        0x36 => Some((Opcode::Rol, AddressMode::ZeroPageX)),
        0x38 => Some((Opcode::Sec, AddressMode::Implicit)),
        0x39 => Some((Opcode::And, AddressMode::AbsoluteY)),
        0x3D => Some((Opcode::And, AddressMode::AbsoluteX)),
        0x3E => Some((Opcode::Rol, AddressMode::AbsoluteX)),
        0x40 => Some((Opcode::Rti, AddressMode::Implicit)),
        0x41 => Some((Opcode::Eor, AddressMode::IndirectX)),
        0x45 => Some((Opcode::Eor, AddressMode::ZeroPage)),
        0x46 => Some((Opcode::Lsr, AddressMode::ZeroPage)),
        0x48 => Some((Opcode::Pha, AddressMode::Implicit)),
        0x49 => Some((Opcode::Eor, AddressMode::Immediate)),
        0x4A => Some((Opcode::Lsr, AddressMode::Accumulator)),
        0x4C => Some((Opcode::Jmp, AddressMode::Absolute)),
        0x4D => Some((Opcode::Eor, AddressMode::Absolute)),
        0x4E => Some((Opcode::Lsr, AddressMode::Absolute)),
        0x50 => Some((Opcode::Bvc, AddressMode::Relative)),
        0x51 => Some((Opcode::Eor, AddressMode::IndirectY)),
        0x55 => Some((Opcode::Eor, AddressMode::ZeroPageX)),
        0x56 => Some((Opcode::Lsr, AddressMode::ZeroPageX)),
        0x58 => Some((Opcode::Cli, AddressMode::Implicit)),
        0x59 => Some((Opcode::Eor, AddressMode::AbsoluteY)),
        0x5D => Some((Opcode::Eor, AddressMode::AbsoluteX)),
        0x5E => Some((Opcode::Lsr, AddressMode::AbsoluteX)),
        0x60 => Some((Opcode::Rts, AddressMode::Implicit)),
        0x61 => Some((Opcode::Adc, AddressMode::IndirectX)),
        0x65 => Some((Opcode::Adc, AddressMode::ZeroPage)),
        0x66 => Some((Opcode::Ror, AddressMode::ZeroPage)),
        0x68 => Some((Opcode::Pla, AddressMode::Implicit)),
        0x69 => Some((Opcode::Adc, AddressMode::Immediate)),
        0x6A => Some((Opcode::Ror, AddressMode::Accumulator)),
        0x6D => Some((Opcode::Adc, AddressMode::Absolute)),
        0x6E => Some((Opcode::Ror, AddressMode::Absolute)),
        0x70 => Some((Opcode::Bvs, AddressMode::Relative)),
        0x71 => Some((Opcode::Adc, AddressMode::IndirectY)),
        0x75 => Some((Opcode::Adc, AddressMode::ZeroPageX)),
        0x76 => Some((Opcode::Ror, AddressMode::ZeroPageX)),
        0x78 => Some((Opcode::Sei, AddressMode::Implicit)),
        0x79 => Some((Opcode::Adc, AddressMode::AbsoluteY)),
        0x7D => Some((Opcode::Adc, AddressMode::AbsoluteX)),
        0x7E => Some((Opcode::Ror, AddressMode::AbsoluteX)),
        0x81 => Some((Opcode::Sta, AddressMode::IndirectX)),
        0x84 => Some((Opcode::Sty, AddressMode::ZeroPage)),
        0x85 => Some((Opcode::Sta, AddressMode::ZeroPage)),
        0x86 => Some((Opcode::Stx, AddressMode::ZeroPage)),
        0x88 => Some((Opcode::Dey, AddressMode::Implicit)),
        0x8A => Some((Opcode::Txa, AddressMode::Implicit)),
        0x8C => Some((Opcode::Sty, AddressMode::Absolute)),
        0x8D => Some((Opcode::Sta, AddressMode::Absolute)),
        0x8E => Some((Opcode::Stx, AddressMode::Absolute)),
        0x90 => Some((Opcode::Bcc, AddressMode::Relative)),
        0x91 => Some((Opcode::Sta, AddressMode::IndirectY)),
        0x94 => Some((Opcode::Sty, AddressMode::ZeroPageX)),
        0x95 => Some((Opcode::Sta, AddressMode::ZeroPageX)),
        0x96 => Some((Opcode::Stx, AddressMode::ZeroPageY)),
        0x98 => Some((Opcode::Tya, AddressMode::Implicit)),
        0x99 => Some((Opcode::Sta, AddressMode::AbsoluteY)),
        0x9A => Some((Opcode::Txs, AddressMode::Implicit)),
        0x9D => Some((Opcode::Sta, AddressMode::AbsoluteX)),
        0xA0 => Some((Opcode::Ldy, AddressMode::Immediate)),
        0xA1 => Some((Opcode::Lda, AddressMode::IndirectX)),
        0xA2 => Some((Opcode::Ldx, AddressMode::Immediate)),
        0xA4 => Some((Opcode::Ldy, AddressMode::ZeroPage)),
        0xA5 => Some((Opcode::Lda, AddressMode::ZeroPage)),
        0xA6 => Some((Opcode::Ldx, AddressMode::ZeroPage)),
        0xA8 => Some((Opcode::Tay, AddressMode::Implicit)),
        0xA9 => Some((Opcode::Lda, AddressMode::Immediate)),
        0xAA => Some((Opcode::Tax, AddressMode::Implicit)),
        0xAC => Some((Opcode::Ldy, AddressMode::Absolute)),
        0xAD => Some((Opcode::Lda, AddressMode::Absolute)),
        0xAE => Some((Opcode::Ldx, AddressMode::Absolute)),
        0xB0 => Some((Opcode::Bcs, AddressMode::Relative)),
        0xB1 => Some((Opcode::Lda, AddressMode::IndirectY)),
        0xB4 => Some((Opcode::Ldy, AddressMode::ZeroPageX)),
        0xB5 => Some((Opcode::Lda, AddressMode::ZeroPageX)),
        0xB6 => Some((Opcode::Ldx, AddressMode::ZeroPageY)),
        0xB8 => Some((Opcode::Clv, AddressMode::Implicit)),
        0xB9 => Some((Opcode::Lda, AddressMode::AbsoluteY)),
        0xBA => Some((Opcode::Tsx, AddressMode::Implicit)),
        0xBC => Some((Opcode::Ldy, AddressMode::AbsoluteX)),
        0xBD => Some((Opcode::Lda, AddressMode::AbsoluteX)),
        0xBE => Some((Opcode::Ldx, AddressMode::AbsoluteY)),
        0xC0 => Some((Opcode::Cpy, AddressMode::Immediate)),
        0xC1 => Some((Opcode::Cmp, AddressMode::IndirectX)),
        0xC4 => Some((Opcode::Cpy, AddressMode::ZeroPage)),
        0xC5 => Some((Opcode::Cmp, AddressMode::ZeroPage)),
        0xC6 => Some((Opcode::Dec, AddressMode::ZeroPage)),
        0xC8 => Some((Opcode::Iny, AddressMode::Implicit)),
        0xC9 => Some((Opcode::Cmp, AddressMode::Immediate)),
        0xCA => Some((Opcode::Dex, AddressMode::Implicit)),
        0xCC => Some((Opcode::Cpy, AddressMode::Absolute)),
        0xCD => Some((Opcode::Cmp, AddressMode::Absolute)),
        0xCE => Some((Opcode::Dec, AddressMode::Absolute)),
        0xD0 => Some((Opcode::Bne, AddressMode::Relative)),
        0xD1 => Some((Opcode::Cmp, AddressMode::IndirectY)),
        0xD5 => Some((Opcode::Cmp, AddressMode::ZeroPageX)),
        0xD6 => Some((Opcode::Dec, AddressMode::ZeroPageX)),
        0xD8 => Some((Opcode::Cld, AddressMode::Implicit)),
        0xD9 => Some((Opcode::Cmp, AddressMode::AbsoluteY)),
        0xDD => Some((Opcode::Cmp, AddressMode::AbsoluteX)),
        0xDE => Some((Opcode::Dec, AddressMode::AbsoluteX)),
        0xE0 => Some((Opcode::Cpx, AddressMode::Immediate)),
        0xE1 => Some((Opcode::Sbc, AddressMode::IndirectX)),
        0xE4 => Some((Opcode::Cpx, AddressMode::ZeroPage)),
        0xE5 => Some((Opcode::Sbc, AddressMode::ZeroPage)),
        0xE6 => Some((Opcode::Inc, AddressMode::ZeroPage)),
        0xE8 => Some((Opcode::Inx, AddressMode::Implicit)),
        0xE9 => Some((Opcode::Sbc, AddressMode::Immediate)),
        0xEA => Some((Opcode::Nop, AddressMode::Implicit)),
        0xEC => Some((Opcode::Cpx, AddressMode::Absolute)),
        0xED => Some((Opcode::Sbc, AddressMode::Absolute)),
        0xEE => Some((Opcode::Inc, AddressMode::Absolute)),
        0xF0 => Some((Opcode::Beq, AddressMode::Relative)),
        0xF1 => Some((Opcode::Sbc, AddressMode::IndirectY)),
        0xF5 => Some((Opcode::Sbc, AddressMode::ZeroPageX)),
        0xF6 => Some((Opcode::Inc, AddressMode::ZeroPageX)),
        0xF8 => Some((Opcode::Sed, AddressMode::Implicit)),
        0xF9 => Some((Opcode::Sbc, AddressMode::AbsoluteY)),
        0xFD => Some((Opcode::Sbc, AddressMode::AbsoluteX)),
        0xFE => Some((Opcode::Inc, AddressMode::AbsoluteX)),
        _ => None,
    }
}

fn decode_operand(state: &mut State, addr_mode: AddressMode) -> Operand {
    match addr_mode {
        AddressMode::Implicit => Operand::None,
        AddressMode::Accumulator => Operand::Accumulator,
        AddressMode::Immediate => Operand::Immediate(state.consume_instruction_byte()),
        AddressMode::ZeroPage => Operand::Memory(state.consume_instruction_byte() as u16),
        AddressMode::ZeroPageX => {
            Operand::Memory(state.regs.x.wrapping_add(state.consume_instruction_byte()) as u16)
        }
        AddressMode::ZeroPageY => {
            Operand::Memory(state.regs.y.wrapping_add(state.consume_instruction_byte()) as u16)
        }
        AddressMode::Relative => Operand::Immediate(state.consume_instruction_byte()),
        AddressMode::Absolute => Operand::Memory(math::bytes_to_u16_le([
            state.consume_instruction_byte(),
            state.consume_instruction_byte(),
        ])),
        AddressMode::AbsoluteX => Operand::Memory(
            math::bytes_to_u16_le([
                state.consume_instruction_byte(),
                state.consume_instruction_byte(),
            ]) + state.regs.x as u16,
        ),
        AddressMode::AbsoluteY => Operand::Memory(
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
            Operand::Memory(state.mem_read16(math::bytes_to_u16_le(bytes)))
        }
        AddressMode::IndirectX => {
            let offset = state.consume_instruction_byte();
            Operand::Memory(state.mem_read16(state.regs.x.wrapping_add(offset) as u16))
        }
        AddressMode::IndirectY => {
            let base = state.consume_instruction_byte() as u16;
            Operand::Memory(state.mem_read16(base) + state.regs.y as u16)
        }
    }
}

#[test]
fn test_decode_implicit() {
    let mut state = State::new();
    assert_eq!(
        decode_operand(&mut state, AddressMode::Implicit),
        Operand::None
    );
    assert_eq!(state.regs.pc, 0);
}

#[test]
fn test_decode_accumulator() {
    let mut state = State::new();
    assert_eq!(
        decode_operand(&mut state, AddressMode::Accumulator),
        Operand::Accumulator
    );
    assert_eq!(state.regs.pc, 0);
}

#[test]
fn test_decode_immediate() {
    let mut state = State::new();
    state.mem_write(0, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::Immediate),
        Operand::Immediate(0xAB),
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_zero_page() {
    let mut state = State::new();
    state.mem_write(0, 0x1F);
    state.mem_write(0x1F, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::ZeroPage),
        Operand::Memory(0x1F)
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_zero_page_x() {
    let mut state = State::new();
    state.regs.x = 1;
    state.mem_write(0, 0x10);
    assert_eq!(
        decode_operand(&mut state, AddressMode::ZeroPageX),
        Operand::Memory(0x11)
    );
    assert_eq!(state.regs.pc, 1);

    // zero-page wrapping
    let mut state = State::new();
    state.regs.x = 2;
    state.mem_write(0, 0xFF);
    assert_eq!(
        decode_operand(&mut state, AddressMode::ZeroPageX),
        Operand::Memory(0x01)
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_zero_page_y() {
    let mut state = State::new();
    state.regs.y = 1;
    state.mem_write(0, 0x10);
    assert_eq!(
        decode_operand(&mut state, AddressMode::ZeroPageY),
        Operand::Memory(0x11)
    );
    assert_eq!(state.regs.pc, 1);

    // zero-page wrapping
    let mut state = State::new();
    state.regs.y = 2;
    state.mem_write(0, 0xFF);
    assert_eq!(
        decode_operand(&mut state, AddressMode::ZeroPageY),
        Operand::Memory(0x01)
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_relative() {
    let mut state = State::new();
    state.mem_write(0, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::Relative),
        Operand::Immediate(0xAB),
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_absolute() {
    let mut state = State::new();
    state.mem_write(0, 0xCD);
    state.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::Absolute),
        Operand::Memory(0xABCD)
    );
    assert_eq!(state.regs.pc, 2);
}

#[test]
fn test_decode_absolute_x() {
    let mut state = State::new();
    state.regs.x = 0x1;
    state.mem_write(0, 0xCD);
    state.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::AbsoluteX),
        Operand::Memory(0xABCE)
    );
    assert_eq!(state.regs.pc, 2);
}

#[test]
fn test_decode_absolute_y() {
    let mut state = State::new();
    state.regs.y = 0x1;
    state.mem_write(0, 0xCD);
    state.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::AbsoluteY),
        Operand::Memory(0xABCE)
    );
    assert_eq!(state.regs.pc, 2);
}

#[test]
fn test_decode_indirect() {
    let mut state = State::new();
    state.regs.x = 1;
    state.mem_write(0, 0xFF);
    state.mem_write(1, 1);
    state.mem_write(0x1FF, 0xCD);
    state.mem_write(0x200, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::Indirect),
        Operand::Memory(0xABCD)
    );
    assert_eq!(state.regs.pc, 2);
}

#[test]
fn test_decode_indirect_x() {
    let mut state = State::new();
    state.regs.x = 1;
    state.mem_write(0, 0xF);
    state.mem_write(0x10, 0xCD);
    state.mem_write(0x11, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::IndirectX),
        Operand::Memory(0xABCD)
    );
    assert_eq!(state.regs.pc, 1);

    // zero-page wrapping
    let mut state = State::new();
    state.regs.x = 2;
    state.mem_write(0, 0xFF);
    state.mem_write(1, 0xCD);
    state.mem_write(2, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::IndirectX),
        Operand::Memory(0xABCD)
    );
    assert_eq!(state.regs.pc, 1);
}

#[test]
fn test_decode_indirect_y() {
    let mut state = State::new();
    state.regs.y = 1;
    state.mem_write(0, 0xF);
    state.mem_write(0xF, 0xCD);
    state.mem_write(0x10, 0xAB);
    assert_eq!(
        decode_operand(&mut state, AddressMode::IndirectY),
        Operand::Memory(0xABCE)
    );
    assert_eq!(state.regs.pc, 1);
}
