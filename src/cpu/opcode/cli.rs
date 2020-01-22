use super::super::operand::Operand;
use super::super::state::Cpu;
use super::super::status::Status;

pub fn execute(cpu: &mut Cpu, _operand: Operand) {
    cpu.regs.status_set(Status::InterruptDisable, false);
}

#[test]
fn test() {
    let mut cpu = Cpu::new();
    cpu.regs.p = Status::InterruptDisable.mask();
    execute(&mut cpu, Operand::None);
    assert_eq!(cpu.regs.p, 0);
}
