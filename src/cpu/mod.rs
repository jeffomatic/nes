mod address_mode;
mod assemble;
mod instruction;
mod opcode;
mod operand;
mod state;
mod status;

impl state::Cpu {
    pub fn step(&mut self) {
        let (opcode_type, addr_mode, base_cost) =
            opcode::decode(self.instruction_fetch_byte()).unwrap();
        let (operand, operand_cost) = operand::decode(self, opcode_type, addr_mode);
        instruction::execute(opcode_type, self, operand);
        self.cycle_add(base_cost + operand_cost);
    }
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

    cpu.step();
    assert_eq!(cpu.regs.pc, 2);
    assert_eq!(cpu.regs.a, 1);
    assert_eq!(cpu.regs.p, 0);
    assert_eq!(cpu.cycles, 2);

    cpu.step();
    assert_eq!(cpu.regs.pc, 4);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(
        cpu.regs.p,
        status::Status::Carry.mask() | status::Status::Zero.mask()
    );
    assert_eq!(cpu.cycles, 4);

    cpu.step();
    assert_eq!(cpu.regs.pc, 6);
    assert_eq!(cpu.regs.a, 0);
    assert_eq!(
        cpu.regs.p,
        status::Status::Carry.mask() | status::Status::Zero.mask()
    );
    assert_eq!(cpu.cycles, 6);
}

#[test]
fn fibonacci_test() {
    let asm = "
define target $0400
define temp0 $0500
define temp1 $0501
define temp2 $0502
lda #$00
sta temp0
lda #$01
sta temp1
ldx #$00
loop: lda temp1
sta target,x
sta temp2
adc temp0
sta temp1
lda temp2
sta temp0
inx
cpx #$0a
bmi loop
brk
    ";

    let mut cpu = state::Cpu::new();
    cpu.mem_write_buf(0, assemble::assemble(asm, 0).unwrap());

    while !cpu.regs.status_check(status::Status::BreakCommand) {
        cpu.step();
    }

    assert_eq!(
        cpu.mem_read_buf(0x400, 10),
        vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
    );
}
