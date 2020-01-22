use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;
use crate::math;

pub fn execute(state: &mut State, operand: Operand) {
    if !state.regs.status_check(Status::Carry) {
        return;
    }

    state.regs.pc = math::byte_addr_offset(state.regs.pc, operand.read(state));
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.pc = 0x10;
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::Immediate(2));
    assert_eq!(state.regs.pc, 0x12);

    let mut state = State::new();
    state.regs.pc = 0x10;
    execute(&mut state, Operand::Immediate(2));
    assert_eq!(state.regs.pc, 0x10);

    let mut state = State::new();
    state.regs.pc = 0x10;
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::Immediate(0xFE));
    assert_eq!(state.regs.pc, 0xE);
}
