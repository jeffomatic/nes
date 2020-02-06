use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn tsx(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.x = cpu.regs.s;
    cpu.regs.status_set_zn(cpu.regs.x);
}

#[test]
fn test_tsx() {
    let mut cpu = Cpu::new_test();
    cpu.regs.s = 1;
    tsx(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.s = 0;
    tsx(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.s = 0xFF;
    tsx(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.x, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}

pub fn txs(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.s = cpu.regs.x;
}

#[test]
fn test_txs() {
    let mut cpu = Cpu::new_test();
    cpu.regs.x = 1;
    txs(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.s, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0;
    txs(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.s, 0);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0xFF;
    txs(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.s, 0xFF);
    assert_eq!(cpu.regs.p, 0);
}

pub fn pha(cpu: &mut Cpu, _operand: Operand) {
    cpu.stack_push(cpu.regs.a);
}

#[test]
fn test_pha() {
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 1;
    pha(&mut cpu, Operand::None);
    assert_eq!(cpu.stack_peek(0), 1);
}

pub fn php(cpu: &mut Cpu, _operand: Operand) {
    cpu.stack_push(cpu.regs.p);
}

#[test]
fn test_php() {
    let mut cpu = Cpu::new_test();
    cpu.regs.p = 1;
    php(&mut cpu, Operand::None);
    assert_eq!(cpu.stack_peek(0), 1);
}

pub fn pla(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.a = cpu.stack_pop();
}

#[test]
fn test_pla() {
    let mut cpu = Cpu::new_test();
    cpu.stack_push(1);
    pla(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.a, 1);
}

pub fn plp(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.p = cpu.stack_pop();
}

#[test]
fn test_plp() {
    let mut cpu = Cpu::new_test();
    cpu.stack_push(1);
    plp(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, 1);
}
