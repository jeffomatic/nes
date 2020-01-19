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
}
