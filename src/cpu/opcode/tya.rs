use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn execute(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.a = cpu.regs.y;
    cpu.regs.status_set_zn(cpu.regs.a);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.y = 1;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.y = 0;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.y = 0xFF;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
