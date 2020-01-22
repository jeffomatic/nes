use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let res = state.regs.a & operand.read(state);
    state.regs.a = res;
    state.regs.status_set_zn(res);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.a = 1;
    execute(&mut state, Operand::Immediate(0));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.regs.a = 0;
    execute(&mut state, Operand::Immediate(1));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());

    let mut state = State::new();
    state.regs.a = 0x11;
    execute(&mut state, Operand::Immediate(0x10));
    assert_eq!(state.regs.a, 0x10);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.a = 0x81;
    execute(&mut state, Operand::Immediate(0x80));
    assert_eq!(state.regs.a, 0x80);
    assert_eq!(state.regs.p, Status::Negative.mask());
}
