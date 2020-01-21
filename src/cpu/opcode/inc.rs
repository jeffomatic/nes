use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let prev = operand.read(state);
    let res = prev.wrapping_add(1);
    operand.write(state, res);
    state.regs.p = Status::with_zero_negative(state.regs.p, res);
}

#[test]
fn test() {
    let mut state = State::new();
    state.memwrite(0, 0xFE);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.memread(0), 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());

    let mut state = State::new();
    state.memwrite(0, 0xFF);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.memread(0), 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.memwrite(0, 0);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.memread(0), 1);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.memwrite(0, 1);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.memread(0), 2);
    assert_eq!(state.regs.p, 0);
}