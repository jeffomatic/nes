use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    let res = cpu.regs.a & operand.read(cpu);
    cpu.regs.a = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.a = 1;
    execute(&mut cpu, Operand::Immediate(0));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0;
    execute(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0x11;
    execute(&mut cpu, Operand::Immediate(0x10));
    assert_eq!(cpu.regs.a, 0x10);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.a = 0x81;
    execute(&mut cpu, Operand::Immediate(0x80));
    assert_eq!(cpu.regs.a, 0x80);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
