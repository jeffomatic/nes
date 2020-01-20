use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, _operand: Operand) {
    state.regs.p = Status::InterruptDisable.set_into(state.regs.p, true);
}

#[test]
fn test() {
    let mut state = State::new();
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.p, Status::InterruptDisable.mask());
}
