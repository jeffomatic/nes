use crate::cpu::state::{self, State};
use crate::cpu::status::Status;
use crate::math;

pub fn update(state: &State, operand: u8) -> Vec<state::Update> {
    let prev = state.regs.a;
    let res = prev.wrapping_add(operand);
    let negative = math::is_negative(res);
    vec![
        state::Update::Accumulator(res),
        state::Update::Status(Status::Carry, res < prev),
        state::Update::Status(Status::Zero, res == 0),
        state::Update::Status(
            Status::Overflow,
            !math::is_negative(prev) && !math::is_negative(operand) && negative,
        ),
        state::Update::Status(Status::Negative, negative),
    ]
}

#[test]
fn test() {
    struct Case {
        operand: u8,
        a: u8,
        want: Vec<state::Update>,
    }

    for (i, c) in [
        Case {
            operand: 0,
            a: 0,
            want: vec![
                state::Update::Accumulator(0),
                state::Update::Status(Status::Carry, false),
                state::Update::Status(Status::Zero, true),
                state::Update::Status(Status::Overflow, false),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0,
            a: 1,
            want: vec![
                state::Update::Accumulator(1),
                state::Update::Status(Status::Carry, false),
                state::Update::Status(Status::Zero, false),
                state::Update::Status(Status::Overflow, false),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 1,
            a: 0,
            want: vec![
                state::Update::Accumulator(1),
                state::Update::Status(Status::Carry, false),
                state::Update::Status(Status::Zero, false),
                state::Update::Status(Status::Overflow, false),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0,
            a: 0xFF,
            want: vec![
                state::Update::Accumulator(0xFF),
                state::Update::Status(Status::Carry, false),
                state::Update::Status(Status::Zero, false),
                state::Update::Status(Status::Overflow, false),
                state::Update::Status(Status::Negative, true),
            ],
        },
        Case {
            operand: 1,
            a: 0xFF,
            want: vec![
                state::Update::Accumulator(0),
                state::Update::Status(Status::Carry, true),
                state::Update::Status(Status::Zero, true),
                state::Update::Status(Status::Overflow, false),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0x7F,
            a: 0x7F,
            want: vec![
                state::Update::Accumulator(0xFE),
                state::Update::Status(Status::Carry, false),
                state::Update::Status(Status::Zero, false),
                state::Update::Status(Status::Overflow, true),
                state::Update::Status(Status::Negative, true),
            ],
        },
    ]
    .iter()
    .enumerate()
    {
        let mut state = State::new();
        state.regs.a = c.a;
        let got = update(&state, c.operand);
        assert_eq!(got, c.want, "case {}", i);
    }
}
