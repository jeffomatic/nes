use super::address_mode::AddressMode;
use super::opcode;

// Reference: http://obelisk.me.uk/6502/reference.html
#[derive(Clone, Copy, Debug)]
pub enum Type {
    Adc, // Add with carry
    And, // Bitwise and
    Asl, // Arithmetic shift left
    Bcc, // Branch if carry clear
    Bcs, // Branch if carry set
    Beq, // Brance if equal
    Bit, // Bit test
    Bmi, // Branch if minus
    Bne, // Branch if not equal
    Bpl, // Branch if positive
    Brk, // Force interrupt
    Bvc, // Branch if overflow clear
    Bvs, // Branch if overflow set
    Clc, // Clear carry flag
    Cld, // Clear decimal mode
    Cli, // Clear interrupt disable
    Clv, // Clear overflow flag
    Cmp, // Compare
    Cpx, // Compare X register
    Cpy, // Compare Y register
    Dec, // Decrement memory
    Dex, // Decrement X register
    Dey, // Decrement Y register
    Eor, // Exclusive Or
    Inc, // Increment memory
    Inx, // Increment X register
    Iny, // Increment Y register
    Jmp, // Jump
    Jsr, // Jump to subroutine
    Lda, // Load accumulator
    Ldx, // Load X register
    Ldy, // Load Y register
    Lsr, // Logical shift right
    Nop, // No-op
    Ora, // Logical inclusive or
    Pha, // Push accumulator
    Php, // Push processor status
    Pla, // Pull accumulator
    Plp, // Pull processor status
    Rol, // Rotate left
    Ror, // Rotate right
    Rti, // Return from interrupt
    Rts, // Return from subroutine
    Sbc, // Subtract with carry
    Sec, // Set carry flag
    Sed, // Set decimal flag
    Sei, // Set interrupt disable
    Sta, // Store accumulator
    Stx, // Store X register
    Sty, // Store Y register
    Tax, // Transfer accumulator to X
    Tay, // Transfer accumulator to Y
    Tsx, // Transfer stack pointer to X
    Txa, // Transfer stack pointer to accumulator
    Txs, // Transfer X to stack pointer
    Tya, // Transfer Y to accumulator
}

impl Type {
    pub fn writes_memory(self) -> bool {
        match self {
            Self::Asl | Self::Dec | Self::Inc | Self::Rol | Self::Ror | Self::Sta => true,
            _ => false,
        }
    }
}

pub struct Opcode {
    pub opcode_type: opcode::Type,
    pub addr_mode: AddressMode,
    pub base_cycle_cost: u64,
    pub encoding: u8,
}

pub const OPCODES: &[Opcode] = &[
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
