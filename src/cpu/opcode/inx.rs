use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn execute(cpu: &mut Cpu, _operand: Operand) {
    let res = cpu.regs.x.wrapping_add(1);
    cpu.regs.x = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.x = 0xFE;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new();
    cpu.regs.x = 0xFF;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.x = 0;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.x = 1;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 2);
    assert_eq!(cpu.regs.p, 0);
}
