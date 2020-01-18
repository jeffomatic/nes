use self::address_mode::AddressMode;
use self::opcode::Opcode;
use self::registers::Registers;
use super::math;
use super::memory::Memory;

mod address_mode;
mod opcode;
mod registers;
mod state;
mod status;
mod update;

// Reference: obelisk.me.uk/6502/reference.html
fn decode(raw_opcode: u8) -> Option<(Opcode, AddressMode)> {
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
        0x19 => Some((Opcode::Ora, AddressMode::AbsoluteX)),
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Opval {
    Byte(u8),
    Address(u16),
}

fn fetch_operand(regs: &Registers, mem: &Memory, addr_mode: AddressMode) -> Option<Opval> {
    match addr_mode {
        AddressMode::Implicit => None,
        AddressMode::Accumulator => Some(Opval::Byte(regs.a)),
        AddressMode::Immediate => Some(Opval::Byte(mem.read(regs.pc + 1))),
        AddressMode::ZeroPage => Some(Opval::Byte(mem.read(mem.read(regs.pc + 1) as u16))),
        AddressMode::ZeroPageX => Some(Opval::Byte(
            mem.read(mem.read(regs.pc + 1) as u16 + regs.x as u16),
        )),
        AddressMode::ZeroPageY => Some(Opval::Byte(
            mem.read(mem.read(regs.pc + 1) as u16 + regs.y as u16),
        )),
        AddressMode::Relative => Some(Opval::Address(math::byte_addr_offset(
            regs.pc,
            mem.read(regs.pc + 1),
        ))),
        _ => unimplemented!(),
    }
}

#[test]
fn test_fetch_operand() {
    let mut regs = Registers::default();
    regs.a = 0x01;
    regs.x = 0x02;
    regs.y = 0x03;
    regs.pc = 0x200;

    let mut mem = Memory::new();
    mem.write(0xEE, 0xAA);
    mem.write(0xF0, 0xBB);
    mem.write(0xF1, 0xCC);
    mem.write(0x201, 0xEE);

    assert_eq!(fetch_operand(&regs, &mem, AddressMode::Implicit), None);

    assert_eq!(
        fetch_operand(&regs, &mem, AddressMode::Accumulator),
        Some(Opval::Byte(0x01))
    );

    assert_eq!(
        fetch_operand(&regs, &mem, AddressMode::Immediate),
        Some(Opval::Byte(0xEE))
    );

    assert_eq!(
        fetch_operand(&regs, &mem, AddressMode::ZeroPage),
        Some(Opval::Byte(0xAA))
    );

    assert_eq!(
        fetch_operand(&regs, &mem, AddressMode::ZeroPageX),
        Some(Opval::Byte(0xBB))
    );

    assert_eq!(
        fetch_operand(&regs, &mem, AddressMode::ZeroPageY),
        Some(Opval::Byte(0xCC))
    );

    assert_eq!(
        fetch_operand(&regs, &mem, AddressMode::Relative),
        Some(Opval::Address(0x1EE))
    );
}

fn run(state: &mut state::State) {
    let mut cycle: u64 = 0;
    let mut opbytes = 0;
    loop {
        let (opcode, addr_mode) = decode(state.mem.read(state.regs.pc)).unwrap();

        cycle += 1;
    }
}
