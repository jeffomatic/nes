use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn tax(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.x = cpu.regs.a;
    cpu.regs.status_set_zn(cpu.regs.x);
}

#[test]
fn test_tax() {
    let mut cpu = Cpu::new();
    cpu.regs.a = 1;
    tax(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    tax(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0xFF;
    tax(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}

pub fn tay(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.y = cpu.regs.a;
    cpu.regs.status_set_zn(cpu.regs.y);
}

#[test]
fn test_tay() {
    let mut cpu = Cpu::new();
    cpu.regs.a = 1;
    tay(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    tay(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0xFF;
    tay(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.y, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}

pub fn txa(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.a = cpu.regs.x;
    cpu.regs.status_set_zn(cpu.regs.a);
}

#[test]
fn test_txa() {
    let mut cpu = Cpu::new();
    cpu.regs.x = 1;
    txa(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.x = 0;
    txa(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.x = 0xFF;
    txa(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}

pub fn tya(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.a = cpu.regs.y;
    cpu.regs.status_set_zn(cpu.regs.a);
}

#[test]
fn test_tya() {
    let mut cpu = Cpu::new();
    cpu.regs.y = 1;
    tya(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.y = 0;
    tya(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.y = 0xFF;
    tya(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
