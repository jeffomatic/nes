mod decode;
mod opcode;
mod operand;
mod state;
mod status;

fn next(state: &mut state::State) {
    let (opcode, operand) = decode::decode(state).unwrap();
    opcode.execute(state, operand);
}

#[test]
fn test() {
    let mut state = state::State::new();
    state.memwrite(0, 0x69); // adc #$01
    state.memwrite(1, 0x01);
    state.memwrite(2, 0x69); // adc #$FF
    state.memwrite(3, 0xFF);
    state.memwrite(4, 0x69); // adc #$FF
    state.memwrite(5, 0xFF);

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
