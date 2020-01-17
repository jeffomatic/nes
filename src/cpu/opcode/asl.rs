use crate::cpu::registers::Registers;
use crate::cpu::status::Status;
use crate::cpu::update::Update;
use crate::math;

fn update(registers: &Registers) -> Vec<Update> {
    // TODO: memory update version
    let prev = registers.a;
    let res = prev << 1;
    vec![
        Update::Accumulator(res),
        Update::Status(Status::Carry, prev & 0b1000_0000 != 0),
        Update::Status(Status::Zero, res == 0),
        Update::Status(Status::Negative, math::is_negative(res)),
    ]
}

#[test]
fn test() {
    struct Case {
        a: u8,
        want: Vec<Update>,
    }

    for (i, c) in [
        Case {
            a: 0,
            want: vec![
                Update::Accumulator(0),
                Update::Status(Status::Carry, false),
                Update::Status(Status::Zero, true),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            a: 0b1,
            want: vec![
                Update::Accumulator(0b10),
                Update::Status(Status::Carry, false),
                Update::Status(Status::Zero, false),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            a: 0b1000_0001,
            want: vec![
                Update::Accumulator(0b0000_0010),
                Update::Status(Status::Carry, true),
                Update::Status(Status::Zero, false),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            a: 0b1100_0000,
            want: vec![
                Update::Accumulator(0b1000_0000),
                Update::Status(Status::Carry, true),
                Update::Status(Status::Zero, false),
                Update::Status(Status::Negative, true),
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
