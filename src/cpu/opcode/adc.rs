use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;
use crate::math;

pub fn execute(state: &mut State, operand: Operand) {
    let prev = state.regs.a;
    let opval = operand.read(state);
    let res16 = prev as u16
        + opval as u16
        + if Status::Carry.check(state.regs.p) {
            1
        } else {
            0
        };
    let res = res16 as u8;

    state.regs.a = res;
    state.regs.p = Status::with_zero_negative(state.regs.p, res);
    state.regs.p = Status::Carry.set_into(state.regs.p, res16 & 0x100 != 0);
    state.regs.p = Status::Overflow.set_into(
        state.regs.p,
        // Overflow occurs in the following cases:
        // positive + positive = negative
        // negative + negative = positive
        math::same_sign(prev, opval) && !math::same_sign(prev, res),
    );
}

#[test]
fn test() {
    // no-mask operation
    let mut state = State::new();
    execute(&mut state, Operand::Immediate(1));
    assert_eq!(state.regs.a, 1);
    assert_eq!(state.regs.p, 0);

    // incorporates carry
    let mut state = State::new();
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::Immediate(1));
    assert_eq!(state.regs.a, 2);
    assert_eq!(state.regs.p, 0);

    // sets zero mask
    let mut state = State::new();
    execute(&mut state, Operand::Immediate(0));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    // sets negative mask
    let mut state = State::new();
    execute(&mut state, Operand::Immediate(0xFF));
    assert_eq!(state.regs.a, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());

    // sets carry
    let mut state = State::new();
    state.regs.a = 0xFF;
    execute(&mut state, Operand::Immediate(1));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // incorporates carry to trigger carry
    let mut state = State::new();
    state.regs.p = Status::Carry.mask();
    state.regs.a = 0;
    execute(&mut state, Operand::Immediate(0xFF));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Zero.mask());

    // positive overflow
    let mut state = State::new();
    state.regs.a = 0x7F;
    execute(&mut state, Operand::Immediate(0x7F));
    assert_eq!(state.regs.a, 0xFE);
    assert_eq!(
        state.regs.p,
        Status::Overflow.mask() | Status::Negative.mask()
    );

    // negative overflow
    let mut state = State::new();
    state.regs.a = 0x80;
    execute(&mut state, Operand::Immediate(0x80));
    assert_eq!(state.regs.a, 0);
    assert_eq!(
        state.regs.p,
        Status::Carry.mask() | Status::Zero.mask() | Status::Overflow.mask()
    );

    // carry does not trigger overflow
    let mut state = State::new();
    state.regs.p = Status::Carry.mask();
    state.regs.a = 0x7F;
    execute(&mut state, Operand::Immediate(0x80));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Carry.mask() | Status::Zero.mask());
}
