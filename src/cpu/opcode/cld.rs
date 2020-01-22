use super::super::operand::Operand;
use super::super::state::State;
use super::super::status::Status;

pub fn execute(state: &mut State, _operand: Operand) {
    state.regs.status_set(Status::DecimalMode, false);
}

#[test]
fn test() {
    let mut state = State::new();
    state.regs.p = Status::DecimalMode.mask();
    execute(&mut state, Operand::None);
    assert_eq!(state.regs.p, 0);
}
