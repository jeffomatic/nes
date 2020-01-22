use crate::cpu::operand::Operand;
use crate::cpu::state::State;
use crate::cpu::status::Status;

pub fn execute(state: &mut State, operand: Operand) {
    let res = operand.read(state);
    state.regs.a = res;
    state.regs.status_set_zn(res);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.a = 0x69;
    state.mem_write(0x200, 1);
    execute(&mut state, Operand::Memory(0x200));
    assert_eq!(state.regs.a, 1);
    assert_eq!(state.regs.p, 0);

    let mut state = State::new();
    state.regs.a = 0x69;
    state.mem_write(0x200, 0xFF);
    execute(&mut state, Operand::Memory(0x200));
    assert_eq!(state.regs.a, 0xFF);
    assert_eq!(state.regs.p, Status::Negative.mask());

    let mut state = State::new();
    state.regs.a = 0x69;
    state.mem_write(0x200, 0);
    execute(&mut state, Operand::Memory(0x200));
    assert_eq!(state.regs.a, 0);
    assert_eq!(state.regs.p, Status::Zero.mask());
}
