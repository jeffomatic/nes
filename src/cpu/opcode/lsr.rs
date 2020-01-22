use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    let prev = operand.read(cpu);
    let res = prev >> 1;
    operand.write(cpu, res);
    cpu.regs.status_set_zn(res);
    cpu.regs.status_set(Status::Carry, prev & 1 != 0);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    execute(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0b1000_0000;
    execute(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b0100_0000);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.a = 0b11;
    execute(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    let mut cpu = Cpu::new();
    cpu.mem_write(0x10, 0b10);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.mem_read(0x10), 1);
    assert_eq!(cpu.regs.p, 0);

    // ensure that carry isn't transferred
    let mut cpu = Cpu::new();
    cpu.regs.a = 0b1000_0000;
    cpu.regs.p = Status::Carry.mask();
    execute(&mut cpu, Operand::Accumulator);
    assert_eq!(cpu.regs.a, 0b0100_0000);
    assert_eq!(cpu.regs.p, 0);
}
