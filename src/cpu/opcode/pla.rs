use crate::cpu::operand::Operand;
use crate::cpu::state::State;

pub fn execute(state: &mut State, _operand: Operand) {
    state.regs.a = state.stack_pop();
}

#[test]
fn test() {
    let mut state = State::new();
    state.stack_push(1);
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.a, 1);
}
