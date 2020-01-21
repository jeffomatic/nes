use crate::cpu::operand::Operand;
use crate::cpu::state::State;

pub fn execute(state: &mut State, _operand: Operand) {
    state.stack_push(state.regs.a);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.a = 1;
    execute(&mut state, Operand::None);
    assert_eq!(state.stack_peek(0), 1);
}
