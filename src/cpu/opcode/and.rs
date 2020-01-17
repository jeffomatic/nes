use crate::cpu::registers::Registers;
use crate::cpu::status::Status;
use crate::cpu::update::Update;
use crate::math;

fn update(registers: &Registers, operand: u8) -> Vec<Update> {
    let res = registers.a & operand;
    vec![
        Update::Accumulator(res),
        Update::Status(Status::Zero, res == 0),
        Update::Status(Status::Negative, math::is_negative(res)),
    ]
}

#[test]
fn test() {
    struct Case {
        operand: u8,
        a: u8,
        want: Vec<Update>,
    }

    for (i, c) in [
        Case {
            operand: 0,
            a: 1,
            want: vec![
                Update::Accumulator(0),
                Update::Status(Status::Zero, true),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0,
            a: 1,
            want: vec![
                Update::Accumulator(0),
                Update::Status(Status::Zero, true),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0x10,
            a: 0x11,
            want: vec![
                Update::Accumulator(0x10),
                Update::Status(Status::Zero, false),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0x80,
            a: 0x81,
            want: vec![
                Update::Accumulator(0x80),
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
        let got = update(&registers, c.operand);
        assert_eq!(got, c.want, "case {}", i);
    }
}
