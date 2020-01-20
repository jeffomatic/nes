use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let v = operand.read(state);
    state.regs.p = Status::Zero.set_into(state.regs.p, state.regs.a & v == 0);
    state.regs.p = Status::Overflow.set_into(state.regs.p, v & 0b0100_0000 != 0);
    state.regs.p = Status::Negative.set_into(state.regs.p, v & 0b1000_0000 != 0);
}

#[test]
fn test() {
    let mut state = State::new();
    state.mem.write(0, 0);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.regs.a = 0xFF;
    state.mem.write(0, 1);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.mem.write(0, 0b0100_0000);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.regs.p, Status::Zero.mask() | Status::Overflow.mask());

    let mut state = State::new();
    state.regs.a = 0xFF;
    state.mem.write(0, 0b0100_0000);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.regs.p, Status::Overflow.mask());

    let mut state = State::new();
    state.mem.write(0, 0b1000_0000);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.regs.p, Status::Zero.mask() | Status::Negative.mask());

    let mut state = State::new();
    state.regs.a = 0xFF;
    state.mem.write(0, 0b1000_0000);
    execute(&mut state, Operand::Memory(0));
    assert_eq!(state.regs.p, Status::Negative.mask());
}
