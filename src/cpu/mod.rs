mod decode;
mod opcode;
mod operand;
mod registers;
mod state;
mod status;

fn next(state: &mut state::State) {
    let (opcode, operand) = decode::decode(state).unwrap();
    opcode.execute(state, operand);
}

#[test]
fn test() {
    let mut state = state::State::new();
    state.mem.write(0, 0x69); // adc #$01
    state.mem.write(1, 0x01);
    state.mem.write(2, 0x69); // adc #$FF
    state.mem.write(3, 0xFF);
    state.mem.write(4, 0x69); // adc #$FF
    state.mem.write(5, 0xFF);

    next(&mut state);
    assert_eq!(state.regs.pc, 2);
    assert_eq!(state.regs.a, 1);
    assert_eq!(state.regs.p, 0);

    next(&mut state);
    assert_eq!(state.regs.pc, 4);
    assert_eq!(state.regs.a, 0);
    assert_eq!(
        state.regs.p,
        status::Status::Carry.mask() | status::Status::Zero.mask()
    );

    next(&mut state);
    assert_eq!(state.regs.pc, 6);
    assert_eq!(state.regs.a, 0xFF);
    assert_eq!(state.regs.p, status::Status::Negative.mask());
}
