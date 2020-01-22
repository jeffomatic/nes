use super::super::operand::Operand;
use super::super::state::Cpu;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    cpu.regs.pc = operand.address();
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    execute(&mut cpu, Operand::Memory(0x20));
    assert_eq!(cpu.regs.pc, 0x20);
}
