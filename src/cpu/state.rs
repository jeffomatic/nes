use super::status::Status;
use crate::math;

const STACK_BASE: u16 = 0x100;
const STACK_SIZE: usize = 0x100;
const RAM_SIZE: usize = 1 << 11;
const MAX_RAM_ADDR: u16 = (RAM_SIZE - 1) as u16;

// Reference: http://obelisk.me.uk/6502/registers.html
#[derive(Clone, Debug, Eq, PartialEq)]
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
    // 4 - Break command: set during an interrupt sequence if the interrupt
    //     occurred due to user command.
    // 5 - Expansion bit: unused
    // 6 - Overflow flag: set if the last op resulted in a value greater than
    //     127. If this flag is set, the negative flag will also be set.
    // 7 - Negative flag: set if the last op resulted in a high bit of 1.
    pub p: u8,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: (STACK_SIZE - 1) as u8,
            p: 0,
        }
    }

    pub fn status_set(&mut self, s: Status, on: bool) {
        if on {
            self.p |= s.mask();
        } else {
            self.p &= !s.mask();
        }
    }

    pub fn status_set_zn(&mut self, val: u8) {
        self.status_set(Status::Zero, val == 0);
        self.status_set(Status::Negative, val & 0b1000_0000 != 0);
    }

    pub fn status_check(&self, s: Status) -> bool {
        self.p & s.mask() != 0
    }
}

#[derive(Clone, Default)]
pub struct Vectors {
    pub nmi: [u8; 2],
    pub reset: [u8; 2],
    pub irq_brk: [u8; 2],
}

#[derive(Clone)]
pub struct State {
    pub regs: Registers,
    pub ram: [u8; RAM_SIZE],
    pub vectors: Vectors,
}

impl State {
    pub fn new() -> State {
        State {
            regs: Registers::new(),
            ram: [0; RAM_SIZE],
            vectors: Vectors::default(),
        }
    }

    pub fn memread(&self, addr: u16) -> u8 {
        match addr {
            0..=MAX_RAM_ADDR => self.ram[addr as usize],
            0xFFFA => self.vectors.nmi[0],
            0xFFFB => self.vectors.nmi[1],
            0xFFFC => self.vectors.reset[0],
            0xFFFD => self.vectors.reset[1],
            0xFFFE => self.vectors.irq_brk[0],
            0xFFFF => self.vectors.irq_brk[1],
            other => panic!("no memory map for address {:?}", other),
        }
    }

    pub fn memread16(&self, addr: u16) -> u16 {
        math::bytes_to_u16_le([self.memread(addr), self.memread(addr + 1)])
    }

    pub fn memwrite(&mut self, addr: u16, v: u8) {
        match addr {
            0..=0x7FF => self.ram[addr as usize] = v,
            0xFFFA => self.vectors.nmi[0] = v,
            0xFFFB => self.vectors.nmi[1] = v,
            0xFFFC => self.vectors.reset[0] = v,
            0xFFFD => self.vectors.reset[1] = v,
            0xFFFE => self.vectors.irq_brk[0] = v,
            0xFFFF => self.vectors.irq_brk[1] = v,
            other => panic!("no memory map for address {:?}", other),
        }
    }

    pub fn consume_instruction_byte(&mut self) -> u8 {
        self.regs.pc += 1;
        self.memread(self.regs.pc - 1)
    }

    pub fn stack_pointer(&self) -> u16 {
        STACK_BASE + self.regs.s as u16
    }

    pub fn stack_push(&mut self, v: u8) {
        self.memwrite(self.stack_pointer(), v);
        self.regs.s -= 1;
    }

    pub fn stack_push16(&mut self, v: u16) {
        let bytes = math::u16_to_bytes_le(v);
        self.stack_push(bytes[0]);
        self.stack_push(bytes[1]);
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.regs.s += 1;
        self.memread(STACK_BASE + self.regs.s as u16)
    }

    pub fn stack_pop16(&mut self) -> u16 {
        let a = self.stack_pop();
        let b = self.stack_pop();
        math::bytes_to_u16_le([b, a])
    }

    /// Returns the u8 value that would be returned during a stack pop. The
    /// offset will skip backward through pushed bytes. An offset of zero
    /// denotes the most recent byte pushed to the stack.
    pub fn stack_peek(&self, offset: u8) -> u8 {
        // TODO: distinguish between read and load. memread() may be updated
        // to consume CPU cycles.
        self.memread(STACK_BASE + (self.regs.s + offset + 1) as u16)
    }

    pub fn stack_peek16(&self, offset: u8) -> u16 {
        math::bytes_to_u16_le([self.stack_peek(offset + 1), self.stack_peek(offset)])
    }
}
