use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let res = state.regs.a ^ operand.read(state);
    state.regs.a = res;
    state.regs.p = Status::with_zero_negative(state.regs.p, res);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.a = 0;
    execute(&mut state, Operand::Immediate(0xFF));
    assert_eq!(state.regs.a, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());

    let mut state = State::new();
    state.regs.a = 0b1111;
    execute(&mut state, Operand::Immediate(0b1111_1111));
    assert_eq!(state.regs.a, 0b1111_0000);
    assert_eq!(state.regs.p, Status::Negative.mask());

    let mut state = State::new();
    state.regs.a = 0b1111_0000;
    execute(&mut state, Operand::Immediate(0b1111_1111));
    assert_eq!(state.regs.a, 0b0000_1111);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.a = 0xFF;
    execute(&mut state, Operand::Immediate(0xFF));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());
}
