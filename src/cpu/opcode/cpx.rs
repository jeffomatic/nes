use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let opval = operand.read(state);
    state.regs.p = Status::Carry.set_into(state.regs.p, state.regs.x >= opval);
    state.regs.p = Status::with_zero_negative(state.regs.p, state.regs.x.wrapping_sub(opval));
}

#[test]
fn test() {
    // See cmp implementation for notes

    // X < M, N = 0
    let mut state = State::new();
    state.regs.x = 3;
    state.memwrite(0x10, 4);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Negative.mask());

    // X < M, N = 0
    let mut state = State::new();
    state.regs.x = 2;
    state.memwrite(0x10, 0xFF);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, 0);

    // X = M
    let mut state = State::new();
    state.regs.x = 3;
    state.memwrite(0x10, 3);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // X > M, N = 0
    let mut state = State::new();
    state.regs.x = 2;
    state.memwrite(0x10, 1);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Carry.mask());

    // X > M, N = 1
    let mut state = State::new();
    state.regs.x = 0xFF;
    state.memwrite(0xFE, 1);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Negative.mask());
}
