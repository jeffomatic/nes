use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;

pub fn execute(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.p = cpu.stack_pop();
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.stack_push(1);
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, 1);
}
