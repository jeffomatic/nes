use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    let opval = operand.read(cpu);
    cpu.regs.status_set(Status::Carry, cpu.regs.x >= opval);
    cpu.regs.status_set_zn(cpu.regs.x.wrapping_sub(opval));
}

#[test]
fn test() {
    // See cmp implementation for notes

    // X < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.x = 3;
    cpu.mem_write(0x10, 4);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // X < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.x = 2;
    cpu.mem_write(0x10, 0xFF);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, 0);

    // X = M
    let mut cpu = Cpu::new();
    cpu.regs.x = 3;
    cpu.mem_write(0x10, 3);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // X > M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.x = 2;
    cpu.mem_write(0x10, 1);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // X > M, N = 1
    let mut cpu = Cpu::new();
    cpu.regs.x = 0xFF;
    cpu.mem_write(0xFE, 1);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Negative.mask());
}
