use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    let res = cpu.regs.a | operand.read(cpu);
    cpu.regs.a = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.a = 0;
    execute(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.a = 0;
    execute(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0b0000_1111;
    execute(&mut cpu, Operand::Immediate(0b1111_0000));
    assert_eq!(cpu.regs.a, 0b1111_1111);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0;
    execute(&mut cpu, Operand::Immediate(0));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());
}
