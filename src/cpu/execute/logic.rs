use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;

pub fn and(cpu: &mut Cpu, operand: Operand) {
    let res = cpu.regs.a & operand.read(cpu);
    cpu.regs.a = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_and() {
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 1;
    and(&mut cpu, Operand::Immediate(0));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0;
    and(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0x11;
    and(&mut cpu, Operand::Immediate(0x10));
    assert_eq!(cpu.regs.a, 0x10);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0x81;
    and(&mut cpu, Operand::Immediate(0x80));
    assert_eq!(cpu.regs.a, 0x80);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}

pub fn eor(cpu: &mut Cpu, operand: Operand) {
    let res = cpu.regs.a ^ operand.read(cpu);
    cpu.regs.a = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_eor() {
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0;
    eor(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1111;
    eor(&mut cpu, Operand::Immediate(0b1111_1111));
    assert_eq!(cpu.regs.a, 0b1111_0000);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1111_0000;
    eor(&mut cpu, Operand::Immediate(0b1111_1111));
    assert_eq!(cpu.regs.a, 0b0000_1111);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0xFF;
    eor(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());
}

pub fn ora(cpu: &mut Cpu, operand: Operand) {
    let res = cpu.regs.a | operand.read(cpu);
    cpu.regs.a = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test_ora() {
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0;
    ora(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0;
    ora(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b0000_1111;
    ora(&mut cpu, Operand::Immediate(0b1111_0000));
    assert_eq!(cpu.regs.a, 0b1111_1111);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0;
    ora(&mut cpu, Operand::Immediate(0));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());
}

pub fn bit(cpu: &mut Cpu, operand: Operand) {
    let v = operand.read(cpu);
    cpu.regs.status_set(Status::Zero, cpu.regs.a & v == 0);
    cpu.regs.status_set(Status::Overflow, v & 0b0100_0000 != 0);
    cpu.regs.status_set(Status::Negative, v & 0b1000_0000 != 0);
}

#[test]
fn test_bit() {
    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 0);
    bit(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0xFF;
    cpu.mem_write(0, 1);
    bit(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 0b0100_0000);
    bit(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Zero.mask() | Status::Overflow.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0xFF;
    cpu.mem_write(0, 0b0100_0000);
    bit(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Overflow.mask());

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0, 0b1000_0000);
    bit(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Zero.mask() | Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0xFF;
    cpu.mem_write(0, 0b1000_0000);
    bit(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
