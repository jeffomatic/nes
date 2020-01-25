mod address_mode;
mod instruction;
mod opcode;
mod operand;
mod state;
mod status;

fn next(cpu: &mut state::Cpu) {
    let (opcode_type, operand, _cycles) = instruction::decode(cpu).unwrap();
    instruction::execute(opcode_type, cpu, operand);
}

#[test]
fn test() {
    let mut cpu = state::Cpu::new();
    cpu.mem_write(0, 0x69); // adc #$01
    cpu.mem_write(1, 0x01);
    cpu.mem_write(2, 0x69); // adc #$FF
    cpu.mem_write(3, 0xFF);
    cpu.mem_write(4, 0x69); // adc #$FF
    cpu.mem_write(5, 0xFF);

    next(&mut cpu);
    assert_eq!(cpu.regs.pc, 2);
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);

    next(&mut cpu);
    assert_eq!(cpu.regs.pc, 4);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(
        cpu.regs.p,
        status::Status::Carry.mask() | status::Status::Zero.mask()
    );

    next(&mut cpu);
    assert_eq!(cpu.regs.pc, 6);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(
        cpu.regs.p,
        status::Status::Carry.mask() | status::Status::Zero.mask()
    );
}
