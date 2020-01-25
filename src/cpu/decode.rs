use super::address_mode::AddressMode;
use super::opcode;
use super::operand::Operand;
use super::state::Cpu;
use crate::math;

/// Decodes the instruction at the PC and returns a tuple containing the opcode,
/// operand, and base cycle cost. The PC will be incremented to the start of the
/// next instruction.
pub fn decode_instruction(cpu: &mut Cpu) -> Option<(opcode::Type, Operand, u64)> {
    let (opcode_type, addr_mode, cycles) = decode_opcode(cpu.consume_instruction_byte())?;
    let (operand, cycle_adjust) = decode_operand(cpu, opcode_type, addr_mode);
    Some((opcode_type, operand, cycles + cycle_adjust))
}

/// Takes a encoded opcode and converts it to a tuple containing the opcode,
/// addressing mode, and base cycle cost.
///
/// Reference: obelisk.me.uk/6502/reference.html
fn decode_opcode(opcode: u8) -> Option<(opcode::Type, AddressMode, u64)> {
    lazy_static! {
        static ref OPCODES_BY_ENCODING: [Option<(opcode::Type, AddressMode, u64)>; 256] = {
            let mut opcodes = [None; 256];
            for raw in opcode::OPCODES.iter() {
                opcodes[raw.encoding as usize] =
                    Some((raw.opcode_type, raw.addr_mode, raw.base_cycle_cost));
            }
            opcodes
        };
    }

    OPCODES_BY_ENCODING[opcode as usize]
}

// Consumes bytes from the instruction "segment" to calculate an operand value,
// based on the provided addressing mode. It returns the operand, plus any CPU
// cycle adjustment required to process indexed memory reads that cross page
// boundaries. See page_crossing_cycle_adjustment() for more.
fn decode_operand(
    cpu: &mut Cpu,
    opcode_type: opcode::Type,
    addr_mode: AddressMode,
) -> (Operand, u64) {
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
                page_crossing_cycle_adjusment(opcode_type, base, addr),
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
                page_crossing_cycle_adjusment(opcode_type, base, addr),
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
                page_crossing_cycle_adjusment(opcode_type, base, addr),
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
fn page_crossing_cycle_adjusment(opcode_type: opcode::Type, before: u16, after: u16) -> u64 {
    if !opcode_type.writes_memory() && math::page_crossing(before, after) {
        1
    } else {
        0
    }
}

#[test]
fn test_decode_implicit() {
    let mut cpu = Cpu::new();
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Brk, AddressMode::Implicit),
        (Operand::None, 0)
    );
    assert_eq!(cpu.regs.pc, 0);
}

#[test]
fn test_decode_accumulator() {
    let mut cpu = Cpu::new();
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Asl, AddressMode::Accumulator),
        (Operand::Accumulator, 0)
    );
    assert_eq!(cpu.regs.pc, 0);
}

#[test]
fn test_decode_immediate() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Adc, AddressMode::Immediate),
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
        decode_operand(&mut cpu, opcode::Type::Adc, AddressMode::ZeroPage),
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
        decode_operand(&mut cpu, opcode::Type::Adc, AddressMode::ZeroPageX),
        (Operand::Memory(0x11), 0)
    );
    assert_eq!(cpu.regs.pc, 1);

    // zero-page wrapping
    let mut cpu = Cpu::new();
    cpu.regs.x = 2;
    cpu.mem_write(0, 0xFF);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Adc, AddressMode::ZeroPageX),
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
        decode_operand(&mut cpu, opcode::Type::Adc, AddressMode::ZeroPageY),
        (Operand::Memory(0x11), 0)
    );
    assert_eq!(cpu.regs.pc, 1);

    // zero-page wrapping
    let mut cpu = Cpu::new();
    cpu.regs.y = 2;
    cpu.mem_write(0, 0xFF);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Adc, AddressMode::ZeroPageY),
        (Operand::Memory(0x01), 0)
    );
    assert_eq!(cpu.regs.pc, 1);
}

#[test]
fn test_decode_relative() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Beq, AddressMode::Relative),
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
        decode_operand(&mut cpu, opcode::Type::Jsr, AddressMode::Absolute),
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
        decode_operand(&mut cpu, opcode::Type::Lda, AddressMode::AbsoluteX),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Read-only op, page crossing
    let mut cpu = Cpu::new();
    cpu.regs.x = 0x1;
    cpu.mem_write(0, 0xFF);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Lda, AddressMode::AbsoluteX),
        (Operand::Memory(0xAC00), 1)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Write-only op
    let mut cpu = Cpu::new();
    cpu.regs.x = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Sta, AddressMode::AbsoluteX),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Read/write op
    let mut cpu = Cpu::new();
    cpu.regs.x = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Dec, AddressMode::AbsoluteX),
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
        decode_operand(&mut cpu, opcode::Type::Lda, AddressMode::AbsoluteY),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Read-only op, page crossing
    let mut cpu = Cpu::new();
    cpu.regs.y = 0x1;
    cpu.mem_write(0, 0xFF);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Lda, AddressMode::AbsoluteY),
        (Operand::Memory(0xAC00), 1)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Write-only op
    let mut cpu = Cpu::new();
    cpu.regs.y = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Sta, AddressMode::AbsoluteY),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 2);

    // Read/write op
    let mut cpu = Cpu::new();
    cpu.regs.y = 0x1;
    cpu.mem_write(0, 0xCD);
    cpu.mem_write(1, 0xAB);
    assert_eq!(
        decode_operand(&mut cpu, opcode::Type::Dec, AddressMode::AbsoluteY),
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
        decode_operand(&mut cpu, opcode::Type::Jmp, AddressMode::Indirect),
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
        decode_operand(&mut cpu, opcode::Type::Adc, AddressMode::IndirectX),
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
        decode_operand(&mut cpu, opcode::Type::Adc, AddressMode::IndirectX),
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
        decode_operand(&mut cpu, opcode::Type::Lda, AddressMode::IndirectY),
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
        decode_operand(&mut cpu, opcode::Type::Lda, AddressMode::IndirectY),
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
        decode_operand(&mut cpu, opcode::Type::Sta, AddressMode::IndirectY),
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
        decode_operand(&mut cpu, opcode::Type::Dec, AddressMode::IndirectY),
        (Operand::Memory(0xABCE), 0)
    );
    assert_eq!(cpu.regs.pc, 1);
}
