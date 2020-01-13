use crate::math;

enum Status {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode, // No effect on the NES.
    Break1,
    Break2,
    Overflow,
    Negative,
}

impl Status {
    fn bit(&self) -> u8 {
        match *self {
            Self::Carry => 0,
            Self::Zero => 1,
            Self::InterruptDisable => 2,
            Self::DecimalMode => 3,
            Self::Break1 => 4,
            Self::Break2 => 5,
            Self::Overflow => 6,
            Self::Negative => 7,
        }
    }

    fn mask(&self) -> u8 {
        1 << self.bit()
    }

    fn set_into(&self, bitfield: u8, on: bool) -> u8 {
        if on {
            bitfield | self.mask()
        } else {
            bitfield & !self.mask()
        }
    }
}

// Reference: http://obelisk.me.uk/6502/registers.html
#[derive(Debug, Default)]
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
    // 3 - Decimal mode: no effect on the NES. For reference, this status is set
    //     by SED and cleared by CLD. When set, arithmetic operations will obey
    //     Binary Coded Decimal (BCD). A byte represents a two-digit decimal
    //     number, with the low nibble representing the low digit, and the high
    //     nibble representing the high digit.
    // 4 - Break command
    // 5 - Overflow flag: set if the last op resulted in a value greater than
    //     127. If this flag is set, the negative flag will also be set.
    // 6 - Negative flag: set if the last op resulted in a high bit of 1.
    pub p: u8,
}

// Reference: http://obelisk.me.uk/6502/addressing.html
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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

pub struct ConcreteInstruction {
    ins: Instruction,
    operand8: Option<u8>,
    operand16: Option<u16>,
}

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: Registers,
    pub ram: Vec<u8>, // should be 2k u8 array, but we're hacking around derive(Debug).
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut ram = Vec::new();
        ram.resize_with(2 * 1024, Default::default);
        Cpu {
            registers: Registers::default(),
            ram: ram,
        }
    }

    pub fn get_status(&self, s: Status) -> bool {
        self.registers.p & s.mask() > 0
    }

    pub fn set_status(&mut self, s: Status, on: bool) {
        self.registers.p = s.set_into(self.registers.p, on)
    }

    pub fn execute(&mut self, ci: &ConcreteInstruction) {
        match ci.ins {
            Instruction::Adc => {
                let prev = self.registers.a;
                let operand = ci.operand8.unwrap();
                let res = self.registers.a.wrapping_add(operand);
                let negative = (res & 0b1000_0000) != 0;

                self.registers.a = res;
                self.set_status(Status::Carry, res < prev);
                self.set_status(Status::Zero, res == 0);
                self.set_status(
                    Status::Overflow,
                    (prev as i8 >= 0) && (operand as i8 >= 0) && negative,
                );
                self.set_status(Status::Negative, negative);
            }
            Instruction::And => {
                let res = self.registers.a & ci.operand8.unwrap();

                self.registers.a = res;
                self.set_status(Status::Zero, res == 0);
                self.set_status(Status::Negative, (res & 0b1000_0000) != 0);
            }
            Instruction::Asl => {
                // TODO: memory update version
                let prev = self.registers.a;
                let res = self.registers.a << 1;

                self.registers.a = res;
                self.set_status(Status::Carry, prev & 0b1000_0000 != 0);
                self.set_status(Status::Zero, res == 0);
                self.set_status(Status::Negative, res & 0b1000_0000 != 0);
            }
            Instruction::Lsr => {
                // TODO: memory update version
                let prev = self.registers.a;
                let res = self.registers.a >> 1;

                self.registers.a = res;
                self.set_status(Status::Carry, prev & 1 != 0);
                self.set_status(Status::Zero, res == 0);
                self.set_status(Status::Negative, res & 0b1000_0000 != 0);
            }
            other => panic!("instruction {:?} not implemented", other),
        }
    }

    pub fn get_operand_8(&self) -> u8 {
        self.mem_read(self.registers.pc + 1)
    }

    pub fn get_operand_16(&self) -> u16 {
        math::bytes_to_u16_le(&[
            self.mem_read(self.registers.pc + 1),
            self.mem_read(self.registers.pc + 2),
        ])
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
                math::byte_addr_offset(self.registers.pc, self.get_operand_8())
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

#[test]
fn test_adc() {
    struct Case {
        operand: u8,
        a: u8,
        want_a: u8,
        want_p: u8,
    }

    for (i, c) in [
        Case {
            operand: 0,
            a: 0,
            want_a: 0,
            want_p: Status::Zero.mask(),
        },
        Case {
            operand: 0,
            a: 1,
            want_a: 1,
            want_p: 0,
        },
        Case {
            operand: 1,
            a: 0,
            want_a: 1,
            want_p: 0,
        },
        Case {
            operand: 0,
            a: 0xFF,
            want_a: 0xFF,
            want_p: Status::Negative.mask(),
        },
        Case {
            operand: 1,
            a: 0xFF,
            want_a: 0,
            want_p: Status::Carry.mask() | Status::Zero.mask(),
        },
        Case {
            operand: 0x7F,
            a: 0x7F,
            want_a: 0xFE,
            want_p: Status::Overflow.mask() | Status::Negative.mask(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut cpu = Cpu::new();
        cpu.registers.a = c.a;

        cpu.execute(&ConcreteInstruction {
            ins: Instruction::Adc,
            operand8: Some(c.operand),
            operand16: None,
        });

        assert_eq!(cpu.registers.a, c.want_a, "case {}: accumulator", i);
        assert_eq!(cpu.registers.p, c.want_p, "case {}: procesor status", i);
    }
}

#[test]
fn test_and() {
    struct Case {
        operand: u8,
        a: u8,
        want_a: u8,
        want_p: u8,
    }

    for (i, c) in [
        Case {
            operand: 0,
            a: 1,
            want_a: 0,
            want_p: Status::Zero.mask(),
        },
        Case {
            operand: 0,
            a: 1,
            want_a: 0,
            want_p: Status::Zero.mask(),
        },
        Case {
            operand: 0x10,
            a: 0x11,
            want_a: 0x10,
            want_p: 0,
        },
        Case {
            operand: 0x80,
            a: 0x81,
            want_a: 0x80,
            want_p: Status::Negative.mask(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut cpu = Cpu::new();
        cpu.registers.a = c.a;

        cpu.execute(&ConcreteInstruction {
            ins: Instruction::And,
            operand8: Some(c.operand),
            operand16: None,
        });

        assert_eq!(cpu.registers.a, c.want_a, "case {}: accumulator", i);
        assert_eq!(cpu.registers.p, c.want_p, "case {}: procesor status", i);
    }
}

#[test]
fn test_asl() {
    struct Case {
        a: u8,
        want_a: u8,
        want_p: u8,
    }

    for (i, c) in [
        Case {
            a: 0,
            want_a: 0,
            want_p: Status::Zero.mask(),
        },
        Case {
            a: 0b1,
            want_a: 0b10,
            want_p: 0,
        },
        Case {
            a: 0b1000_0001,
            want_a: 0b0000_0010,
            want_p: Status::Carry.mask(),
        },
        Case {
            a: 0b1100_0000,
            want_a: 0b1000_0000,
            want_p: Status::Carry.mask() | Status::Negative.mask(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut cpu = Cpu::new();
        cpu.registers.a = c.a;

        cpu.execute(&ConcreteInstruction {
            ins: Instruction::Asl,
            operand8: None,
            operand16: None,
        });

        assert_eq!(cpu.registers.a, c.want_a, "case {}: accumulator", i);
        assert_eq!(cpu.registers.p, c.want_p, "case {}: procesor status", i);
    }
}

#[test]
fn test_lsr() {
    struct Case {
        a: u8,
        want_a: u8,
        want_p: u8,
    }

    for (i, c) in [
        Case {
            a: 0,
            want_a: 0,
            want_p: Status::Zero.mask(),
        },
        Case {
            a: 0b1000_0000,
            want_a: 0b0100_0000,
            want_p: 0,
        },
        Case {
            a: 0b11,
            want_a: 1,
            want_p: Status::Carry.mask(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut cpu = Cpu::new();
        cpu.registers.a = c.a;

        cpu.execute(&ConcreteInstruction {
            ins: Instruction::Lsr,
            operand8: None,
            operand16: None,
        });

        assert_eq!(cpu.registers.a, c.want_a, "case {}: accumulator", i);
        assert_eq!(cpu.registers.p, c.want_p, "case {}: procesor status", i);
    }
}
