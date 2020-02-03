use super::super::ppu;
use super::status::Status;
use crate::math;

const STACK_BASE: u16 = 0x100;
const STACK_SIZE: usize = 0x100;
const RAM_SIZE: usize = 1 << 11;

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
    // 6 - Overflow flag: set if the last op resulted in a value larger than 127
    //     or less than -128. If the overflow flag is set, the negative flag
    //     should be interpreted with opposite semantics.
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
    pub nmi: u16,
    pub reset: u16,
    pub irq_brk: u16,
}

pub struct Cpu {
    pub cycles: u64,
    pub regs: Registers,
    pub ram: [u8; RAM_SIZE],
    pub vectors: Vectors,
    pub ppu: ppu::Ppu,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            cycles: 0,
            regs: Registers::new(),
            ram: [0; RAM_SIZE],
            vectors: Vectors::default(),
            ppu: ppu::Ppu::new(),
        }
    }

    pub fn cycle_add(&mut self, amt: u64) {
        self.cycles += amt;
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            // Source: https://wiki.nesdev.com/w/index.php/CPU_memory_map
            0..=0x07FF => self.ram[addr],
            0x0800..=0x0FFF => self.ram[addr - 0x0800],
            0x1000..=0x17FF => self.ram[addr - 0x1000],
            0x1800..=0x1FFF => self.ram[addr - 0x1800],
            0x2000..=0x3FFF => match addr % 8 {
                0 => self.ppu.regs.ppuctrl,
                1 => self.ppu.regs.ppumask,
                2 => self.ppu.regs.ppustatus,
                3 => self.ppu.regs.oamaddr,
                4 => self.ppu.regs.oamdata,
                5 => self.ppu.regs.ppuscroll,
                6 => self.ppu.regs.ppuaddr,
                7 => self.ppu.regs.ppudata,
                _ => unreachable!(),
            },
            0x4014 => self.ppu.regs.oamdata,
            0xFFFA => math::u16_lo(self.vectors.nmi),
            0xFFFB => math::u16_hi(self.vectors.nmi),
            0xFFFC => math::u16_lo(self.vectors.reset),
            0xFFFD => math::u16_hi(self.vectors.reset),
            0xFFFE => math::u16_lo(self.vectors.irq_brk),
            0xFFFF => math::u16_hi(self.vectors.irq_brk),
            other => panic!("no memory map for address {:?}", other),
        }
    }

    pub fn mem_read16(&self, addr: u16) -> u16 {
        math::bytes_to_u16_le([self.mem_read(addr), self.mem_read(addr + 1)])
    }

    pub fn mem_read_buf(&self, addr: u16, len: usize) -> Vec<u8> {
        let mut res = Vec::with_capacity(len);
        for i in 0..len {
            res.push(self.mem_read(addr + i as u16));
        }
        res
    }

    pub fn mem_write(&mut self, addr: u16, v: u8) {
        let addr = addr as usize;
        match addr as usize {
            0..=0x07FF => self.ram[addr] = v,
            0x0800..=0x0FFF => self.ram[addr - 0x0800] = v,
            0x1000..=0x17FF => self.ram[addr - 0x1000] = v,
            0x1800..=0x1FFF => self.ram[addr - 0x1800] = v,
            0x2000..=0x3FFF => match addr % 8 {
                0 => self.ppu.regs.ppuctrl = v,
                1 => self.ppu.regs.ppumask = v,
                2 => self.ppu.regs.ppustatus = v,
                3 => self.ppu.regs.oamaddr = v,
                4 => self.ppu.regs.oamdata = v,
                5 => self.ppu.regs.ppuscroll = v,
                6 => self.ppu.regs.ppuaddr = v,
                7 => self.ppu.regs.ppudata = v,
                _ => unreachable!(),
            },
            0x4014 => self.ppu.regs.oamdata = v,
            0xFFFA => self.vectors.nmi = math::u16_set_lo(self.vectors.nmi, v),
            0xFFFB => self.vectors.nmi = math::u16_set_hi(self.vectors.nmi, v),
            0xFFFC => self.vectors.reset = math::u16_set_lo(self.vectors.reset, v),
            0xFFFD => self.vectors.reset = math::u16_set_hi(self.vectors.reset, v),
            0xFFFE => self.vectors.irq_brk = math::u16_set_lo(self.vectors.irq_brk, v),
            0xFFFF => self.vectors.irq_brk = math::u16_set_hi(self.vectors.irq_brk, v),
            other => panic!("no memory map for address {:?}", other),
        }
    }

    pub fn mem_write_buf(&mut self, addr: u16, buf: Vec<u8>) {
        for (i, v) in buf.iter().enumerate() {
            self.mem_write(addr + i as u16, *v);
        }
    }

    pub fn instruction_fetch_byte(&mut self) -> u8 {
        self.regs.pc += 1;
        self.mem_read(self.regs.pc - 1)
    }

    pub fn stack_pointer(&self) -> u16 {
        STACK_BASE + self.regs.s as u16
    }

    pub fn stack_push(&mut self, v: u8) {
        self.mem_write(self.stack_pointer(), v);
        self.regs.s -= 1;
    }

    pub fn stack_push16(&mut self, v: u16) {
        let bytes = math::u16_to_bytes_le(v);
        self.stack_push(bytes[0]);
        self.stack_push(bytes[1]);
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.regs.s += 1;
        self.mem_read(STACK_BASE + self.regs.s as u16)
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
        // TODO: distinguish between read and load. mem_read() may be updated
        // to consume CPU cycles.
        self.mem_read(STACK_BASE + (self.regs.s + offset + 1) as u16)
    }

    pub fn stack_peek16(&self, offset: u8) -> u16 {
        math::bytes_to_u16_le([self.stack_peek(offset + 1), self.stack_peek(offset)])
    }
}
