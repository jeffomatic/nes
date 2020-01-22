use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;
use crate::cpu::status::Status;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    let res = operand.read(cpu);
    cpu.regs.x = res;
    cpu.regs.status_set_zn(res);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.x = 0x69;
    cpu.mem_write(0x200, 1);
    execute(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.x, 1);
    assert_eq!(cpu.regs.p, 0);

    let mut cpu = Cpu::new();
    cpu.regs.x = 0x69;
    cpu.mem_write(0x200, 0xFF);
    execute(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.x, 0xFF);
    assert_eq!(cpu.regs.p, Status::Negative.mask());

    let mut cpu = Cpu::new();
    cpu.regs.x = 0x69;
    cpu.mem_write(0x200, 0);
    execute(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.regs.x, 0);
    assert_eq!(cpu.regs.p, Status::Zero.mask());
}
