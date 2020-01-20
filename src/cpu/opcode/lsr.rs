use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let prev = operand.read(state);
    let res = prev >> 1;
    operand.write(state, res);
    state.regs.p = Status::with_zero_negative(state.regs.p, res);
    state.regs.p = Status::Carry.set_into(state.regs.p, prev & 1 != 0);
}

#[test]
fn test() {
    let mut state = State::new();
    execute(&mut state, Operand::Accumulator);
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.regs.a = 0b1000_0000;
    execute(&mut state, Operand::Accumulator);
    assert_eq!(state.regs.a, 0b0100_0000);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.a = 0b11;
    execute(&mut state, Operand::Accumulator);
    assert_eq!(state.regs.a, 1);
    assert_eq!(state.regs.p, Status::Carry.mask());

    let mut state = State::new();
    state.memwrite(0x10, 0b10);
    execute(&mut state, Operand::Memory(0x10));
    assert_eq!(state.memread(0x10), 1);
    assert_eq!(state.regs.p, 0);
}
