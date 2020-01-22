use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn execute(cpu: &mut Cpu, _operand: Operand) {
    let res = cpu.regs.y.wrapping_sub(1);
    cpu.regs.y = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.y = 2;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.y = 1;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.y = 0;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new();
    cpu.regs.y = 0xFF;
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0xFE);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
