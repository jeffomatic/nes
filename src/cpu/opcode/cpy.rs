use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    let opval = operand.read(cpu);
    cpu.regs.status_set(Status::Carry, cpu.regs.y >= opval);
    cpu.regs.status_set_zn(cpu.regs.y.wrapping_sub(opval));
}

#[test]
fn test() {
    // See cmp implementation for notes

    // Y < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.y = 3;
    cpu.mem_write(0x10, 4);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    // Y < M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.y = 2;
    cpu.mem_write(0x10, 0xFF);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, 0);

    // Y = M
    let mut cpu = Cpu::new();
    cpu.regs.y = 3;
    cpu.mem_write(0x10, 3);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // Y > M, N = 0
    let mut cpu = Cpu::new();
    cpu.regs.y = 2;
    cpu.mem_write(0x10, 1);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask());

    // Y > M, N = 1
    let mut cpu = Cpu::new();
    cpu.regs.y = 0xFF;
    cpu.mem_write(0xFE, 1);
    execute(&mut cpu, Operand::Memory(0x10));
    assert_eq!(cpu.regs.p, Status::Carry.mask() | Status::Negative.mask());
}
