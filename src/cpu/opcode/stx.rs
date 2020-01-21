use crate::cpu::operand::Operand;
use crate::cpu::state::State;

pub fn execute(state: &mut State, operand: Operand) {
    operand.write(state, state.regs.x);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.x = 1;
    state.memwrite(0x200, 0x69);
    execute(&mut state, Operand::Memory(0x200));
    assert_eq!(state.memread(0x200), 1);
}