use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;
use crate::math;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
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
fn test() {
    // no-mask operation
    let mut cpu = Cpu::new();
    execute(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);

    // incorporates carry
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::Carry.mask();
    execute(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 2);
    assert_eq!(cpu.regs.p, 0);

    // sets zero mask
    let mut cpu = Cpu::new();
    execute(&mut cpu, Operand::Immediate(0));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    // sets negative mask
    let mut cpu = Cpu::new();
    execute(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // sets carry
    let mut cpu = Cpu::new();
    cpu.regs.a = 0xFF;
    execute(&mut cpu, Operand::Immediate(1));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // incorporates carry to trigger carry
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::Carry.mask();
    cpu.regs.a = 0;
    execute(&mut cpu, Operand::Immediate(0xFF));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // positive overflow
    let mut cpu = Cpu::new();
    cpu.regs.a = 0x7F;
    execute(&mut cpu, Operand::Immediate(0x7F));
    assert_eq!(cpu.regs.a, 0xFE);
    assert_eq!(
        cpu.regs.p,
        Status::Overflow.mask() | Status::Negative.mask()
    );

    // negative overflow
    let mut cpu = Cpu::new();
    cpu.regs.a = 0x80;
    execute(&mut cpu, Operand::Immediate(0x80));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(
        cpu.regs.p,
        Status::Carry.mask() | Status::Zero.mask() | Status::Overflow.mask()
    );

    // carry does not trigger overflow
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::Carry.mask();
    cpu.regs.a = 0x7F;
    execute(&mut cpu, Operand::Immediate(0x80));
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());
}
