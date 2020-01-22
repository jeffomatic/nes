use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;

pub fn execute(cpu: &mut Cpu, operand: Operand) {
    operand.write(cpu, cpu.regs.x);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.x = 1;
    cpu.mem_write(0x200, 0x69);
    execute(&mut cpu, Operand::Memory(0x200));
    assert_eq!(cpu.mem_read(0x200), 1);
}
