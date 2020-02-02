use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;
use crate::math;

pub fn adc(cpu: &mut Cpu, operand: Operand) {
    let prev = cpu.regs.a;
    let opval = operand.read(cpu);
    let res16 = prev as u16
        + opval as u16
        + if cpu.regs.status_check(Status::Carry) {
            1
        } else {
            0
        };
    let res = res16 as u8;

    cpu.regs.a = res;
    cpu.regs.status_set_zn(res);
    cpu.regs.status_set(Status::Carry, res16 & 0x100 != 0);

    // Overflow occurs in the following cases:
    // positive + positive = negative
    // negative + negative = positive
    cpu.regs.status_set(
        Status::Overflow,
        math::same_sign(prev, opval) && !math::same_sign(prev, res),
    );
}

#[test]
fn test_adc() {
    // no-mask operation
    let mut cpu = Cpu::new();
    adc(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);

    // incorporates carry
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::Carry.mask();
    adc(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 2);
    assert_eq!(cpu.regs.p, 0);

    // sets zero mask
    let mut cpu = Cpu::new();
    adc(&mut cpu, Operand::Immediate(0));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    // sets negative mask
    let mut cpu = Cpu::new();
    adc(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // sets carry
    let mut cpu = Cpu::new();
    cpu.regs.a = 0xFF;
    adc(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // incorporates carry to trigger carry
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::Carry.mask();
    cpu.regs.a = 0;
    adc(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // positive overflow
    let mut cpu = Cpu::new();
    cpu.regs.a = 0x7F;
    adc(&mut cpu, Operand::Immediate(0x7F));
    assert_eq!(cpu.regs.a, 0xFE);
    assert_eq!(
        cpu.regs.p,
        Status::Overflow.mask() | Status::Negative.mask()
    );

    // negative overflow
    let mut cpu = Cpu::new();
    cpu.regs.a = 0x80;
    adc(&mut cpu, Operand::Immediate(0x80));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(
        cpu.regs.p,
        Status::Carry.mask() | Status::Zero.mask() | Status::Overflow.mask()
    );

    // carry does not trigger overflow
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::Carry.mask();
    cpu.regs.a = 0x7F;
    adc(&mut cpu, Operand::Immediate(0x80));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());
}

pub fn sbc(cpu: &mut Cpu, operand: Operand) {
    let prev = cpu.regs.a;
    let opval = operand.read(cpu);
    let complement = !opval as u16
        + if cpu.regs.status_check(Status::Carry) {
            1
        } else {
            0
        };
    let res16 = prev as u16 + complement;
    let res = res16 as u8;

    cpu.regs.a = res;
    cpu.regs.status_set_zn(res);
    cpu.regs.status_set(Status::Carry, res16 & 0x100 != 0);

    // Overflow occurs in the following cases:
    // positive - negative = negative
    // negative - positive = positive
    cpu.regs.status_set(
        Status::Overflow,
        !math::same_sign(prev, opval) && math::same_sign(opval, res),
    );
}

#[test]
fn test_sbc() {
    // no-borrow subtraction, positive result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    cpu.regs.p = Status::Carry.mask();
    sbc(&mut cpu, Operand::Immediate(3));
    assert_eq!(cpu.regs.a, 2);
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // no-borrow subtraction, zero result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    cpu.regs.p = Status::Carry.mask();
    sbc(&mut cpu, Operand::Immediate(5));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // no-borrow subtraction, negative result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    cpu.regs.p = Status::Carry.mask();
    sbc(&mut cpu, Operand::Immediate(6));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // no-borrow subtraction, positive result with overflow
    let mut cpu = Cpu::new();
    cpu.regs.a = 0x7F;
    cpu.regs.p = Status::Carry.mask();
    sbc(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0x80);
    assert_eq!(
        cpu.regs.p,
        Status::Overflow.mask() | Status::Negative.mask()
    );

    // no-borrow subtraction, negative result with overflow
    let mut cpu = Cpu::new();
    cpu.regs.a = 0x80;
    cpu.regs.p = Status::Carry.mask();
    sbc(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 0x7F);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Overflow.mask());

    // borrow subtraction, positive result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    sbc(&mut cpu, Operand::Immediate(3));
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // borrow subtraction, zero result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    sbc(&mut cpu, Operand::Immediate(4));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // borrow subtraction, negative result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    sbc(&mut cpu, Operand::Immediate(5));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}

pub fn cmp(cpu: &mut Cpu, operand: Operand) {
    let opval = operand.read(cpu);
    cpu.regs.status_set(Status::Carry, cpu.regs.a >= opval);
    cpu.regs.status_set_zn(cpu.regs.a.wrapping_sub(opval));
}

#[test]
fn test_cmp() {
    // Reference for flag cpus:
    // http://users.telenet.be/kim1-6502/6502/proman.html#421
    //
    // Carry is set if the A >= M unsigned comparison is true
    // Zero is set if A - M is zero
    // Negative is set if A - M is negative

    // A < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.a = 3;
    cpu.mem_write(0x10, 4);
    cmp(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // A < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.a = 2;
    cpu.mem_write(0x10, 0xFF);
    cmp(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, 0);

    // A = M
    let mut cpu = Cpu::new();
    cpu.regs.a = 3;
    cpu.mem_write(0x10, 3);
    cmp(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // A > M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.a = 2;
    cpu.mem_write(0x10, 1);
    cmp(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // A > M, N = 1
    let mut cpu = Cpu::new();
    cpu.regs.a = 0xFF;
    cpu.mem_write(0xFE, 1);
    cmp(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Negative.mask());
}

pub fn cpx(cpu: &mut Cpu, operand: Operand) {
    let opval = operand.read(cpu);
    cpu.regs.status_set(Status::Carry, cpu.regs.x >= opval);
    cpu.regs.status_set_zn(cpu.regs.x.wrapping_sub(opval));
}

#[test]
fn test_cpx() {
    // See cmp implementation for notes

    // X < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.x = 3;
    cpu.mem_write(0x10, 4);
    cpx(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // X < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.x = 2;
    cpu.mem_write(0x10, 0xFF);
    cpx(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, 0);

    // X = M
    let mut cpu = Cpu::new();
    cpu.regs.x = 3;
    cpu.mem_write(0x10, 3);
    cpx(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // X > M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.x = 2;
    cpu.mem_write(0x10, 1);
    cpx(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // X > M, N = 1
    let mut cpu = Cpu::new();
    cpu.regs.x = 0xFF;
    cpu.mem_write(0xFE, 1);
    cpx(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Negative.mask());
}

pub fn cpy(cpu: &mut Cpu, operand: Operand) {
    let opval = operand.read(cpu);
    cpu.regs.status_set(Status::Carry, cpu.regs.y >= opval);
    cpu.regs.status_set_zn(cpu.regs.y.wrapping_sub(opval));
}

#[test]
fn test_cpy() {
    // See cmp implementation for notes

    // Y < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.y = 3;
    cpu.mem_write(0x10, 4);
    cpy(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // Y < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.y = 2;
    cpu.mem_write(0x10, 0xFF);
    cpy(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, 0);

    // Y = M
    let mut cpu = Cpu::new();
    cpu.regs.y = 3;
    cpu.mem_write(0x10, 3);
    cpy(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // Y > M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.y = 2;
    cpu.mem_write(0x10, 1);
    cpy(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // Y > M, N = 1
    let mut cpu = Cpu::new();
    cpu.regs.y = 0xFF;
    cpu.mem_write(0xFE, 1);
    cpy(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Negative.mask());
}
