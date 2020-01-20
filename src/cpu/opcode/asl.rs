use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let prev = operand.read(state);
    let res = prev << 1;
    operand.write(state, res);
    state.regs.p = Status::with_zero_negative(state.regs.p, res);
    state.regs.p = Status::Carry.set_into(state.regs.p, prev & 0b1000_0000 != 0);
}

#[test]
fn test() {
    let mut state = State::new();
    execute(&mut state, Operand::Accumulator);
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.regs.a = 1;
    execute(&mut state, Operand::Accumulator);
    assert_eq!(state.regs.a, 0b10);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.a = 0b1000_0001;
    execute(&mut state, Operand::Accumulator);
    assert_eq!(state.regs.a, 0b10);
    assert_eq!(state.regs.p, Status::Carry.mask());

    let mut state = State::new();
    state.regs.a = 0b1100_0000;
    execute(&mut state, Operand::Accumulator);
    assert_eq!(state.regs.a, 0b1000_0000);
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Negative.mask());

    let mut state = State::new();
    state.mem.write(0x10, 1);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.mem.read(0x10), 0b10);
    assert_eq!(state.regs.p, 0);
}
