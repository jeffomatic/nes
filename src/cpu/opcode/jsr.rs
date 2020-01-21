use super::super::operand::Operand;
use super::super::state::State;

pub fn execute(state: &mut State, operand: Operand) {
    state.stack_push16(state.regs.pc - 1);
    state.regs.pc = operand.address();
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.pc = 0x202;
    execute(&mut state, Operand::Memory(0x300));
    assert_eq!(state.stack_peek16(0), 0x201);
    assert_eq!(state.regs.pc, 0x300);
}
