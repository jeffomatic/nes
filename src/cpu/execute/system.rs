use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn brk(cpu: &mut Cpu, _operand: Operand) {
    cpu.stack_push16(cpu.regs.pc);
    cpu.stack_push(cpu.regs.p);
    cpu.regs.status_set(Status::BreakCommand, true);
    cpu.regs.pc = cpu.mem_read16(0xFFFE);
}

#[test]
fn test_brk() {
    let mut cpu = Cpu::new_test();
    cpu.regs.pc = 0x201;
    cpu.regs.p = 0b1000_0001;
    cpu.mem_write(0xFFFE, 0xFF);
    cpu.mem_write(0xFFFF, 0x02);
    brk(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.pc, 0x2FF);
    assert_eq!(cpu.regs.p, 0b1001_0001);
    assert_eq!(cpu.stack_peek(0), 0b1000_0001);
    assert_eq!(cpu.stack_peek16(1), 0x201);
}

pub fn rti(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.p = cpu.stack_pop();
    cpu.regs.pc = cpu.stack_pop16();
}

#[test]
fn test_rti() {
    let mut cpu = Cpu::new_test();
    cpu.stack_push(0x69); // sentinel
    cpu.stack_push16(0x200); // PC
    cpu.stack_push(1); // status
    rti(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.pc, 0x200);
    assert_eq!(cpu.regs.p, 1);
    assert_eq!(cpu.stack_peek(0), 0x69);
}

pub fn nop(_cpu: &mut Cpu, _operand: Operand) {}
