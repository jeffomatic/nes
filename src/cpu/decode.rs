use super::opcode::Opcode;
use super::operand::Operand;
use super::state::Cpu;
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

/// Decodes the instruction at the PC and returns a tuple containing the opcode,
/// operand, and base cycle cost. The PC will be incremented to the start of the
/// next instruction.
pub fn decode(cpu: &mut Cpu) -> Option<(Opcode, Operand, u64)> {
    let (opcode, addr_mode, cycles) = decode_raw_opcode(cpu.consume_instruction_byte())?;
    let (operand, cycle_adjust) = decode_operand(cpu, opcode, addr_mode);
    Some((opcode, operand, cycles + cycle_adjust))
}

/// Takes a encoded opcode and converts it to a tuple containing the opcode,
/// addressing mode, and base cycle cost.
///
/// Reference: obelisk.me.uk/6502/reference.html
fn decode_raw_opcode(raw_opcode: u8) -> Option<(Opcode, AddressMode, u64)> {
    match raw_opcode {
        // ADC
        0x69 => Some((Opcode::Adc, AddressMode::Immediate, 2)),
        0x65 => Some((Opcode::Adc, AddressMode::ZeroPage, 3)),
        0x75 => Some((Opcode::Adc, AddressMode::ZeroPageX, 4)),
        0x6D => Some((Opcode::Adc, AddressMode::Absolute, 4)),
        0x7D => Some((Opcode::Adc, AddressMode::AbsoluteX, 4)),
        0x79 => Some((Opcode::Adc, AddressMode::AbsoluteY, 4)),
        0x61 => Some((Opcode::Adc, AddressMode::IndirectX, 6)),
        0x71 => Some((Opcode::Adc, AddressMode::IndirectY, 5)),

        // AND
        0x29 => Some((Opcode::And, AddressMode::Immediate, 2)),
        0x25 => Some((Opcode::And, AddressMode::ZeroPage, 3)),
        0x35 => Some((Opcode::And, AddressMode::ZeroPageX, 4)),
        0x2D => Some((Opcode::And, AddressMode::Absolute, 4)),
        0x3D => Some((Opcode::And, AddressMode::AbsoluteX, 4)),
        0x39 => Some((Opcode::And, AddressMode::AbsoluteY, 4)),
        0x21 => Some((Opcode::And, AddressMode::IndirectX, 6)),
        0x31 => Some((Opcode::And, AddressMode::IndirectY, 5)),

        // ASL
        0x0A => Some((Opcode::Asl, AddressMode::Accumulator, 2)),
        0x06 => Some((Opcode::Asl, AddressMode::ZeroPage, 5)),
        0x16 => Some((Opcode::Asl, AddressMode::ZeroPageX, 6)),
        0x0E => Some((Opcode::Asl, AddressMode::Absolute, 6)),
        0x1E => Some((Opcode::Asl, AddressMode::AbsoluteX, 7)),

        // BCC
        0x90 => Some((Opcode::Bcc, AddressMode::Relative, 2)),

        // BCS
        0xB0 => Some((Opcode::Bcs, AddressMode::Relative, 2)),

        // BEQ
        0xF0 => Some((Opcode::Beq, AddressMode::Relative, 2)),

        // BIT
        0x24 => Some((Opcode::Bit, AddressMode::ZeroPage, 3)),
        0x2C => Some((Opcode::Bit, AddressMode::Absolute, 4)),

        // BMI
        0x30 => Some((Opcode::Bmi, AddressMode::Relative, 2)),

        // BNE
        0xD0 => Some((Opcode::Bne, AddressMode::Relative, 2)),

        // BPL
        0x10 => Some((Opcode::Bpl, AddressMode::Relative, 2)),

        // BRK
        0x00 => Some((Opcode::Brk, AddressMode::Implicit, 7)),

        // BVC
        0x50 => Some((Opcode::Bvc, AddressMode::Relative, 2)),

        // BVS
        0x70 => Some((Opcode::Bvs, AddressMode::Relative, 2)),

        // CLC
        0x18 => Some((Opcode::Clc, AddressMode::Implicit, 2)),

        // CLD
        0xD8 => Some((Opcode::Cld, AddressMode::Implicit, 2)),

        // CLI
        0x58 => Some((Opcode::Cli, AddressMode::Implicit, 2)),

        // CLV
        0xB8 => Some((Opcode::Clv, AddressMode::Implicit, 2)),

        // CMP
        0xC9 => Some((Opcode::Cmp, AddressMode::Immediate, 2)),
        0xC5 => Some((Opcode::Cmp, AddressMode::ZeroPage, 3)),
        0xD5 => Some((Opcode::Cmp, AddressMode::ZeroPageX, 4)),
        0xCD => Some((Opcode::Cmp, AddressMode::Absolute, 4)),
        0xDD => Some((Opcode::Cmp, AddressMode::AbsoluteX, 4)),
        0xD9 => Some((Opcode::Cmp, AddressMode::AbsoluteY, 4)),
        0xC1 => Some((Opcode::Cmp, AddressMode::IndirectX, 6)),
        0xD1 => Some((Opcode::Cmp, AddressMode::IndirectY, 5)),

        // CPX
        0xE0 => Some((Opcode::Cpx, AddressMode::Immediate, 2)),
        0xE4 => Some((Opcode::Cpx, AddressMode::ZeroPage, 3)),
        0xEC => Some((Opcode::Cpx, AddressMode::Absolute, 4)),

        // CPY
        0xC0 => Some((Opcode::Cpy, AddressMode::Immediate, 2)),
        0xC4 => Some((Opcode::Cpy, AddressMode::ZeroPage, 3)),
        0xCC => Some((Opcode::Cpy, AddressMode::Absolute, 4)),

        // DEC
        0xC6 => Some((Opcode::Dec, AddressMode::ZeroPage, 5)),
        0xD6 => Some((Opcode::Dec, AddressMode::ZeroPageX, 6)),
        0xCE => Some((Opcode::Dec, AddressMode::Absolute, 6)),
        0xDE => Some((Opcode::Dec, AddressMode::AbsoluteX, 7)),

        // DEX
        0xCA => Some((Opcode::Dex, AddressMode::Implicit, 2)),

        // DEY
        0x88 => Some((Opcode::Dey, AddressMode::Implicit, 2)),

        // EOR
        0x49 => Some((Opcode::Eor, AddressMode::Immediate, 2)),
        0x45 => Some((Opcode::Eor, AddressMode::ZeroPage, 3)),
        0x55 => Some((Opcode::Eor, AddressMode::ZeroPageX, 4)),
        0x4D => Some((Opcode::Eor, AddressMode::Absolute, 4)),
        0x5D => Some((Opcode::Eor, AddressMode::AbsoluteX, 4)),
        0x59 => Some((Opcode::Eor, AddressMode::AbsoluteY, 4)),
        0x41 => Some((Opcode::Eor, AddressMode::IndirectX, 6)),
        0x51 => Some((Opcode::Eor, AddressMode::IndirectY, 5)),

        // INC
        0xE6 => Some((Opcode::Inc, AddressMode::ZeroPage, 5)),
        0xF6 => Some((Opcode::Inc, AddressMode::ZeroPageX, 6)),
        0xEE => Some((Opcode::Inc, AddressMode::Absolute, 6)),
        0xFE => Some((Opcode::Inc, AddressMode::AbsoluteX, 7)),

        // INX
        0xE8 => Some((Opcode::Inx, AddressMode::Implicit, 2)),

        // INY
        0xC8 => Some((Opcode::Iny, AddressMode::Implicit, 2)),

        // JMP
        0x4C => Some((Opcode::Jmp, AddressMode::Absolute, 3)),
        0x6C => Some((Opcode::Jmp, AddressMode::Indirect, 5)),

        // JSR
        0x20 => Some((Opcode::Jsr, AddressMode::Absolute, 6)),

        // LDA
        0xA9 => Some((Opcode::Lda, AddressMode::Immediate, 2)),
        0xA5 => Some((Opcode::Lda, AddressMode::ZeroPage, 3)),
        0xB5 => Some((Opcode::Lda, AddressMode::ZeroPageX, 4)),
        0xAD => Some((Opcode::Lda, AddressMode::Absolute, 4)),
        0xBD => Some((Opcode::Lda, AddressMode::AbsoluteX, 4)),
        0xB9 => Some((Opcode::Lda, AddressMode::AbsoluteY, 4)),
        0xA1 => Some((Opcode::Lda, AddressMode::IndirectX, 6)),
        0xB1 => Some((Opcode::Lda, AddressMode::IndirectY, 5)),

        // LDX
        0xA2 => Some((Opcode::Ldx, AddressMode::Immediate, 2)),
        0xA6 => Some((Opcode::Ldx, AddressMode::ZeroPage, 3)),
        0xB6 => Some((Opcode::Ldx, AddressMode::ZeroPageY, 4)),
        0xAE => Some((Opcode::Ldx, AddressMode::Absolute, 4)),
        0xBE => Some((Opcode::Ldx, AddressMode::AbsoluteY, 4)),

        // LDY
        0xA0 => Some((Opcode::Ldy, AddressMode::Immediate, 2)),
        0xA4 => Some((Opcode::Ldy, AddressMode::ZeroPage, 3)),
        0xB4 => Some((Opcode::Ldy, AddressMode::ZeroPageX, 4)),
        0xAC => Some((Opcode::Ldy, AddressMode::Absolute, 4)),
        0xBC => Some((Opcode::Ldy, AddressMode::AbsoluteX, 4)),

        // LSR
        0x4A => Some((Opcode::Lsr, AddressMode::Accumulator, 2)),
        0x46 => Some((Opcode::Lsr, AddressMode::ZeroPage, 5)),
        0x56 => Some((Opcode::Lsr, AddressMode::ZeroPageX, 6)),
        0x4E => Some((Opcode::Lsr, AddressMode::Absolute, 6)),
        0x5E => Some((Opcode::Lsr, AddressMode::AbsoluteX, 7)),

        // NOP
        0xEA => Some((Opcode::Nop, AddressMode::Implicit, 2)),

        // ORA
        0x09 => Some((Opcode::Ora, AddressMode::Immediate, 2)),
        0x05 => Some((Opcode::Ora, AddressMode::ZeroPage, 3)),
        0x15 => Some((Opcode::Ora, AddressMode::ZeroPageX, 4)),
        0x0D => Some((Opcode::Ora, AddressMode::Absolute, 4)),
        0x1D => Some((Opcode::Ora, AddressMode::AbsoluteX, 4)),
        0x19 => Some((Opcode::Ora, AddressMode::AbsoluteY, 4)),
        0x01 => Some((Opcode::Ora, AddressMode::IndirectX, 6)),
        0x11 => Some((Opcode::Ora, AddressMode::IndirectY, 5)),

        // PHA
        0x48 => Some((Opcode::Pha, AddressMode::Implicit, 3)),

        // PHP
        0x08 => Some((Opcode::Php, AddressMode::Implicit, 3)),

        // PLA
        0x68 => Some((Opcode::Pla, AddressMode::Implicit, 4)),

        // PLP
        0x28 => Some((Opcode::Plp, AddressMode::Implicit, 4)),

        // ROL
        0x2A => Some((Opcode::Rol, AddressMode::Accumulator, 2)),
        0x26 => Some((Opcode::Rol, AddressMode::ZeroPage, 5)),
        0x36 => Some((Opcode::Rol, AddressMode::ZeroPageX, 6)),
        0x2E => Some((Opcode::Rol, AddressMode::Absolute, 6)),
        0x3E => Some((Opcode::Rol, AddressMode::AbsoluteX, 7)),

        // ROR
        0x6A => Some((Opcode::Ror, AddressMode::Accumulator, 2)),
        0x66 => Some((Opcode::Ror, AddressMode::ZeroPage, 5)),
        0x76 => Some((Opcode::Ror, AddressMode::ZeroPageX, 6)),
        0x6E => Some((Opcode::Ror, AddressMode::Absolute, 6)),
        0x7E => Some((Opcode::Ror, AddressMode::AbsoluteX, 7)),

        // RTI
        0x40 => Some((Opcode::Rti, AddressMode::Implicit, 6)),

        // RTS
        0x60 => Some((Opcode::Rts, AddressMode::Implicit, 6)),

        // SBC
        0xE9 => Some((Opcode::Sbc, AddressMode::Immediate, 2)),
        0xE5 => Some((Opcode::Sbc, AddressMode::ZeroPage, 3)),
        0xF5 => Some((Opcode::Sbc, AddressMode::ZeroPageX, 4)),
        0xED => Some((Opcode::Sbc, AddressMode::Absolute, 4)),
        0xFD => Some((Opcode::Sbc, AddressMode::AbsoluteX, 4)),
        0xF9 => Some((Opcode::Sbc, AddressMode::AbsoluteY, 4)),
        0xE1 => Some((Opcode::Sbc, AddressMode::IndirectX, 6)),
        0xF1 => Some((Opcode::Sbc, AddressMode::IndirectY, 5)),

        // SEC
        0x38 => Some((Opcode::Sec, AddressMode::Implicit, 2)),

        // SED
        0xF8 => Some((Opcode::Sed, AddressMode::Implicit, 2)),

        // SEI
        0x78 => Some((Opcode::Sei, AddressMode::Implicit, 2)),

        // STA
        0x85 => Some((Opcode::Sta, AddressMode::ZeroPage, 3)),
        0x95 => Some((Opcode::Sta, AddressMode::ZeroPageX, 4)),
        0x8D => Some((Opcode::Sta, AddressMode::Absolute, 4)),
        0x9D => Some((Opcode::Sta, AddressMode::AbsoluteX, 5)),
        0x99 => Some((Opcode::Sta, AddressMode::AbsoluteY, 5)),
        0x81 => Some((Opcode::Sta, AddressMode::IndirectX, 6)),
        0x91 => Some((Opcode::Sta, AddressMode::IndirectY, 6)),

        // STX
        0x86 => Some((Opcode::Stx, AddressMode::ZeroPage, 3)),
        0x96 => Some((Opcode::Stx, AddressMode::ZeroPageY, 4)),
        0x8E => Some((Opcode::Stx, AddressMode::Absolute, 4)),

        // STY
        0x84 => Some((Opcode::Sty, AddressMode::ZeroPage, 3)),
        0x94 => Some((Opcode::Sty, AddressMode::ZeroPageX, 4)),
        0x8C => Some((Opcode::Sty, AddressMode::Absolute, 4)),

        // TAX
        0xAA => Some((Opcode::Tax, AddressMode::Implicit, 2)),

        // TAY
        0xA8 => Some((Opcode::Tay, AddressMode::Implicit, 2)),

        // TSX
        0xBA => Some((Opcode::Tsx, AddressMode::Implicit, 2)),

        // TXA
        0x8A => Some((Opcode::Txa, AddressMode::Implicit, 2)),

        // TXS
        0x9A => Some((Opcode::Txs, AddressMode::Implicit, 2)),

        // TYA
        0x98 => Some((Opcode::Tya, AddressMode::Implicit, 2)),

        _ => None,
    }
}

// Consumes bytes from the instruction "segment" to calculate an operand value,
// based on the provided addressing mode. It returns the operand, plus any CPU
// cycle adjustment required to process indexed memory reads that cross page
// boundaries. See page_crossing_cycle_adjustment() for more.
fn decode_operand(cpu: &mut Cpu, opcode: Opcode, addr_mode: AddressMode) -> (Operand, u64) {
    match addr_mode {
        AddressMode::Implicit => (Operand::None, 0),
        AddressMode::Accumulator => (Operand::Accumulator, 0),
        AddressMode::Immediate => (Operand::Immediate(cpu.consume_instruction_byte()), 0),
        AddressMode::ZeroPage => (Operand::Memory(cpu.consume_instruction_byte() as u16), 0),
        AddressMode::ZeroPageX => (
            Operand::Memory(cpu.regs.x.wrapping_add(cpu.consume_instruction_byte()) as u16),
            0,
        ),
        AddressMode::ZeroPageY => (
            Operand::Memory(cpu.regs.y.wrapping_add(cpu.consume_instruction_byte()) as u16),
            0,
        ),
        AddressMode::Relative => (Operand::Immediate(cpu.consume_instruction_byte()), 0),
        AddressMode::Absolute => (
            Operand::Memory(math::bytes_to_u16_le([
                cpu.consume_instruction_byte(),
                cpu.consume_instruction_byte(),
            ])),
            0,
        ),
        AddressMode::AbsoluteX => {
            let base = math::bytes_to_u16_le([
                cpu.consume_instruction_byte(),
                cpu.consume_instruction_byte(),
            ]);
            let addr = base + cpu.regs.x as u16;
            (
                Operand::Memory(addr),
                page_crossing_cycle_adjusment(opcode, base, addr),
            )
        }
        AddressMode::AbsoluteY => {
            let base = math::bytes_to_u16_le([
                cpu.consume_instruction_byte(),
                cpu.consume_instruction_byte(),
            ]);
            let addr = base + cpu.regs.y as u16;
            (
                Operand::Memory(addr),
                page_crossing_cycle_adjusment(opcode, base, addr),
            )
        }
        AddressMode::Indirect => {
            let bytes = [
                cpu.consume_instruction_byte(),
                cpu.consume_instruction_byte(),
            ];
            (
                Operand::Memory(cpu.mem_read16(math::bytes_to_u16_le(bytes))),
                0,
            )
        }
        AddressMode::IndirectX => {
            let offset = cpu.consume_instruction_byte();
            (
                Operand::Memory(cpu.mem_read16(cpu.regs.x.wrapping_add(offset) as u16)),
                0,
            )
        }
        AddressMode::IndirectY => {
            let indirect = cpu.consume_instruction_byte() as u16;
            let base = cpu.mem_read16(indirect);
            let addr = base + cpu.regs.y as u16;
            (
                Operand::Memory(addr),
                page_crossing_cycle_adjusment(opcode, base, addr),
            )
        }
    }
}

// Calculates a cycle cost adjustment to the base cycle cost listed in the
// opcode table in decode_raw_page(). This adjustment should only be used for
// indexed address modes that add an 8-bit offset to a 16-bit base address.
//
// Since the adder unit is only 8-bit, calculating the offset address could
// require two cycles to process in cases where there is a carry from the first
// adder cycle. As an optimization, the CPU will execute a speculative fetch
// using the result of the first pass through the adder. If there is no carry,
// then the speculative fetch was correct, and no second add is required. The
// base cycle cost assumes this optimized scenario. If there is a carry,
// however, the cycle cost needs to be increased by 1.
//
// A carry after adding an 8-bit value to a 16-bit value means that the high
// byte of the result will differ from the high byte of the base by exactly 1,
// e.g. 0xA0FF and 0xA100. As such, this scenario is often referred to as a
// "page crossing" between the base address and the offset address.
//
// Note that the address mode alone is not enough to determine whether the base
// cycle count needs to be adjusted. In particular, the adjustment only occurs
// for opcodes that read from memory, but do not write to it. Speculative
// reads are fine, but speculative writes are not. Thus, opcodes that write to
// memory, or read from and write to the same address, must incur the full cost
// of a two-cycle add. The base cycle cost for those opcodes accounts for this.
fn page_crossing_cycle_adjusment(opcode: Opcode, before: u16, after: u16) -> u64 {
    if !opcode.writes_memory() && ((before & 0xFF00) != (after & 0xFF00)) {
        1
    } else {
        0
    }
}

#[test]
fn test_decode_implicit() {
    let mut cpu = Cpu::new();
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Brk, AddressMode::Implicit),
        (Operand::None, 0)
    );
    assert_eq!(cpu.regs.pc, 0);
}

#[test]
fn test_decode_accumulator() {
    let mut cpu = Cpu::new();
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Asl, AddressMode::Accumulator),
        (Operand::Accumulator, 0)
    );
    assert_eq!(cpu.regs.pc, 0);
}

#[test]
fn test_decode_immediate() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Adc, AddressMode::Immediate),
        (Operand::Immediate(0xAB), 0)
    );
    assert_eq!(cpu.regs.pc, 1);
}

#[test]
fn test_decode_zero_page() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0x1F);
    cpu.mem_write(0x1F, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Adc, AddressMode::ZeroPage),
        (Operand::Memory(0x1F), 0)
    );
    assert_eq!(cpu.regs.pc, 1);
}

#[test]
fn test_decode_zero_page_x() {
    let mut cpu = Cpu::new();
    cpu.regs.x = 1;
    cpu.mem_write(0, 0x10);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Adc, AddressMode::ZeroPageX),
        (Operand::Memory(0x11), 0)
    );
    assert_eq!(cpu.regs.pc, 1);

    // zero-page wrapping
    let mut cpu = Cpu::new();
    cpu.regs.x = 2;
    cpu.mem_write(0, 0xFF);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Adc, AddressMode::ZeroPageX),
        (Operand::Memory(0x01), 0)
    );
    assert_eq!(cpu.regs.pc, 1);
}

#[test]
fn test_decode_zero_page_y() {
    let mut cpu = Cpu::new();
    cpu.regs.y = 1;
    cpu.mem_write(0, 0x10);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Adc, AddressMode::ZeroPageY),
        (Operand::Memory(0x11), 0)
    );
    assert_eq!(cpu.regs.pc, 1);

    // zero-page wrapping
    let mut cpu = Cpu::new();
    cpu.regs.y = 2;
    cpu.mem_write(0, 0xFF);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Adc, AddressMode::ZeroPageY),
        (Operand::Memory(0x01), 0)
    );
    assert_eq!(cpu.regs.pc, 1);
}

#[test]
fn test_decode_relative() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Beq, AddressMode::Relative),
        (Operand::Immediate(0xAB), 0)
    );
    assert_eq!(cpu.regs.pc, 1);
}

#[test]
fn test_decode_absolute() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Jsr, AddressMode::Absolute),
        (Operand::Memory(0xABCD), 0)
    );
    assert_eq!(cpu.regs.pc, 2);
}

#[test]
fn test_decode_absolute_x() {
    // Read-only op, no page crossing
    let mut cpu = Cpu::new();
    cpu.regs.x = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Lda, AddressMode::AbsoluteX),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Read-only op, page crossing
    let mut cpu = Cpu::new();
    cpu.regs.x = 0x1;
    cpu.mem_write(0, 0xFF);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Lda, AddressMode::AbsoluteX),
        (Operand::Memory(0xAC00), 1)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Write-only op
    let mut cpu = Cpu::new();
    cpu.regs.x = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Sta, AddressMode::AbsoluteX),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Read/write op
    let mut cpu = Cpu::new();
    cpu.regs.x = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Dec, AddressMode::AbsoluteX),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);
}

#[test]
fn test_decode_absolute_y() {
    // Read-only op, no page crossing
    let mut cpu = Cpu::new();
    cpu.regs.y = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Lda, AddressMode::AbsoluteY),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Read-only op, page crossing
    let mut cpu = Cpu::new();
    cpu.regs.y = 0x1;
    cpu.mem_write(0, 0xFF);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Lda, AddressMode::AbsoluteY),
        (Operand::Memory(0xAC00), 1)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Write-only op
    let mut cpu = Cpu::new();
    cpu.regs.y = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Sta, AddressMode::AbsoluteY),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Read/write op
    let mut cpu = Cpu::new();
    cpu.regs.y = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Dec, AddressMode::AbsoluteY),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);
}

#[test]
fn test_decode_indirect() {
    let mut cpu = Cpu::new();
    cpu.regs.x = 1;
    cpu.mem_write(0, 0xFF);
    cpu.mem_write(1, 1);
    cpu.mem_write(0x1FF, 0xCD);
    cpu.mem_write(0x200, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Jmp, AddressMode::Indirect),
        (Operand::Memory(0xABCD), 0)
    );
    assert_eq!(cpu.regs.pc, 2);
}

#[test]
fn test_decode_indirect_x() {
    let mut cpu = Cpu::new();
    cpu.regs.x = 1;
    cpu.mem_write(0, 0xF);
    cpu.mem_write(0x10, 0xCD);
    cpu.mem_write(0x11, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Adc, AddressMode::IndirectX),
        (Operand::Memory(0xABCD), 0)
    );
    assert_eq!(cpu.regs.pc, 1);

    // zero-page wrapping
    let mut cpu = Cpu::new();
    cpu.regs.x = 2;
    cpu.mem_write(0, 0xFF);
    cpu.mem_write(1, 0xCD);
    cpu.mem_write(2, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Adc, AddressMode::IndirectX),
        (Operand::Memory(0xABCD), 0)
    );
    assert_eq!(cpu.regs.pc, 1);
}

#[test]
fn test_decode_indirect_y() {
    // Read-only op, no page crossing
    let mut cpu = Cpu::new();
    cpu.regs.y = 1;
    cpu.mem_write(0, 0xF);
    cpu.mem_write(0xF, 0xCD);
    cpu.mem_write(0x10, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Lda, AddressMode::IndirectY),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 1);

    // Read-only op, page crossing
    let mut cpu = Cpu::new();
    cpu.regs.y = 1;
    cpu.mem_write(0, 0xF);
    cpu.mem_write(0xF, 0xFF);
    cpu.mem_write(0x10, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Lda, AddressMode::IndirectY),
        (Operand::Memory(0xAC00), 1)
    );
    assert_eq!(cpu.regs.pc, 1);

    // Write-only op
    let mut cpu = Cpu::new();
    cpu.regs.y = 1;
    cpu.mem_write(0, 0xF);
    cpu.mem_write(0xF, 0xCD);
    cpu.mem_write(0x10, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Sta, AddressMode::IndirectY),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 1);

    // Read/write op
    let mut cpu = Cpu::new();
    cpu.regs.y = 1;
    cpu.mem_write(0, 0xF);
    cpu.mem_write(0xF, 0xCD);
    cpu.mem_write(0x10, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, Opcode::Dec, AddressMode::IndirectY),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 1);
}
