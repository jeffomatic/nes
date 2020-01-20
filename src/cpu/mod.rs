use self::opcode::Opcode;

mod decode;
mod opcode;
mod operand;
mod registers;
mod state;
mod status;

fn next(state: &mut state::State) {
    let (opcode, operand) = decode::decode(state).unwrap();
    match opcode {
        Opcode::Adc => opcode::adc::execute(state, operand),
        Opcode::And => opcode::and::execute(state, operand),
        Opcode::Asl => opcode::asl::execute(state, operand),
        Opcode::Lsr => opcode::lsr::execute(state, operand),
        _ => unimplemented!(),
        // Opcode::Bcc => (),
        // Opcode::Bcs => (),
        // Opcode::Beq => (),
        // Opcode::Bit => (),
        // Opcode::Bmi => (),
        // Opcode::Bne => (),
        // Opcode::Bpl => (),
        // Opcode::Brk => (),
        // Opcode::Bvc => (),
        // Opcode::Bvs => (),
        // Opcode::Clc => (),
        // Opcode::Cld => (),
        // Opcode::Cli => (),
        // Opcode::Clv => (),
        // Opcode::Cmp => (),
        // Opcode::Cpx => (),
        // Opcode::Cpy => (),
        // Opcode::Dec => (),
        // Opcode::Dex => (),
        // Opcode::Dey => (),
        // Opcode::Eor => (),
        // Opcode::Inc => (),
        // Opcode::Inx => (),
        // Opcode::Iny => (),
        // Opcode::Jmp => (),
        // Opcode::Jsr => (),
        // Opcode::Lda => (),
        // Opcode::Ldx => (),
        // Opcode::Ldy => (),
        // Opcode::Nop => (),
        // Opcode::Ora => (),
        // Opcode::Pha => (),
        // Opcode::Php => (),
        // Opcode::Pla => (),
        // Opcode::Plp => (),
        // Opcode::Rol => (),
        // Opcode::Ror => (),
        // Opcode::Rti => (),
        // Opcode::Rts => (),
        // Opcode::Sbc => (),
        // Opcode::Sec => (),
        // Opcode::Sed => (),
        // Opcode::Sei => (),
        // Opcode::Sta => (),
        // Opcode::Stx => (),
        // Opcode::Sty => (),
        // Opcode::Tax => (),
        // Opcode::Tay => (),
        // Opcode::Tsx => (),
        // Opcode::Txa => (),
        // Opcode::Txs => (),
        // Opcode::Tya => (),
    };
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
