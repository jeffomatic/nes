use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;
use crate::math;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
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
fn test() {
    // no-borrow subtraction, positive result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    cpu.regs.p = Status::Carry.mask();
    execute(&mut cpu, Operand::Immediate(3));
    assert_eq!(cpu.regs.a, 2);
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // no-borrow subtraction, zero result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    cpu.regs.p = Status::Carry.mask();
    execute(&mut cpu, Operand::Immediate(5));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // no-borrow subtraction, negative result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    cpu.regs.p = Status::Carry.mask();
    execute(&mut cpu, Operand::Immediate(6));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // no-borrow subtraction, positive result with overflow
    let mut cpu = Cpu::new();
    cpu.regs.a = 0x7F;
    cpu.regs.p = Status::Carry.mask();
    execute(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0x80);
    assert_eq!(
        cpu.regs.p,
        Status::Overflow.mask() | Status::Negative.mask()
    );

    // no-borrow subtraction, negative result with overflow
    let mut cpu = Cpu::new();
    cpu.regs.a = 0x80;
    cpu.regs.p = Status::Carry.mask();
    execute(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 0x7F);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Overflow.mask());

    // borrow subtraction, positive result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    execute(&mut cpu, Operand::Immediate(3));
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // borrow subtraction, zero result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    execute(&mut cpu, Operand::Immediate(4));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // borrow subtraction, negative result
    let mut cpu = Cpu::new();
    cpu.regs.a = 5;
    execute(&mut cpu, Operand::Immediate(5));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
