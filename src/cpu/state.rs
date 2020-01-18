use super::super::memory;
use super::registers;

pub struct State {
    pub regs: registers::Registers,
    pub mem: memory::Memory,
}
