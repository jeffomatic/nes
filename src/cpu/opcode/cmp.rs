use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    let opval = operand.read(cpu);
    cpu.regs.status_set(Status::Carry, cpu.regs.a >= opval);
    cpu.regs.status_set_zn(cpu.regs.a.wrapping_sub(opval));
}

#[test]
fn test() {
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
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // A < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.a = 2;
    cpu.mem_write(0x10, 0xFF);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, 0);

    // A = M
    let mut cpu = Cpu::new();
    cpu.regs.a = 3;
    cpu.mem_write(0x10, 3);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // A > M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.a = 2;
    cpu.mem_write(0x10, 1);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // A > M, N = 1
    let mut cpu = Cpu::new();
    cpu.regs.a = 0xFF;
    cpu.mem_write(0xFE, 1);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Negative.mask());
}
