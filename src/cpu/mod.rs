use self::opcode::Opcode;

mod address_mode;
mod opcode;
mod operand;
mod registers;
mod state;
mod status;

fn next(state: &mut state::State) {
    let (opcode, addr_mode) = opcode::decode(state.mem.read(state.regs.pc)).unwrap();
    let updates = match opcode {
        Opcode::Adc => opcode::adc::update(
            &state,
            operand::fetch_byte(&state.regs, &state.mem, addr_mode),
        ),
        _ => unimplemented!(),
        // Opcode::And => (),
        // Opcode::Asl => (),
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
        // Opcode::Lsr => (),
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

    for u in updates {
        u.apply(state);
    }

    state.regs.pc += 1 + addr_mode.operand_offset();
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
