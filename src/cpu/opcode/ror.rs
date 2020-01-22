use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let prev = operand.read(state);
    let res = prev >> 1
        | if state.regs.status_check(Status::Carry) {
            0b1000_0000
        } else {
            0
        };
    operand.write(state, res);
    state.regs.status_set_zn(res);
    state.regs.status_set(Status::Carry, prev & 1 != 0);
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

    // ensure that carry is transferred
    let mut state = State::new();
    state.regs.a = 0b1000_0000;
    state.regs.p = Status::Carry.mask();
    execute(&mut state, Operand::Accumulator);
    assert_eq!(state.regs.a, 0b1100_0000);
    assert_eq!(state.regs.p, Status::Negative.mask());
}
