use super::super::operand::Operand;
use super::super::state::State;

pub fn execute(state: &mut State, _operand: Operand) {
    state.regs.s = state.regs.x;
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.x = 1;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.s, 1);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.x = 0;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.s, 0);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.x = 0xFF;
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.s, 0xFF);
    assert_eq!(state.regs.p, 0);
}
