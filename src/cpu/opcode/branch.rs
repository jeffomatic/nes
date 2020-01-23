use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;
use crate::math;

fn branch(cpu: &mut Cpu, operand: Operand, cond: bool) {
    if !cond {
        return;
    }

    cpu.cycle_add(1);
    let addr = math::byte_addr_offset(cpu.regs.pc, operand.read(cpu));
    if math::page_crossing(cpu.regs.pc, addr) {
        cpu.cycle_add(1);
    }

    cpu.regs.pc = addr;
}

#[test]
fn test_branch_cycle() {
    // no branch
    let mut cpu = Cpu::new();
    branch(&mut cpu, Operand::Immediate(1), false);
    assert_eq!(cpu.cycles, 0);

    // branch, no page crossing
    let mut cpu = Cpu::new();
    branch(&mut cpu, Operand::Immediate(1), true);
    assert_eq!(cpu.cycles, 1);

    // branch, positive page crossing
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0xFF;
    branch(&mut cpu, Operand::Immediate(1), true);
    assert_eq!(cpu.cycles, 2);

    // branch, negative page crossing
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x100;
    branch(&mut cpu, Operand::Immediate(0x80), true);
    assert_eq!(cpu.cycles, 2);
}

pub fn bcc(cpu: &mut Cpu, operand: Operand) {
    branch(cpu, operand, !cpu.regs.status_check(Status::Carry));
}

#[test]
fn test_bcc() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bcc(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x12);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Carry.mask();
    bcc(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x10);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bcc(&mut cpu, Operand::Immediate(0xFE));
    assert_eq!(cpu.regs.pc, 0xE);
}

pub fn bcs(cpu: &mut Cpu, operand: Operand) {
    branch(cpu, operand, cpu.regs.status_check(Status::Carry))
}

#[test]
fn test_bcs() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Carry.mask();
    bcs(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x12);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bcs(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x10);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Carry.mask();
    bcs(&mut cpu, Operand::Immediate(0xFE));
    assert_eq!(cpu.regs.pc, 0xE);
}

pub fn beq(cpu: &mut Cpu, operand: Operand) {
    branch(cpu, operand, cpu.regs.status_check(Status::Zero))
}

#[test]
fn test_beq() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Zero.mask();
    beq(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x12);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    beq(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x10);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Zero.mask();
    beq(&mut cpu, Operand::Immediate(0xFE));
    assert_eq!(cpu.regs.pc, 0xE);
}

pub fn bmi(cpu: &mut Cpu, operand: Operand) {
    branch(cpu, operand, cpu.regs.status_check(Status::Negative));
}

#[test]
fn test_bmi() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Negative.mask();
    bmi(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x12);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bmi(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x10);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Negative.mask();
    bmi(&mut cpu, Operand::Immediate(0xFE));
    assert_eq!(cpu.regs.pc, 0xE);
}

pub fn bne(cpu: &mut Cpu, operand: Operand) {
    branch(cpu, operand, !cpu.regs.status_check(Status::Zero));
}

#[test]
fn test_bne() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bne(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x12);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Zero.mask();
    bne(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x10);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bne(&mut cpu, Operand::Immediate(0xFE));
    assert_eq!(cpu.regs.pc, 0xE);
}

pub fn bpl(cpu: &mut Cpu, operand: Operand) {
    branch(cpu, operand, !cpu.regs.status_check(Status::Negative))
}

#[test]
fn test_bpl() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bpl(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x12);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Negative.mask();
    bpl(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x10);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bpl(&mut cpu, Operand::Immediate(0xFE));
    assert_eq!(cpu.regs.pc, 0xE);
}

pub fn bvc(cpu: &mut Cpu, operand: Operand) {
    branch(cpu, operand, !cpu.regs.status_check(Status::Overflow));
}

#[test]
fn test_bvc() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bvc(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x12);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Overflow.mask();
    bvc(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x10);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bvc(&mut cpu, Operand::Immediate(0xFE));
    assert_eq!(cpu.regs.pc, 0xE);
}

pub fn bvs(cpu: &mut Cpu, operand: Operand) {
    branch(cpu, operand, cpu.regs.status_check(Status::Overflow));
}

#[test]
fn test_bvs() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Overflow.mask();
    bvs(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x12);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    bvs(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x10);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Overflow.mask();
    bvs(&mut cpu, Operand::Immediate(0xFE));
    assert_eq!(cpu.regs.pc, 0xE);
}
