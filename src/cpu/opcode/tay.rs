use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, _operand: Operand) {
    state.regs.y = state.regs.a;
    state.regs.p = Status::with_zero_negative(state.regs.p, state.regs.y);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.a = 1;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.y, 1);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.y, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.regs.a = 0xFF;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.y, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());
}
