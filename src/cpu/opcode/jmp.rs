use super::super::operand::Operand;
use super::super::state::State;

pub fn execute(state: &mut State, operand: Operand) {
    state.regs.pc = operand.address();
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.pc = 0x10;
    execute(&mut state, Operand::Memory(0x20));
    assert_eq!(state.regs.pc, 0x20);
}
