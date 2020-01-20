use super::registers::Registers;
use crate::math;

const STACK_BASE: u16 = 0x100;
const RAM_SIZE: usize = 1 << 11;
const MAX_RAM_ADDR: u16 = (RAM_SIZE - 1) as u16;

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
            regs: Registers::default(),
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
        math::byte_addr_offset(STACK_BASE, self.regs.s)
    }

    pub fn stack_push(&mut self, v: u8) {
        self.memwrite(self.stack_pointer(), v);
        self.regs.s += 1;
    }

    pub fn stack_push16(&mut self, v: u16) {
        let bytes = math::u16_to_bytes_le(v);
        self.stack_push(bytes[0]);
        self.stack_push(bytes[1]);
    }
}
