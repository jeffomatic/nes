use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let opval = operand.read(state);
    state.regs.status_set(Status::Carry, state.regs.y >= opval);
    state.regs.status_set_zn(state.regs.y.wrapping_sub(opval));
}

#[test]
fn test() {
    // See cmp implementation for notes

    // Y < M, N = 0
    let mut state = State::new();
    state.regs.y = 3;
    state.mem_write(0x10, 4);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Negative.mask());

    // Y < M, N = 0
    let mut state = State::new();
    state.regs.y = 2;
    state.mem_write(0x10, 0xFF);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, 0);

    // Y = M
    let mut state = State::new();
    state.regs.y = 3;
    state.mem_write(0x10, 3);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // Y > M, N = 0
    let mut state = State::new();
    state.regs.y = 2;
    state.mem_write(0x10, 1);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Carry.mask());

    // Y > M, N = 1
    let mut state = State::new();
    state.regs.y = 0xFF;
    state.mem_write(0xFE, 1);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Negative.mask());
}
