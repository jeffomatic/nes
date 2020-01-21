use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, _operand: Operand) {
    state.stack_push16(state.regs.pc);
    state.stack_push(state.regs.p);
    state.regs.p = Status::BreakCommand.set_into(state.regs.p, true);
    state.regs.pc = state.memread16(0xFFFE);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.pc = 0x201;
    state.regs.p = 0b1000_0001;
    state.memwrite(0xFFFE, 0xFF);
    state.memwrite(0xFFFF, 0x02);
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.pc, 0x2FF);
    assert_eq!(state.regs.p, 0b1001_0001);
    assert_eq!(state.stack_peek(0), 0b1000_0001);
    assert_eq!(state.stack_peek16(1), 0x201);
}
