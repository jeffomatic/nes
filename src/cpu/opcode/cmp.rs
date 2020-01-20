use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let opval = operand.read(state);
    state.regs.p = Status::Carry.set_into(state.regs.p, state.regs.a >= opval);
    state.regs.p = Status::with_zero_negative(state.regs.p, state.regs.a.wrapping_sub(opval));
}

#[test]
fn test() {
    // Reference for flag states:
    // http://users.telenet.be/kim1-6502/6502/proman.html#421
    //
    // Carry is set if the A >= M unsigned comparison is true
    // Zero is set if A - M is zero
    // Negative is set if A - M is negative

    // A < M, N = 0
    let mut state = State::new();
    state.regs.a = 3;
    state.memwrite(0x10, 4);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Negative.mask());

    // A < M, N = 0
    let mut state = State::new();
    state.regs.a = 2;
    state.memwrite(0x10, 0xFF);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, 0);

    // A = M
    let mut state = State::new();
    state.regs.a = 3;
    state.memwrite(0x10, 3);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // A > M, N = 0
    let mut state = State::new();
    state.regs.a = 2;
    state.memwrite(0x10, 1);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Carry.mask());

    // A > M, N = 1
    let mut state = State::new();
    state.regs.a = 0xFF;
    state.memwrite(0xFE, 1);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Negative.mask());
}
