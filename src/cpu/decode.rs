use super::address_mode::AddressMode;
use super::opcode::Opcode;
use super::operand::Operand;
use super::state::Cpu;
use crate::math;

/// Decodes the instruction at the PC and returns a tuple containing the opcode,
/// operand, and base cycle cost. The PC will be incremented to the start of the
/// next instruction.
pub fn decode(cpu: &mut Cpu) -> Option<(Opcode, Operand, u64)> {
    let (opcode, addr_mode, cycles) = decode_raw_opcode(cpu.consume_instruction_byte())?;
    let (operand, cycle_adjust) = decode_operand(cpu, opcode, addr_mode);
    Some((opcode, operand, cycles + cycle_adjust))
}

struct RawOpcode {
    opcode: Opcode,
    addr_mode: AddressMode,
    base_cycle_cost: u64,
    encoding: u8,
}

const RAW_OPCODES: &[RawOpcode] = &[
    RawOpcode {
        opcode: Opcode::Adc,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0x69,
    },
    RawOpcode {
        opcode: Opcode::Adc,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x65,
    },
    RawOpcode {
        opcode: Opcode::Adc,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x75,
    },
    RawOpcode {
        opcode: Opcode::Adc,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x6D,
    },
    RawOpcode {
        opcode: Opcode::Adc,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0x7D,
    },
    RawOpcode {
        opcode: Opcode::Adc,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0x79,
    },
    RawOpcode {
        opcode: Opcode::Adc,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x61,
    },
    RawOpcode {
        opcode: Opcode::Adc,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0x71,
    },
    // AND
    RawOpcode {
        opcode: Opcode::And,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0x29,
    },
    RawOpcode {
        opcode: Opcode::And,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x25,
    },
    RawOpcode {
        opcode: Opcode::And,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x35,
    },
    RawOpcode {
        opcode: Opcode::And,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x2D,
    },
    RawOpcode {
        opcode: Opcode::And,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0x3D,
    },
    RawOpcode {
        opcode: Opcode::And,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0x39,
    },
    RawOpcode {
        opcode: Opcode::And,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x21,
    },
    RawOpcode {
        opcode: Opcode::And,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0x31,
    },
    // ASL
    RawOpcode {
        opcode: Opcode::Asl,
        addr_mode: AddressMode::Accumulator,
        base_cycle_cost: 2,
        encoding: 0x0A,
    },
    RawOpcode {
        opcode: Opcode::Asl,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0x06,
    },
    RawOpcode {
        opcode: Opcode::Asl,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0x16,
    },
    RawOpcode {
        opcode: Opcode::Asl,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x0E,
    },
    RawOpcode {
        opcode: Opcode::Asl,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0x1E,
    },
    // BCC
    RawOpcode {
        opcode: Opcode::Bcc,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x90,
    },
    // BCS
    RawOpcode {
        opcode: Opcode::Bcs,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0xB0,
    },
    // BEQ
    RawOpcode {
        opcode: Opcode::Beq,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0xF0,
    },
    // BIT
    RawOpcode {
        opcode: Opcode::Bit,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x24,
    },
    RawOpcode {
        opcode: Opcode::Bit,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x2C,
    },
    // BMI
    RawOpcode {
        opcode: Opcode::Bmi,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x30,
    },
    // BNE
    RawOpcode {
        opcode: Opcode::Bne,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0xD0,
    },
    // BPL
    RawOpcode {
        opcode: Opcode::Bpl,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x10,
    },
    // BRK
    RawOpcode {
        opcode: Opcode::Brk,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 7,
        encoding: 0x00,
    },
    // BVC
    RawOpcode {
        opcode: Opcode::Bvc,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x50,
    },
    // BVS
    RawOpcode {
        opcode: Opcode::Bvs,
        addr_mode: AddressMode::Relative,
        base_cycle_cost: 2,
        encoding: 0x70,
    },
    // CLC
    RawOpcode {
        opcode: Opcode::Clc,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x18,
    },
    // CLD
    RawOpcode {
        opcode: Opcode::Cld,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xD8,
    },
    // CLI
    RawOpcode {
        opcode: Opcode::Cli,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x58,
    },
    // CLV
    RawOpcode {
        opcode: Opcode::Clv,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xB8,
    },
    // CMP
    RawOpcode {
        opcode: Opcode::Cmp,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xC9,
    },
    RawOpcode {
        opcode: Opcode::Cmp,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xC5,
    },
    RawOpcode {
        opcode: Opcode::Cmp,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0xD5,
    },
    RawOpcode {
        opcode: Opcode::Cmp,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xCD,
    },
    RawOpcode {
        opcode: Opcode::Cmp,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0xDD,
    },
    RawOpcode {
        opcode: Opcode::Cmp,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0xD9,
    },
    RawOpcode {
        opcode: Opcode::Cmp,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0xC1,
    },
    RawOpcode {
        opcode: Opcode::Cmp,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0xD1,
    },
    // CPX
    RawOpcode {
        opcode: Opcode::Cpx,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xE0,
    },
    RawOpcode {
        opcode: Opcode::Cpx,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xE4,
    },
    RawOpcode {
        opcode: Opcode::Cpx,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xEC,
    },
    // CPY
    RawOpcode {
        opcode: Opcode::Cpy,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xC0,
    },
    RawOpcode {
        opcode: Opcode::Cpy,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xC4,
    },
    RawOpcode {
        opcode: Opcode::Cpy,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xCC,
    },
    // DEC
    RawOpcode {
        opcode: Opcode::Dec,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0xC6,
    },
    RawOpcode {
        opcode: Opcode::Dec,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0xD6,
    },
    RawOpcode {
        opcode: Opcode::Dec,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0xCE,
    },
    RawOpcode {
        opcode: Opcode::Dec,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0xDE,
    },
    // DEX
    RawOpcode {
        opcode: Opcode::Dex,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xCA,
    },
    // DEY
    RawOpcode {
        opcode: Opcode::Dey,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x88,
    },
    // EOR
    RawOpcode {
        opcode: Opcode::Eor,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0x49,
    },
    RawOpcode {
        opcode: Opcode::Eor,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x45,
    },
    RawOpcode {
        opcode: Opcode::Eor,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x55,
    },
    RawOpcode {
        opcode: Opcode::Eor,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x4D,
    },
    RawOpcode {
        opcode: Opcode::Eor,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0x5D,
    },
    RawOpcode {
        opcode: Opcode::Eor,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0x59,
    },
    RawOpcode {
        opcode: Opcode::Eor,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x41,
    },
    RawOpcode {
        opcode: Opcode::Eor,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0x51,
    },
    // INC
    RawOpcode {
        opcode: Opcode::Inc,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0xE6,
    },
    RawOpcode {
        opcode: Opcode::Inc,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0xF6,
    },
    RawOpcode {
        opcode: Opcode::Inc,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0xEE,
    },
    RawOpcode {
        opcode: Opcode::Inc,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0xFE,
    },
    // INX
    RawOpcode {
        opcode: Opcode::Inx,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xE8,
    },
    // INY
    RawOpcode {
        opcode: Opcode::Iny,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xC8,
    },
    // JMP
    RawOpcode {
        opcode: Opcode::Jmp,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 3,
        encoding: 0x4C,
    },
    RawOpcode {
        opcode: Opcode::Jmp,
        addr_mode: AddressMode::Indirect,
        base_cycle_cost: 5,
        encoding: 0x6C,
    },
    // JSR
    RawOpcode {
        opcode: Opcode::Jsr,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x20,
    },
    // LDA
    RawOpcode {
        opcode: Opcode::Lda,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xA9,
    },
    RawOpcode {
        opcode: Opcode::Lda,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xA5,
    },
    RawOpcode {
        opcode: Opcode::Lda,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0xB5,
    },
    RawOpcode {
        opcode: Opcode::Lda,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xAD,
    },
    RawOpcode {
        opcode: Opcode::Lda,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0xBD,
    },
    RawOpcode {
        opcode: Opcode::Lda,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0xB9,
    },
    RawOpcode {
        opcode: Opcode::Lda,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0xA1,
    },
    RawOpcode {
        opcode: Opcode::Lda,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0xB1,
    },
    // LDX
    RawOpcode {
        opcode: Opcode::Ldx,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xA2,
    },
    RawOpcode {
        opcode: Opcode::Ldx,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xA6,
    },
    RawOpcode {
        opcode: Opcode::Ldx,
        addr_mode: AddressMode::ZeroPageY,
        base_cycle_cost: 4,
        encoding: 0xB6,
    },
    RawOpcode {
        opcode: Opcode::Ldx,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xAE,
    },
    RawOpcode {
        opcode: Opcode::Ldx,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0xBE,
    },
    // LDY
    RawOpcode {
        opcode: Opcode::Ldy,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xA0,
    },
    RawOpcode {
        opcode: Opcode::Ldy,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xA4,
    },
    RawOpcode {
        opcode: Opcode::Ldy,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0xB4,
    },
    RawOpcode {
        opcode: Opcode::Ldy,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xAC,
    },
    RawOpcode {
        opcode: Opcode::Ldy,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0xBC,
    },
    // LSR
    RawOpcode {
        opcode: Opcode::Lsr,
        addr_mode: AddressMode::Accumulator,
        base_cycle_cost: 2,
        encoding: 0x4A,
    },
    RawOpcode {
        opcode: Opcode::Lsr,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0x46,
    },
    RawOpcode {
        opcode: Opcode::Lsr,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0x56,
    },
    RawOpcode {
        opcode: Opcode::Lsr,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x4E,
    },
    RawOpcode {
        opcode: Opcode::Lsr,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0x5E,
    },
    // NOP
    RawOpcode {
        opcode: Opcode::Nop,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xEA,
    },
    // ORA
    RawOpcode {
        opcode: Opcode::Ora,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0x09,
    },
    RawOpcode {
        opcode: Opcode::Ora,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x05,
    },
    RawOpcode {
        opcode: Opcode::Ora,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x15,
    },
    RawOpcode {
        opcode: Opcode::Ora,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x0D,
    },
    RawOpcode {
        opcode: Opcode::Ora,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0x1D,
    },
    RawOpcode {
        opcode: Opcode::Ora,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0x19,
    },
    RawOpcode {
        opcode: Opcode::Ora,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x01,
    },
    RawOpcode {
        opcode: Opcode::Ora,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0x11,
    },
    // PHA
    RawOpcode {
        opcode: Opcode::Pha,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 3,
        encoding: 0x48,
    },
    // PHP
    RawOpcode {
        opcode: Opcode::Php,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 3,
        encoding: 0x08,
    },
    // PLA
    RawOpcode {
        opcode: Opcode::Pla,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 4,
        encoding: 0x68,
    },
    // PLP
    RawOpcode {
        opcode: Opcode::Plp,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 4,
        encoding: 0x28,
    },
    // ROL
    RawOpcode {
        opcode: Opcode::Rol,
        addr_mode: AddressMode::Accumulator,
        base_cycle_cost: 2,
        encoding: 0x2A,
    },
    RawOpcode {
        opcode: Opcode::Rol,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0x26,
    },
    RawOpcode {
        opcode: Opcode::Rol,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0x36,
    },
    RawOpcode {
        opcode: Opcode::Rol,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x2E,
    },
    RawOpcode {
        opcode: Opcode::Rol,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0x3E,
    },
    // ROR
    RawOpcode {
        opcode: Opcode::Ror,
        addr_mode: AddressMode::Accumulator,
        base_cycle_cost: 2,
        encoding: 0x6A,
    },
    RawOpcode {
        opcode: Opcode::Ror,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 5,
        encoding: 0x66,
    },
    RawOpcode {
        opcode: Opcode::Ror,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 6,
        encoding: 0x76,
    },
    RawOpcode {
        opcode: Opcode::Ror,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 6,
        encoding: 0x6E,
    },
    RawOpcode {
        opcode: Opcode::Ror,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 7,
        encoding: 0x7E,
    },
    // RTI
    RawOpcode {
        opcode: Opcode::Rti,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 6,
        encoding: 0x40,
    },
    // RTS
    RawOpcode {
        opcode: Opcode::Rts,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 6,
        encoding: 0x60,
    },
    // SBC
    RawOpcode {
        opcode: Opcode::Sbc,
        addr_mode: AddressMode::Immediate,
        base_cycle_cost: 2,
        encoding: 0xE9,
    },
    RawOpcode {
        opcode: Opcode::Sbc,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0xE5,
    },
    RawOpcode {
        opcode: Opcode::Sbc,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0xF5,
    },
    RawOpcode {
        opcode: Opcode::Sbc,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0xED,
    },
    RawOpcode {
        opcode: Opcode::Sbc,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 4,
        encoding: 0xFD,
    },
    RawOpcode {
        opcode: Opcode::Sbc,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 4,
        encoding: 0xF9,
    },
    RawOpcode {
        opcode: Opcode::Sbc,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0xE1,
    },
    RawOpcode {
        opcode: Opcode::Sbc,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 5,
        encoding: 0xF1,
    },
    // SEC
    RawOpcode {
        opcode: Opcode::Sec,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x38,
    },
    // SED
    RawOpcode {
        opcode: Opcode::Sed,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xF8,
    },
    // SEI
    RawOpcode {
        opcode: Opcode::Sei,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x78,
    },
    // STA
    RawOpcode {
        opcode: Opcode::Sta,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x85,
    },
    RawOpcode {
        opcode: Opcode::Sta,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x95,
    },
    RawOpcode {
        opcode: Opcode::Sta,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x8D,
    },
    RawOpcode {
        opcode: Opcode::Sta,
        addr_mode: AddressMode::AbsoluteX,
        base_cycle_cost: 5,
        encoding: 0x9D,
    },
    RawOpcode {
        opcode: Opcode::Sta,
        addr_mode: AddressMode::AbsoluteY,
        base_cycle_cost: 5,
        encoding: 0x99,
    },
    RawOpcode {
        opcode: Opcode::Sta,
        addr_mode: AddressMode::IndirectX,
        base_cycle_cost: 6,
        encoding: 0x81,
    },
    RawOpcode {
        opcode: Opcode::Sta,
        addr_mode: AddressMode::IndirectY,
        base_cycle_cost: 6,
        encoding: 0x91,
    },
    // STX
    RawOpcode {
        opcode: Opcode::Stx,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x86,
    },
    RawOpcode {
        opcode: Opcode::Stx,
        addr_mode: AddressMode::ZeroPageY,
        base_cycle_cost: 4,
        encoding: 0x96,
    },
    RawOpcode {
        opcode: Opcode::Stx,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x8E,
    },
    // STY
    RawOpcode {
        opcode: Opcode::Sty,
        addr_mode: AddressMode::ZeroPage,
        base_cycle_cost: 3,
        encoding: 0x84,
    },
    RawOpcode {
        opcode: Opcode::Sty,
        addr_mode: AddressMode::ZeroPageX,
        base_cycle_cost: 4,
        encoding: 0x94,
    },
    RawOpcode {
        opcode: Opcode::Sty,
        addr_mode: AddressMode::Absolute,
        base_cycle_cost: 4,
        encoding: 0x8C,
    },
    // TAX
    RawOpcode {
        opcode: Opcode::Tax,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xAA,
    },
    // TAY
    RawOpcode {
        opcode: Opcode::Tay,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xA8,
    },
    // TSX
    RawOpcode {
        opcode: Opcode::Tsx,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0xBA,
    },
    // TXA
    RawOpcode {
        opcode: Opcode::Txa,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x8A,
    },
    // TXS
    RawOpcode {
        opcode: Opcode::Txs,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x9A,
    },
    // TYA
    RawOpcode {
        opcode: Opcode::Tya,
        addr_mode: AddressMode::Implicit,
        base_cycle_cost: 2,
        encoding: 0x98,
    },
];

/// Takes a encoded opcode and converts it to a tuple containing the opcode,
/// addressing mode, and base cycle cost.
///
/// Reference: obelisk.me.uk/6502/reference.html
fn decode_raw_opcode(raw_opcode: u8) -> Option<(Opcode, AddressMode, u64)> {
    lazy_static! {
        static ref OPCODES_BY_ENCODING: [Option<(Opcode, AddressMode, u64)>; 256] = {
            let mut opcodes = [None; 256];
            for raw in RAW_OPCODES.iter() {
                opcodes[raw.encoding as usize] =
                    Some((raw.opcode, raw.addr_mode, raw.base_cycle_cost));
            }
            opcodes
        };
    }

    OPCODES_BY_ENCODING[raw_opcode as usize]
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
    if !opcode.writes_memory() && math::page_crossing(before, after) {
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
