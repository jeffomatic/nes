use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn clc(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.status_set(Status::Carry, false);
}

#[test]
fn test_clc() {
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::Carry.mask();
    clc(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, 0);
}

pub fn cld(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.status_set(Status::DecimalMode, false);
}

#[test]
fn test_cld() {
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::DecimalMode.mask();
    cld(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, 0);
}

pub fn cli(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.status_set(Status::InterruptDisable, false);
}

#[test]
fn test_cli() {
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::InterruptDisable.mask();
    cli(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, 0);
}

pub fn clv(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.status_set(Status::Overflow, false);
}

#[test]
fn test_clv() {
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::Overflow.mask();
    clv(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, 0);
}

pub fn sec(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.status_set(Status::Carry, true);
}

#[test]
fn test_sec() {
    let mut cpu = Cpu::new();
    sec(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, Status::Carry.mask());
}

pub fn sed(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.status_set(Status::DecimalMode, true);
}

#[test]
fn test_sed() {
    let mut cpu = Cpu::new();
    sed(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, Status::DecimalMode.mask());
}

pub fn sei(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.status_set(Status::InterruptDisable, true);
}

#[test]
fn test_sei() {
    let mut cpu = Cpu::new();
    sei(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, Status::InterruptDisable.mask());
}
