use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;

pub fn lda(cpu: &mut Cpu, operand: Operand) {
    let res = operand.read(cpu);
    cpu.regs.a = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_lda() {
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0x69;
    cpu.mem_write(0x200, 1);
    lda(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0x69;
    cpu.mem_write(0x200, 0xFF);
    lda(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0x69;
    cpu.mem_write(0x200, 0);
    lda(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());
}

pub fn ldx(cpu: &mut Cpu, operand: Operand) {
    let res = operand.read(cpu);
    cpu.regs.x = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_ldx() {
    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0x69;
    cpu.mem_write(0x200, 1);
    ldx(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.x, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0x69;
    cpu.mem_write(0x200, 0xFF);
    ldx(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.x, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.x = 0x69;
    cpu.mem_write(0x200, 0);
    ldx(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.x, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());
}

pub fn ldy(cpu: &mut Cpu, operand: Operand) {
    let res = operand.read(cpu);
    cpu.regs.y = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_ldy() {
    let mut cpu = Cpu::new_test();
    cpu.regs.y = 0x69;
    cpu.mem_write(0x200, 1);
    ldy(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.y, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.y = 0x69;
    cpu.mem_write(0x200, 0xFF);
    ldy(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.y, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.y = 0x69;
    cpu.mem_write(0x200, 0);
    ldy(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.y, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());
}

pub fn sta(cpu: &mut Cpu, operand: Operand) {
    operand.write(cpu, cpu.regs.a);
}

#[test]
fn test_sta() {
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 1;
    cpu.mem_write(0x200, 0x69);
    sta(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.mem_read(0x200), 1);
}

pub fn stx(cpu: &mut Cpu, operand: Operand) {
    operand.write(cpu, cpu.regs.x);
}

#[test]
fn test_stx() {
    let mut cpu = Cpu::new_test();
    cpu.regs.x = 1;
    cpu.mem_write(0x200, 0x69);
    stx(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.mem_read(0x200), 1);
}

pub fn sty(cpu: &mut Cpu, operand: Operand) {
    operand.write(cpu, cpu.regs.y);
}

#[test]
fn test() {
    let mut cpu = Cpu::new_test();
    cpu.regs.y = 1;
    cpu.mem_write(0x200, 0x69);
    sty(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.mem_read(0x200), 1);
}
