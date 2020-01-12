enum StatusMask {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode,
    BreakCommand,
    Overflow,
    Negative,
}

impl StatusMask {
    fn bit(&self) -> u8 {
        match *self {
            Self::Carry => 0,
            Self::Zero => 1,
            Self::InterruptDisable => 2,
            Self::DecimalMode => 3,
            Self::BreakCommand => 4,
            Self::Overflow => 5,
            Self::Negative => 6,
        }
    }

    fn mask(&self) -> u8 {
        1 << self.bit()
    }
}

// Reference: http://obelisk.me.uk/6502/registers.html
#[derive(Debug)]
pub struct Registers {
    // Accumulator
    pub a: u8,

    // Index X
    // Used for counters and memory offsets for particular instructions.
    // Unlike the Y register, it can be used to copy or manipulate the stack
    // pointer.
    pub x: u8,

    // Index Y
    pub y: u8,

    // Program counter
    pub pc: u16,

    // Stack pointer
    // Stack is 256 bytes, between 0x100 and 0x1FF. The pointer is the low
    // 8 bits.
    pub s: u8,

    // Processor status
    // 0 - Carry flag: set if the last op resulted in overflow in the high bit,
    //     or underflow in the low bit.
    // 1 - Zero flag: set if the last op resulted in zero.
    // 2 - Interrupt disable: set if interrupts have been disabled by SEI, and
    //     and not yet cleared by CLI.
    // 3 - Decimal mode: when set, arithmetic operations will obey BCD.
    //     Set by SED and cleared by CLD.
    // 4 - Break command
    // 5 - Overflow flag:
    // 6 - Negative flag: set if the last op resulted in a high bit of 1.
    pub p: u8,
}

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub ram: Vec<u8>, // should be 2k u8 array, but we're hacking around derive(Debug).
}

// Reference: http://obelisk.me.uk/6502/addressing.html
#[derive(Debug)]
pub enum AddressMode {
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
    IndirectX,
    IndirectY,
}

// Reference: http://obelisk.me.uk/6502/reference.html
#[derive(Debug)]
pub enum Instruction {
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

impl Cpu {
    pub fn get_operand_8(&self) -> u8 {
        self.mem_read(self.registers.pc + 1)
    }

    pub fn get_operand_16(&self) -> u16 {
        ((self.mem_read(self.registers.pc + 1) as u16) << 8)
            + self.mem_read(self.registers.pc + 2) as u16
    }

    pub fn get_operand_value_8(&self, addr_mode: AddressMode) -> u8 {
        match addr_mode {
            AddressMode::Accumulator => self.registers.a,
            AddressMode::Immediate => self.get_operand_8(),
            AddressMode::ZeroPage => self.mem_read(self.get_operand_8() as u16),
            AddressMode::ZeroPageX => {
                self.mem_read(self.registers.x as u16 + self.get_operand_8() as u16)
            }
            AddressMode::ZeroPageY => {
                self.mem_read(self.registers.y as u16 + self.get_operand_8() as u16)
            }
            AddressMode::AbsoluteX => {
                self.mem_read(self.registers.x as u16 + self.get_operand_16())
            }
            AddressMode::AbsoluteY => {
                self.mem_read(self.registers.y as u16 + self.get_operand_16())
            }
            other => panic!("addressing mode {:?} has no 8-bit operand", other),
        }
    }

    pub fn get_operand_value_16(&self, addr_mode: AddressMode) -> u16 {
        match addr_mode {
            AddressMode::Relative => {
                ((self.registers.pc as i32) + (self.get_operand_8() as i8 as i32)) as u16
            }
            AddressMode::Absolute => self.get_operand_16(),
            other => panic!("addressing mode {:?} has no 16-bit operand", other),
        }
    }

    // https://wiki.nesdev.com/w/index.php/CPU_memory_map
    pub fn mem_read(&self, addr: u16) -> u8 {
        // TODO: only internal RAM access is implemented, mappers are TBD.
        self.ram[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }
}
