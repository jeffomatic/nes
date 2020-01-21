use super::super::operand::Operand;
use super::super::state::State;

pub fn execute(state: &mut State, _operand: Operand) {
    state.regs.pc = state.stack_pop16() + 1;
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.pc = 0x300;
    state.stack_push(0x69); // this is just a sentinel for the test
    state.stack_push16(0x201);
    execute(&mut state, Operand::None);
    assert_eq!(state.stack_peek(0), 0x69);
    assert_eq!(state.regs.pc, 0x202);
}
