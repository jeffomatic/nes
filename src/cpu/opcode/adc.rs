use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;
use crate::math;

pub fn execute(state: &mut State, operand: Operand) {
    let prev = state.regs.a;
    let opval = operand.read(state);
    let res = prev.wrapping_add(opval);
    let negative = math::is_negative(res);
    state.regs.a = res;
    state.regs.p = Status::with_zero_negative(state.regs.p, res);
    state.regs.p = Status::Carry.set_into(state.regs.p, res < prev);
    state.regs.p = Status::Overflow.set_into(
        state.regs.p,
        !math::is_negative(prev) && !math::is_negative(opval) && negative,
    );
}

#[test]
fn test() {
    let mut state = State::new();
    execute(&mut state, Operand::Immediate(0));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    execute(&mut state, Operand::Immediate(1));
    assert_eq!(state.regs.a, 1);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    execute(&mut state, Operand::Immediate(0xFF));
    assert_eq!(state.regs.a, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());

    let mut state = State::new();
    state.regs.a = 0xFF;
    execute(&mut state, Operand::Immediate(1));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Zero.mask());

    let mut state = State::new();
    state.regs.a = 0x7F;
    execute(&mut state, Operand::Immediate(0x7F));
    assert_eq!(state.regs.a, 0xFE);
    assert_eq!(
        state.regs.p,
        Status::Overflow.mask() | Status::Negative.mask()
    );
}
