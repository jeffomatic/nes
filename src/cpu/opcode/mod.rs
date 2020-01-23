use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;

mod arithmetic;
mod branch;
mod brk;
mod incdec;
mod jump;
mod loadstore;
mod logic;
mod rti;
mod shift;
mod stack;
mod status;
mod transfer;

// Reference: http://obelisk.me.uk/6502/reference.html
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Adc, // Add with carry
    And, // Bitwise and
    Asl, // Arithmetic shift left
    Bcc, // Branch if carry clear
    Bcs, // Branch if carry set
    Beq, // Brance if equal
    Bit, // Bit test
    Bmi, // Branch if minus
    Bne, // Branch if not equal
    Bpl, // Branch if positive
    Brk, // Force interrupt
    Bvc, // Branch if overflow clear
    Bvs, // Branch if overflow set
    Clc, // Clear carry flag
    Cld, // Clear decimal mode
    Cli, // Clear interrupt disable
    Clv, // Clear overflow flag
    Cmp, // Compare
    Cpx, // Compare X register
    Cpy, // Compare Y register
    Dec, // Decrement memory
    Dex, // Decrement X register
    Dey, // Decrement Y register
    Eor, // Exclusive Or
    Inc, // Increment memory
    Inx, // Increment X register
    Iny, // Increment Y register
    Jmp, // Jump
    Jsr, // Jump to subroutine
    Lda, // Load accumulator
    Ldx, // Load X register
    Ldy, // Load Y register
    Lsr, // Logical shift right
    Nop, // No-op
    Ora, // Logical inclusive or
    Pha, // Push accumulator
    Php, // Push processor status
    Pla, // Pull accumulator
    Plp, // Pull processor status
    Rol, // Rotate left
    Ror, // Rotate right
    Rti, // Return from interrupt
    Rts, // Return from subroutine
    Sbc, // Subtract with carry
    Sec, // Set carry flag
    Sed, // Set decimal flag
    Sei, // Set interrupt disable
    Sta, // Store accumulator
    Stx, // Store X register
    Sty, // Store Y register
    Tax, // Transfer accumulator to X
    Tay, // Transfer accumulator to Y
    Tsx, // Transfer stack pointer to X
    Txa, // Transfer stack pointer to accumulator
    Txs, // Transfer X to stack pointer
    Tya, // Transfer Y to accumulator
}

impl Opcode {
    pub fn execute(self, cpu: &mut Cpu, operand: Operand) {
        match self {
            Opcode::Adc => arithmetic::adc(cpu, operand),
            Opcode::And => logic::and(cpu, operand),
            Opcode::Asl => shift::asl(cpu, operand),
            Opcode::Bcc => branch::bcc(cpu, operand),
            Opcode::Bcs => branch::bcs(cpu, operand),
            Opcode::Beq => branch::beq(cpu, operand),
            Opcode::Bit => logic::bit(cpu, operand),
            Opcode::Bmi => branch::bmi(cpu, operand),
            Opcode::Bne => branch::bne(cpu, operand),
            Opcode::Bpl => branch::bpl(cpu, operand),
            Opcode::Brk => brk::execute(cpu, operand),
            Opcode::Bvc => branch::bvc(cpu, operand),
            Opcode::Bvs => branch::bvs(cpu, operand),
            Opcode::Clc => status::clc(cpu, operand),
            Opcode::Cld => status::cld(cpu, operand),
            Opcode::Cli => status::cli(cpu, operand),
            Opcode::Clv => status::clv(cpu, operand),
            Opcode::Cmp => arithmetic::cmp(cpu, operand),
            Opcode::Cpx => arithmetic::cpx(cpu, operand),
            Opcode::Cpy => arithmetic::cpy(cpu, operand),
            Opcode::Dec => incdec::dec(cpu, operand),
            Opcode::Dex => incdec::dex(cpu, operand),
            Opcode::Dey => incdec::dey(cpu, operand),
            Opcode::Eor => logic::eor(cpu, operand),
            Opcode::Inc => incdec::inc(cpu, operand),
            Opcode::Inx => incdec::inx(cpu, operand),
            Opcode::Iny => incdec::iny(cpu, operand),
            Opcode::Jmp => jump::jmp(cpu, operand),
            Opcode::Jsr => jump::jsr(cpu, operand),
            Opcode::Lda => loadstore::lda(cpu, operand),
            Opcode::Ldx => loadstore::ldx(cpu, operand),
            Opcode::Ldy => loadstore::ldy(cpu, operand),
            Opcode::Lsr => shift::lsr(cpu, operand),
            Opcode::Nop => (),
            Opcode::Ora => logic::ora(cpu, operand),
            Opcode::Pha => stack::pha(cpu, operand),
            Opcode::Php => stack::php(cpu, operand),
            Opcode::Pla => stack::pla(cpu, operand),
            Opcode::Plp => stack::plp(cpu, operand),
            Opcode::Rol => shift::rol(cpu, operand),
            Opcode::Ror => shift::ror(cpu, operand),
            Opcode::Rti => rti::execute(cpu, operand),
            Opcode::Rts => jump::rts(cpu, operand),
            Opcode::Sbc => arithmetic::sbc(cpu, operand),
            Opcode::Sec => status::sec(cpu, operand),
            Opcode::Sed => status::sed(cpu, operand),
            Opcode::Sei => status::sei(cpu, operand),
            Opcode::Sta => loadstore::sta(cpu, operand),
            Opcode::Stx => loadstore::stx(cpu, operand),
            Opcode::Sty => loadstore::sty(cpu, operand),
            Opcode::Tax => transfer::tax(cpu, operand),
            Opcode::Tay => transfer::tay(cpu, operand),
            Opcode::Tsx => stack::tsx(cpu, operand),
            Opcode::Txa => transfer::txa(cpu, operand),
            Opcode::Txs => stack::txs(cpu, operand),
            Opcode::Tya => transfer::tya(cpu, operand),
        }
    }

    pub fn writes_memory(self) -> bool {
        match self {
            Self::Asl | Self::Dec | Self::Inc | Self::Rol | Self::Ror | Self::Sta => true,
            _ => false,
        }
    }
}
