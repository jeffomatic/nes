use crate::cpu::address_mode::AddressMode;
use crate::cpu::operand;
use crate::cpu::state;
use crate::cpu::status::Status;
use crate::math;

fn update(state: &state::State, addr_mode: AddressMode) -> Vec<state::Update> {
    let prev = operand::fetch_byte(&state.regs, &state.mem, addr_mode);
    let res = prev >> 1;

    let res_update = match addr_mode {
        AddressMode::Accumulator => state::Update::Accumulator(res),
        AddressMode::ZeroPage
        | AddressMode::ZeroPageX
        | AddressMode::Absolute
        | AddressMode::AbsoluteX => state::Update::Memory(
            operand::fetch_address(&state.regs, &state.mem, addr_mode),
            res,
        ),
        other => panic!("address mode {:?} not compatible with asl opcode", other),
    };

    vec![
        res_update,
        state::Update::Status(Status::Carry, prev & 1 != 0),
        state::Update::Status(Status::Zero, res == 0),
        state::Update::Status(Status::Negative, math::is_negative(res)),
    ]
}

#[test]
fn test() {
    let state = state::State::new();
    assert_eq!(
        update(&state, AddressMode::Accumulator),
        vec![
            state::Update::Accumulator(0),
            state::Update::Status(Status::Carry, false),
            state::Update::Status(Status::Zero, true),
            state::Update::Status(Status::Negative, false),
        ]
    );

    let mut state = state::State::new();
    state.regs.a = 0b1000_0000;
    assert_eq!(
        update(&state, AddressMode::Accumulator),
        vec![
            state::Update::Accumulator(0b0100_0000),
            state::Update::Status(Status::Carry, false),
            state::Update::Status(Status::Zero, false),
            state::Update::Status(Status::Negative, false),
        ]
    );

    let mut state = state::State::new();
    state.regs.a = 0b11;
    assert_eq!(
        update(&state, AddressMode::Accumulator),
        vec![
            state::Update::Accumulator(1),
            state::Update::Status(Status::Carry, true),
            state::Update::Status(Status::Zero, false),
            state::Update::Status(Status::Negative, false),
        ]
    );

    let mut state = state::State::new();
    state.mem.write(1, 0x10);
    state.mem.write(0x10, 0b10);
    assert_eq!(
        update(&state, AddressMode::ZeroPage),
        vec![
            state::Update::Memory(0x10, 1),
            state::Update::Status(Status::Carry, false),
            state::Update::Status(Status::Zero, false),
            state::Update::Status(Status::Negative, false),
        ]
    );

    let mut state = state::State::new();
    state.regs.x = 1;
    state.mem.write(1, 0x10);
    state.mem.write(0x11, 0b10);
    assert_eq!(
        update(&state, AddressMode::ZeroPageX),
        vec![
            state::Update::Memory(0x11, 1),
            state::Update::Status(Status::Carry, false),
            state::Update::Status(Status::Zero, false),
            state::Update::Status(Status::Negative, false),
        ]
    );

    let mut state = state::State::new();
    state.mem.write(1, 0x00);
    state.mem.write(2, 2);
    state.mem.write(0x200, 0b10);
    assert_eq!(
        update(&state, AddressMode::Absolute),
        vec![
            state::Update::Memory(0x200, 1),
            state::Update::Status(Status::Carry, false),
            state::Update::Status(Status::Zero, false),
            state::Update::Status(Status::Negative, false),
        ]
    );

    let mut state = state::State::new();
    state.regs.x = 1;
    state.mem.write(1, 0x00);
    state.mem.write(2, 2);
    state.mem.write(0x201, 0b10);
    assert_eq!(
        update(&state, AddressMode::AbsoluteX),
        vec![
            state::Update::Memory(0x201, 1),
            state::Update::Status(Status::Carry, false),
            state::Update::Status(Status::Zero, false),
            state::Update::Status(Status::Negative, false),
        ]
    );
}
