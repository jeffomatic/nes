use super::super::memory;
use super::registers;

struct State {
    regs: registers::Registers,
    mem: memory::Memory,
}
