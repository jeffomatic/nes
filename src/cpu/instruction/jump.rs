use super::super::operand::Operand;
use super::super::state::Cpu;

pub fn jmp(cpu: &mut Cpu, operand: Operand) {
    cpu.regs.pc = operand.address();
}

#[test]
fn test_jmp() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    jmp(&mut cpu, Operand::Memory(0x20));
    assert_eq!(cpu.regs.pc, 0x20);
}

pub fn jsr(cpu: &mut Cpu, operand: Operand) {
    cpu.stack_push16(cpu.regs.pc - 1);
    cpu.regs.pc = operand.address();
}

#[test]
fn test_jsr() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x202;
    jsr(&mut cpu, Operand::Memory(0x300));
    assert_eq!(cpu.stack_peek16(0), 0x201);
    assert_eq!(cpu.regs.pc, 0x300);
}

pub fn rts(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.pc = cpu.stack_pop16() + 1;
}

#[test]
fn test_rts() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x300;
    cpu.stack_push(0x69); // this is just a sentinel for the test
    cpu.stack_push16(0x201);
    rts(&mut cpu, Operand::None);
    assert_eq!(cpu.stack_peek(0), 0x69);
    assert_eq!(cpu.regs.pc, 0x202);
}
