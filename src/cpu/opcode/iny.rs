use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, _operand: Operand) {
    let res = state.regs.y.wrapping_add(1);
    state.regs.y = res;
    state.regs.status_set_zn(res);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.y = 0xFE;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.y, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());

    let mut state = State::new();
    state.regs.y = 0xFF;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.y, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.regs.y = 0;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.y, 1);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.y = 1;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.y, 2);
    assert_eq!(state.regs.p, 0);
}
