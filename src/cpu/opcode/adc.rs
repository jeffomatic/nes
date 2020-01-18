use crate::cpu::registers::Registers;
use crate::cpu::status::Status;
use crate::cpu::update::Update;
use crate::math;

fn update(registers: &Registers, operand: u8) -> Vec<Update> {
    let prev = registers.a;
    let res = prev.wrapping_add(operand);
    let negative = math::is_negative(res);
    vec![
        Update::Accumulator(res),
        Update::Status(Status::Carry, res < prev),
        Update::Status(Status::Zero, res == 0),
        Update::Status(
            Status::Overflow,
            !math::is_negative(prev) && !math::is_negative(operand) && negative,
        ),
        Update::Status(Status::Negative, negative),
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
            a: 0,
            want: vec![
                Update::Accumulator(0),
                Update::Status(Status::Carry, false),
                Update::Status(Status::Zero, true),
                Update::Status(Status::Overflow, false),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0,
            a: 1,
            want: vec![
                Update::Accumulator(1),
                Update::Status(Status::Carry, false),
                Update::Status(Status::Zero, false),
                Update::Status(Status::Overflow, false),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 1,
            a: 0,
            want: vec![
                Update::Accumulator(1),
                Update::Status(Status::Carry, false),
                Update::Status(Status::Zero, false),
                Update::Status(Status::Overflow, false),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0,
            a: 0xFF,
            want: vec![
                Update::Accumulator(0xFF),
                Update::Status(Status::Carry, false),
                Update::Status(Status::Zero, false),
                Update::Status(Status::Overflow, false),
                Update::Status(Status::Negative, true),
            ],
        },
        Case {
            operand: 1,
            a: 0xFF,
            want: vec![
                Update::Accumulator(0),
                Update::Status(Status::Carry, true),
                Update::Status(Status::Zero, true),
                Update::Status(Status::Overflow, false),
                Update::Status(Status::Negative, false),
            ],
        },
        Case {
            operand: 0x7F,
            a: 0x7F,
            want: vec![
                Update::Accumulator(0xFE),
                Update::Status(Status::Carry, false),
                Update::Status(Status::Zero, false),
                Update::Status(Status::Overflow, true),
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
