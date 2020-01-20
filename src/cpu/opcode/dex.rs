use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, _operand: Operand) {
    let res = state.regs.x.wrapping_sub(1);
    state.regs.x = res;
    state.regs.p = Status::with_zero_negative(state.regs.p, res);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.x = 2;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.x, 1);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.x = 1;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.x, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.regs.x = 0;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.x, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());

    let mut state = State::new();
    state.regs.x = 0xFF;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.x, 0xFE);
    assert_eq!(state.regs.p, Status::Negative.mask());
}
