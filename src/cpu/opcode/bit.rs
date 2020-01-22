use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    let v = operand.read(cpu);
    cpu.regs.status_set(Status::Zero, cpu.regs.a & v == 0);
    cpu.regs.status_set(Status::Overflow, v & 0b0100_0000 != 0);
    cpu.regs.status_set(Status::Negative, v & 0b1000_0000 != 0);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0xFF;
    cpu.mem_write(0, 1);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0b0100_0000);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Zero.mask() | Status::Overflow.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0xFF;
    cpu.mem_write(0, 0b0100_0000);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Overflow.mask());

    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0b1000_0000);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Zero.mask() | Status::Negative.mask());

    let mut cpu = Cpu::new();
    cpu.regs.a = 0xFF;
    cpu.mem_write(0, 0b1000_0000);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
