use super::address_mode::AddressMode;
use super::opcode;
use super::operand::Operand;
use super::state::Cpu;
use crate::math;

/// Decodes the instruction at the PC and returns a tuple containing the opcode,
/// operand, and base cycle cost. The PC will be incremented to the start of the
/// next instruction.
pub fn decode(cpu: &mut Cpu) -> Option<(opcode::Type, Operand, u64)> {
    let (opcode_type, addr_mode, cycles) = decode_raw_opcode(cpu.consume_instruction_byte())?;
    let (operand, cycle_adjust) = decode_operand(cpu, opcode_type, addr_mode);
    Some((opcode_type, operand, cycles + cycle_adjust))
}

struct Opcode {
    opcode_type: opcode::Type,
    addr_mode: AddressMode,
    base_cycle_cost: u64,
    encoding: u8,
}

const RAW_OPCODES: &[Opcode] = &[
    // ADC
    Opcode {
        opcode_type: opcode::Type::Adc,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0x69,
    },
    Opcode {
        opcode_type: opcode::Type::Adc,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x65,
    },
    Opcode {
        opcode_type: opcode::Type::Adc,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x75,
    },
    Opcode {
        opcode_type: opcode::Type::Adc,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x6D,
    },
    Opcode {
        opcode_type: opcode::Type::Adc,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0x7D,
    },
    Opcode {
        opcode_type: opcode::Type::Adc,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0x79,
    },
    Opcode {
        opcode_type: opcode::Type::Adc,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x61,
    },
    Opcode {
        opcode_type: opcode::Type::Adc,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0x71,
    },
    // AND
    Opcode {
        opcode_type: opcode::Type::And,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0x29,
    },
    Opcode {
        opcode_type: opcode::Type::And,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x25,
    },
    Opcode {
        opcode_type: opcode::Type::And,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x35,
    },
    Opcode {
        opcode_type: opcode::Type::And,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x2D,
    },
    Opcode {
        opcode_type: opcode::Type::And,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0x3D,
    },
    Opcode {
        opcode_type: opcode::Type::And,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0x39,
    },
    Opcode {
        opcode_type: opcode::Type::And,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x21,
    },
    Opcode {
        opcode_type: opcode::Type::And,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0x31,
    },
    // ASL
    Opcode {
        opcode_type: opcode::Type::Asl,
        addr_mode: AddressMode::Accumulator,
        base_cycle_cost: 2,
        encoding: 0x0A,
    },
    Opcode {
        opcode_type: opcode::Type::Asl,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0x06,
    },
    Opcode {
        opcode_type: opcode::Type::Asl,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0x16,
    },
    Opcode {
        opcode_type: opcode::Type::Asl,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x0E,
    },
    Opcode {
        opcode_type: opcode::Type::Asl,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0x1E,
    },
    // BCC
    Opcode {
        opcode_type: opcode::Type::Bcc,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x90,
    },
    // BCS
    Opcode {
        opcode_type: opcode::Type::Bcs,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0xB0,
    },
    // BEQ
    Opcode {
        opcode_type: opcode::Type::Beq,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0xF0,
    },
    // BIT
    Opcode {
        opcode_type: opcode::Type::Bit,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x24,
    },
    Opcode {
        opcode_type: opcode::Type::Bit,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x2C,
    },
    // BMI
    Opcode {
        opcode_type: opcode::Type::Bmi,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x30,
    },
    // BNE
    Opcode {
        opcode_type: opcode::Type::Bne,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0xD0,
    },
    // BPL
    Opcode {
        opcode_type: opcode::Type::Bpl,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x10,
    },
    // BRK
    Opcode {
        opcode_type: opcode::Type::Brk,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 7,
        encoding: 0x00,
    },
    // BVC
    Opcode {
        opcode_type: opcode::Type::Bvc,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x50,
    },
    // BVS
    Opcode {
        opcode_type: opcode::Type::Bvs,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x70,
    },
    // CLC
    Opcode {
        opcode_type: opcode::Type::Clc,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x18,
    },
    // CLD
    Opcode {
        opcode_type: opcode::Type::Cld,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xD8,
    },
    // CLI
    Opcode {
        opcode_type: opcode::Type::Cli,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x58,
    },
    // CLV
    Opcode {
        opcode_type: opcode::Type::Clv,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xB8,
    },
    // CMP
    Opcode {
        opcode_type: opcode::Type::Cmp,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xC9,
    },
    Opcode {
        opcode_type: opcode::Type::Cmp,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xC5,
    },
    Opcode {
        opcode_type: opcode::Type::Cmp,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0xD5,
    },
    Opcode {
        opcode_type: opcode::Type::Cmp,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xCD,
    },
    Opcode {
        opcode_type: opcode::Type::Cmp,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0xDD,
    },
    Opcode {
        opcode_type: opcode::Type::Cmp,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0xD9,
    },
    Opcode {
        opcode_type: opcode::Type::Cmp,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0xC1,
    },
    Opcode {
        opcode_type: opcode::Type::Cmp,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0xD1,
    },
    // CPX
    Opcode {
        opcode_type: opcode::Type::Cpx,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xE0,
    },
    Opcode {
        opcode_type: opcode::Type::Cpx,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xE4,
    },
    Opcode {
        opcode_type: opcode::Type::Cpx,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xEC,
    },
    // CPY
    Opcode {
        opcode_type: opcode::Type::Cpy,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xC0,
    },
    Opcode {
        opcode_type: opcode::Type::Cpy,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xC4,
    },
    Opcode {
        opcode_type: opcode::Type::Cpy,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xCC,
    },
    // DEC
    Opcode {
        opcode_type: opcode::Type::Dec,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0xC6,
    },
    Opcode {
        opcode_type: opcode::Type::Dec,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0xD6,
    },
    Opcode {
        opcode_type: opcode::Type::Dec,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0xCE,
    },
    Opcode {
        opcode_type: opcode::Type::Dec,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0xDE,
    },
    // DEX
    Opcode {
        opcode_type: opcode::Type::Dex,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xCA,
    },
    // DEY
    Opcode {
        opcode_type: opcode::Type::Dey,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x88,
    },
    // EOR
    Opcode {
        opcode_type: opcode::Type::Eor,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0x49,
    },
    Opcode {
        opcode_type: opcode::Type::Eor,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x45,
    },
    Opcode {
        opcode_type: opcode::Type::Eor,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x55,
    },
    Opcode {
        opcode_type: opcode::Type::Eor,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x4D,
    },
    Opcode {
        opcode_type: opcode::Type::Eor,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0x5D,
    },
    Opcode {
        opcode_type: opcode::Type::Eor,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0x59,
    },
    Opcode {
        opcode_type: opcode::Type::Eor,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x41,
    },
    Opcode {
        opcode_type: opcode::Type::Eor,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0x51,
    },
    // INC
    Opcode {
        opcode_type: opcode::Type::Inc,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0xE6,
    },
    Opcode {
        opcode_type: opcode::Type::Inc,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0xF6,
    },
    Opcode {
        opcode_type: opcode::Type::Inc,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0xEE,
    },
    Opcode {
        opcode_type: opcode::Type::Inc,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0xFE,
    },
    // INX
    Opcode {
        opcode_type: opcode::Type::Inx,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xE8,
    },
    // INY
    Opcode {
        opcode_type: opcode::Type::Iny,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xC8,
    },
    // JMP
    Opcode {
        opcode_type: opcode::Type::Jmp,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 3,
        encoding: 0x4C,
    },
    Opcode {
        opcode_type: opcode::Type::Jmp,
        addr_mode: AddressMode::Indirect,
        base_cycle_cost: 5,
        encoding: 0x6C,
    },
    // JSR
    Opcode {
        opcode_type: opcode::Type::Jsr,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x20,
    },
    // LDA
    Opcode {
        opcode_type: opcode::Type::Lda,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xA9,
    },
    Opcode {
        opcode_type: opcode::Type::Lda,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xA5,
    },
    Opcode {
        opcode_type: opcode::Type::Lda,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0xB5,
    },
    Opcode {
        opcode_type: opcode::Type::Lda,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xAD,
    },
    Opcode {
        opcode_type: opcode::Type::Lda,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0xBD,
    },
    Opcode {
        opcode_type: opcode::Type::Lda,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0xB9,
    },
    Opcode {
        opcode_type: opcode::Type::Lda,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0xA1,
    },
    Opcode {
        opcode_type: opcode::Type::Lda,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0xB1,
    },
    // LDX
    Opcode {
        opcode_type: opcode::Type::Ldx,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xA2,
    },
    Opcode {
        opcode_type: opcode::Type::Ldx,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xA6,
    },
    Opcode {
        opcode_type: opcode::Type::Ldx,
        addr_mode: AddressMode::ZeroPageY,
        base_cycle_cost: 4,
        encoding: 0xB6,
    },
    Opcode {
        opcode_type: opcode::Type::Ldx,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xAE,
    },
    Opcode {
        opcode_type: opcode::Type::Ldx,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0xBE,
    },
    // LDY
    Opcode {
        opcode_type: opcode::Type::Ldy,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xA0,
    },
    Opcode {
        opcode_type: opcode::Type::Ldy,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xA4,
    },
    Opcode {
        opcode_type: opcode::Type::Ldy,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0xB4,
    },
    Opcode {
        opcode_type: opcode::Type::Ldy,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xAC,
    },
    Opcode {
        opcode_type: opcode::Type::Ldy,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0xBC,
    },
    // LSR
    Opcode {
        opcode_type: opcode::Type::Lsr,
        addr_mode: AddressMode::Accumulator,
        base_cycle_cost: 2,
        encoding: 0x4A,
    },
    Opcode {
        opcode_type: opcode::Type::Lsr,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0x46,
    },
    Opcode {
        opcode_type: opcode::Type::Lsr,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0x56,
    },
    Opcode {
        opcode_type: opcode::Type::Lsr,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x4E,
    },
    Opcode {
        opcode_type: opcode::Type::Lsr,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0x5E,
    },
    // NOP
    Opcode {
        opcode_type: opcode::Type::Nop,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xEA,
    },
    // ORA
    Opcode {
        opcode_type: opcode::Type::Ora,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0x09,
    },
    Opcode {
        opcode_type: opcode::Type::Ora,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x05,
    },
    Opcode {
        opcode_type: opcode::Type::Ora,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x15,
    },
    Opcode {
        opcode_type: opcode::Type::Ora,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x0D,
    },
    Opcode {
        opcode_type: opcode::Type::Ora,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0x1D,
    },
    Opcode {
        opcode_type: opcode::Type::Ora,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0x19,
    },
    Opcode {
        opcode_type: opcode::Type::Ora,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x01,
    },
    Opcode {
        opcode_type: opcode::Type::Ora,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0x11,
    },
    // PHA
    Opcode {
        opcode_type: opcode::Type::Pha,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 3,
        encoding: 0x48,
    },
    // PHP
    Opcode {
        opcode_type: opcode::Type::Php,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 3,
        encoding: 0x08,
    },
    // PLA
    Opcode {
        opcode_type: opcode::Type::Pla,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 4,
        encoding: 0x68,
    },
    // PLP
    Opcode {
        opcode_type: opcode::Type::Plp,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 4,
        encoding: 0x28,
    },
    // ROL
    Opcode {
        opcode_type: opcode::Type::Rol,
        addr_mode: AddressMode::Accumulator,
        base_cycle_cost: 2,
        encoding: 0x2A,
    },
    Opcode {
        opcode_type: opcode::Type::Rol,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0x26,
    },
    Opcode {
        opcode_type: opcode::Type::Rol,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0x36,
    },
    Opcode {
        opcode_type: opcode::Type::Rol,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x2E,
    },
    Opcode {
        opcode_type: opcode::Type::Rol,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0x3E,
    },
    // ROR
    Opcode {
        opcode_type: opcode::Type::Ror,
        addr_mode: AddressMode::Accumulator,
        base_cycle_cost: 2,
        encoding: 0x6A,
    },
    Opcode {
        opcode_type: opcode::Type::Ror,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0x66,
    },
    Opcode {
        opcode_type: opcode::Type::Ror,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0x76,
    },
    Opcode {
        opcode_type: opcode::Type::Ror,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x6E,
    },
    Opcode {
        opcode_type: opcode::Type::Ror,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0x7E,
    },
    // RTI
    Opcode {
        opcode_type: opcode::Type::Rti,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 6,
        encoding: 0x40,
    },
    // RTS
    Opcode {
        opcode_type: opcode::Type::Rts,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 6,
        encoding: 0x60,
    },
    // SBC
    Opcode {
        opcode_type: opcode::Type::Sbc,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xE9,
    },
    Opcode {
        opcode_type: opcode::Type::Sbc,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xE5,
    },
    Opcode {
        opcode_type: opcode::Type::Sbc,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0xF5,
    },
    Opcode {
        opcode_type: opcode::Type::Sbc,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xED,
    },
    Opcode {
        opcode_type: opcode::Type::Sbc,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0xFD,
    },
    Opcode {
        opcode_type: opcode::Type::Sbc,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0xF9,
    },
    Opcode {
        opcode_type: opcode::Type::Sbc,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0xE1,
    },
    Opcode {
        opcode_type: opcode::Type::Sbc,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0xF1,
    },
    // SEC
    Opcode {
        opcode_type: opcode::Type::Sec,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x38,
    },
    // SED
    Opcode {
        opcode_type: opcode::Type::Sed,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xF8,
    },
    // SEI
    Opcode {
        opcode_type: opcode::Type::Sei,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x78,
    },
    // STA
    Opcode {
        opcode_type: opcode::Type::Sta,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x85,
    },
    Opcode {
        opcode_type: opcode::Type::Sta,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x95,
    },
    Opcode {
        opcode_type: opcode::Type::Sta,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x8D,
    },
    Opcode {
        opcode_type: opcode::Type::Sta,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 5,
        encoding: 0x9D,
    },
    Opcode {
        opcode_type: opcode::Type::Sta,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 5,
        encoding: 0x99,
    },
    Opcode {
        opcode_type: opcode::Type::Sta,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x81,
    },
    Opcode {
        opcode_type: opcode::Type::Sta,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 6,
        encoding: 0x91,
    },
    // STX
    Opcode {
        opcode_type: opcode::Type::Stx,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x86,
    },
    Opcode {
        opcode_type: opcode::Type::Stx,
        addr_mode: AddressMode::ZeroPageY,
        base_cycle_cost: 4,
        encoding: 0x96,
    },
    Opcode {
        opcode_type: opcode::Type::Stx,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x8E,
    },
    // STY
    Opcode {
        opcode_type: opcode::Type::Sty,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x84,
    },
    Opcode {
        opcode_type: opcode::Type::Sty,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x94,
    },
    Opcode {
        opcode_type: opcode::Type::Sty,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x8C,
    },
    // TAX
    Opcode {
        opcode_type: opcode::Type::Tax,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xAA,
    },
    // TAY
    Opcode {
        opcode_type: opcode::Type::Tay,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xA8,
    },
    // TSX
    Opcode {
        opcode_type: opcode::Type::Tsx,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xBA,
    },
    // TXA
    Opcode {
        opcode_type: opcode::Type::Txa,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x8A,
    },
    // TXS
    Opcode {
        opcode_type: opcode::Type::Txs,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x9A,
    },
    // TYA
    Opcode {
        opcode_type: opcode::Type::Tya,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x98,
    },
];

/// Takes a encoded opcode and converts it to a tuple containing the opcode,
/// addressing mode, and base cycle cost.
///
/// Reference: obelisk.me.uk/6502/reference.html
fn decode_raw_opcode(opcode: u8) -> Option<(opcode::Type, AddressMode, u64)> {
    lazy_static! {
        static ref OPCODES_BY_ENCODING: [Option<(opcode::Type, AddressMode, u64)>; 256] = {
            let mut opcodes = [None; 256];
            for raw in RAW_OPCODES.iter() {
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
