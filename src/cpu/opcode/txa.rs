use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, _operand: Operand) {
    state.regs.a = state.regs.x;
    state.regs.status_set_zn(state.regs.a);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.x = 1;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.a, 1);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.x = 0;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.regs.x = 0xFF;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.a, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());
}
