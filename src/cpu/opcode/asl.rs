use crate::cpu::registers::Registers;
use crate::cpu::state;
use crate::cpu::status::Status;
use crate::math;

fn update(registers: &Registers) -> Vec<state::Update> {
    // TODO: memory update version
    let prev = registers.a;
    let res = prev << 1;
    vec![
        state::Update::Accumulator(res),
        state::Update::Status(Status::Carry, prev & 0b1000_0000 != 0),
        state::Update::Status(Status::Zero, res == 0),
        state::Update::Status(Status::Negative, math::is_negative(res)),
    ]
}

#[test]
fn test() {
    struct Case {
        a: u8,
        want: Vec<state::Update>,
    }

    for (i, c) in [
        Case {
            a: 0,
            want: vec![
                state::Update::Accumulator(0),
                state::Update::Status(Status::Carry, false),
                state::Update::Status(Status::Zero, true),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            a: 0b1,
            want: vec![
                state::Update::Accumulator(0b10),
                state::Update::Status(Status::Carry, false),
                state::Update::Status(Status::Zero, false),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            a: 0b1000_0001,
            want: vec![
                state::Update::Accumulator(0b0000_0010),
                state::Update::Status(Status::Carry, true),
                state::Update::Status(Status::Zero, false),
                state::Update::Status(Status::Negative, false),
            ],
        },
        Case {
            a: 0b1100_0000,
            want: vec![
                state::Update::Accumulator(0b1000_0000),
                state::Update::Status(Status::Carry, true),
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
        let got = update(&registers);
        assert_eq!(got, c.want, "case {}", i);
    }
}
