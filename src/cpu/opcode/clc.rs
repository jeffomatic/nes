use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, _operand: Operand) {
    state.regs.p = Status::Carry.set_into(state.regs.p, false);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.p, 0);
}
