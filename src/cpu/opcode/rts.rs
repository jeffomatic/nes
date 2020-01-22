use super::super::operand::Operand;
use super::super::state::Cpu;

pub fn execute(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.pc = cpu.stack_pop16() + 1;
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x300;
    cpu.stack_push(0x69); // this is just a sentinel for the test
    cpu.stack_push16(0x201);
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.stack_peek(0), 0x69);
    assert_eq!(cpu.regs.pc, 0x202);
}
