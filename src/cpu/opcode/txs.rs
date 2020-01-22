use super::super::operand::Operand;
use super::super::state::Cpu;

pub fn execute(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.s = cpu.regs.x;
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.x = 1;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.s, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.x = 0;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.s, 0);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.x = 0xFF;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.s, 0xFF);
    assert_eq!(cpu.regs.p, 0);
}
