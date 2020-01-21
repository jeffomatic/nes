use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;
use crate::math;

pub fn execute(state: &mut State, operand: Operand) {
    let prev = state.regs.a;
    let opval = operand.read(state);
    let complement = !opval as u16
        + if Status::Carry.check(state.regs.p) {
            1
        } else {
            0
        };
    let res16 = prev as u16 + complement;
    let res = res16 as u8;

    state.regs.a = res;
    state.regs.p = Status::with_zero_negative(state.regs.p, res);
    state.regs.p = Status::Carry.set_into(state.regs.p, res16 & 0x100 != 0);
    state.regs.p = Status::Overflow.set_into(
        state.regs.p,
        // Overflow occurs in the following cases:
        // positive - negative = negative
        // negative - positive = positive
        !math::same_sign(prev, opval) && math::same_sign(opval, res),
    );
}

#[test]
fn test() {
    // no-borrow subtraction, positive result
    let mut state = State::new();
    state.regs.a = 5;
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::Immediate(3));
    assert_eq!(state.regs.a, 2);
    assert_eq!(state.regs.p, Status::Carry.mask());

    // no-borrow subtraction, zero result
    let mut state = State::new();
    state.regs.a = 5;
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::Immediate(5));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // no-borrow subtraction, negative result
    let mut state = State::new();
    state.regs.a = 5;
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::Immediate(6));
    assert_eq!(state.regs.a, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());

    // no-borrow subtraction, positive result with overflow
    let mut state = State::new();
    state.regs.a = 0x7F;
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::Immediate(0xFF));
    assert_eq!(state.regs.a, 0x80);
    assert_eq!(
        state.regs.p,
        Status::Overflow.mask() | Status::Negative.mask()
    );

    // no-borrow subtraction, negative result with overflow
    let mut state = State::new();
    state.regs.a = 0x80;
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::Immediate(1));
    assert_eq!(state.regs.a, 0x7F);
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Overflow.mask());

    // borrow subtraction, positive result
    let mut state = State::new();
    state.regs.a = 5;
    execute(&mut state, Operand::Immediate(3));
    assert_eq!(state.regs.a, 1);
    assert_eq!(state.regs.p, Status::Carry.mask());

    // borrow subtraction, zero result
    let mut state = State::new();
    state.regs.a = 5;
    execute(&mut state, Operand::Immediate(4));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // borrow subtraction, negative result
    let mut state = State::new();
    state.regs.a = 5;
    execute(&mut state, Operand::Immediate(5));
    assert_eq!(state.regs.a, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());
}
