use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;

pub fn execute(cpu: &mut Cpu, _operand: Operand) {
    cpu.stack_push(cpu.regs.p);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.p = 1;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.stack_peek(0), 1);
}
