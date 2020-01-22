use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;
use crate::math;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    if !cpu.regs.status_check(Status::Overflow) {
        return;
    }

    cpu.regs.pc = math::byte_addr_offset(cpu.regs.pc, operand.read(cpu));
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Overflow.mask();
    execute(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x12);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    execute(&mut cpu, Operand::Immediate(2));
    assert_eq!(cpu.regs.pc, 0x10);

    let mut cpu = Cpu::new();
    cpu.regs.pc = 0x10;
    cpu.regs.p = Status::Overflow.mask();
    execute(&mut cpu, Operand::Immediate(0xFE));
    assert_eq!(cpu.regs.pc, 0xE);
}
