use super::super::memory::Memory;
use super::registers::Registers;

#[derive(Clone)]
pub struct State {
    pub regs: Registers,
    pub mem: Memory,
}

impl State {
    pub fn new() -> State {
        State {
            regs: Registers::default(),
            mem: Memory::new(),
        }
    }

    pub fn consume_instruction_byte(&mut self) -> u8 {
        self.regs.pc += 1;
        self.mem.read(self.regs.pc - 1)
    }
}
