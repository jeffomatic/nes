use super::super::operand::Operand;
use super::super::state::Cpu;

pub fn execute(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.p = cpu.stack_pop();
    cpu.regs.pc = cpu.stack_pop16();
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.stack_push(0x69); // sentinel
    cpu.stack_push16(0x200); // PC
    cpu.stack_push(1); // status
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.pc, 0x200);
    assert_eq!(cpu.regs.p, 1);
    assert_eq!(cpu.stack_peek(0), 0x69);
}
