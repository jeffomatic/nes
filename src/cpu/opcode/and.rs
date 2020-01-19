use crate::cpu::registers::Registers;
use crate::cpu::state;
use crate::cpu::status::Status;
use crate::math;

fn update(registers: &Registers, operand: u8) -> Vec<state::Update> {
    let res = registers.a & operand;
    vec![
        state::Update::Accumulator(res),
        state::Update::Status(Status::Zero, res == 0),
        state::Update::Status(Status::Negative, math::is_negative(res)),
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
            a: 1,
            want: vec![
                state::Update::Accumulator(0),
                state::Update::Status(Status::Zero, true),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0,
            a: 1,
            want: vec![
                state::Update::Accumulator(0),
                state::Update::Status(Status::Zero, true),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0x10,
            a: 0x11,
            want: vec![
                state::Update::Accumulator(0x10),
                state::Update::Status(Status::Zero, false),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0x80,
            a: 0x81,
            want: vec![
                state::Update::Accumulator(0x80),
                state::Update::Status(Status::Zero, false),
                state::Update::Status(Status::Negative, true),
            ],
        },
    ]
    .iter()
    .enumerate()
    {
        let mut registers = Registers::default();
        registers.a = c.a;
        let got = update(&registers, c.operand);
        assert_eq!(got, c.want, "case {}", i);
    }
}
