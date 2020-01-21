use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let res = operand.read(state);
    state.regs.y = res;
    state.regs.p = Status::with_zero_negative(state.regs.p, res);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.y = 0x69;
    state.memwrite(0x200, 1);
    execute(&mut state, Operand::Memory(0x200));
    assert_eq!(state.regs.y, 1);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.y = 0x69;
    state.memwrite(0x200, 0xFF);
    execute(&mut state, Operand::Memory(0x200));
    assert_eq!(state.regs.y, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());

    let mut state = State::new();
    state.regs.y = 0x69;
    state.memwrite(0x200, 0);
    execute(&mut state, Operand::Memory(0x200));
    assert_eq!(state.regs.y, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());
}
