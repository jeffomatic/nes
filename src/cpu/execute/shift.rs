use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;

pub fn asl(cpu: &mut Cpu, operand: Operand) {
    let prev = operand.read(cpu);
    let res = prev << 1;
    operand.write(cpu, res);
    cpu.regs.status_set_zn(res);
    cpu.regs.status_set(Status::Carry, prev & 0b1000_0000 != 0);
}

#[test]
fn test_asl() {
    let mut cpu = Cpu::new_test();
    asl(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 1;
    asl(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b10);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1000_0001;
    asl(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b10);
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1100_0000;
    asl(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b1000_0000);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0x10, 1);
    asl(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.mem_read(0x10), 0b10);
    assert_eq!(cpu.regs.p, 0);

    // ensure carry is not transferred
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b0000_0001;
    cpu.regs.p = Status::Carry.mask();
    asl(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b10);
    assert_eq!(cpu.regs.p, 0);
}

pub fn lsr(cpu: &mut Cpu, operand: Operand) {
    let prev = operand.read(cpu);
    let res = prev >> 1;
    operand.write(cpu, res);
    cpu.regs.status_set_zn(res);
    cpu.regs.status_set(Status::Carry, prev & 1 != 0);
}

#[test]
fn test_lsr() {
    let mut cpu = Cpu::new_test();
    lsr(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1000_0000;
    lsr(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b0100_0000);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b11;
    lsr(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0x10, 0b10);
    lsr(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.mem_read(0x10), 1);
    assert_eq!(cpu.regs.p, 0);

    // ensure that carry isn't transferred
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1000_0000;
    cpu.regs.p = Status::Carry.mask();
    lsr(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b0100_0000);
    assert_eq!(cpu.regs.p, 0);
}

pub fn rol(cpu: &mut Cpu, operand: Operand) {
    let prev = operand.read(cpu);
    let res = (prev << 1)
        | if cpu.regs.status_check(Status::Carry) {
            1
        } else {
            0
        };
    operand.write(cpu, res);
    cpu.regs.status_set_zn(res);
    cpu.regs.status_set(Status::Carry, prev & 0b1000_0000 != 0);
}

#[test]
fn test_rol() {
    let mut cpu = Cpu::new_test();
    rol(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 1;
    rol(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b10);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1000_0001;
    rol(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b10);
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1100_0000;
    rol(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b1000_0000);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Negative.mask());

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0x10, 1);
    rol(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.mem_read(0x10), 0b10);
    assert_eq!(cpu.regs.p, 0);

    // ensure carry is transferred
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b0000_0001;
    cpu.regs.p = Status::Carry.mask();
    rol(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b11);
    assert_eq!(cpu.regs.p, 0);
}

pub fn ror(cpu: &mut Cpu, operand: Operand) {
    let prev = operand.read(cpu);
    let res = prev >> 1
        | if cpu.regs.status_check(Status::Carry) {
            0b1000_0000
        } else {
            0
        };
    operand.write(cpu, res);
    cpu.regs.status_set_zn(res);
    cpu.regs.status_set(Status::Carry, prev & 1 != 0);
}

#[test]
fn test_ror() {
    let mut cpu = Cpu::new_test();
    ror(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1000_0000;
    ror(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b0100_0000);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b11;
    ror(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    let mut cpu = Cpu::new_test();
    cpu.mem_write(0x10, 0b10);
    ror(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.mem_read(0x10), 1);
    assert_eq!(cpu.regs.p, 0);

    // ensure that carry is transferred
    let mut cpu = Cpu::new_test();
    cpu.regs.a = 0b1000_0000;
    cpu.regs.p = Status::Carry.mask();
    ror(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b1100_0000);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
