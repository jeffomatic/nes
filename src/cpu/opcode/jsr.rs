use super::super::operand::Operand;
use super::super::state::Cpu;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    cpu.stack_push16(cpu.regs.pc - 1);
    cpu.regs.pc = operand.address();
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x202;
    execute(&mut cpu, Operand::Memory(0x300));
    assert_eq!(cpu.stack_peek16(0), 0x201);
    assert_eq!(cpu.regs.pc, 0x300);
}
