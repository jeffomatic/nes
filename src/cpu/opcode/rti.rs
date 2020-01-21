use super::super::operand::Operand;
use super::super::state::State;

pub fn execute(state: &mut State, _operand: Operand) {
    state.regs.p = state.stack_pop();
    state.regs.pc = state.stack_pop16();
}

#[test]
fn test() {
    let mut state = State::new();
    state.stack_push(0x69); // sentinel
    state.stack_push16(0x200); // PC
    state.stack_push(1); // status
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.pc, 0x200);
    assert_eq!(state.regs.p, 1);
    assert_eq!(state.stack_peek(0), 0x69);
}
