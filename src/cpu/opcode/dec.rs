use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    let prev = operand.read(cpu);
    let res = prev.wrapping_sub(1);
    operand.write(cpu, res);
    cpu.regs.status_set_zn(res);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0, 2);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.mem_write(0, 1);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());

    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new();
    cpu.mem_write(0, 0xFF);
    execute(&mut cpu, Operand::Memory(0));
    assert_eq!(cpu.mem_read(0), 0xFE);
    assert_eq!(cpu.regs.p, Status::Negative.mask());
}
