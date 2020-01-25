use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;

mod arithmetic;
mod branch;
mod incdec;
mod jump;
mod loadstore;
mod logic;
mod shift;
mod stack;
mod status;
mod system;
mod transfer;

// Reference: http://obelisk.me.uk/6502/reference.html
#[derive(Clone, Copy, Debug)]
pub enum Type {
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

impl Type {
    pub fn execute(self, cpu: &mut Cpu, operand: Operand) {
        match self {
            Type::Adc => arithmetic::adc(cpu, operand),
            Type::And => logic::and(cpu, operand),
            Type::Asl => shift::asl(cpu, operand),
            Type::Bcc => branch::bcc(cpu, operand),
            Type::Bcs => branch::bcs(cpu, operand),
            Type::Beq => branch::beq(cpu, operand),
            Type::Bit => logic::bit(cpu, operand),
            Type::Bmi => branch::bmi(cpu, operand),
            Type::Bne => branch::bne(cpu, operand),
            Type::Bpl => branch::bpl(cpu, operand),
            Type::Brk => system::brk(cpu, operand),
            Type::Bvc => branch::bvc(cpu, operand),
            Type::Bvs => branch::bvs(cpu, operand),
            Type::Clc => status::clc(cpu, operand),
            Type::Cld => status::cld(cpu, operand),
            Type::Cli => status::cli(cpu, operand),
            Type::Clv => status::clv(cpu, operand),
            Type::Cmp => arithmetic::cmp(cpu, operand),
            Type::Cpx => arithmetic::cpx(cpu, operand),
            Type::Cpy => arithmetic::cpy(cpu, operand),
            Type::Dec => incdec::dec(cpu, operand),
            Type::Dex => incdec::dex(cpu, operand),
            Type::Dey => incdec::dey(cpu, operand),
            Type::Eor => logic::eor(cpu, operand),
            Type::Inc => incdec::inc(cpu, operand),
            Type::Inx => incdec::inx(cpu, operand),
            Type::Iny => incdec::iny(cpu, operand),
            Type::Jmp => jump::jmp(cpu, operand),
            Type::Jsr => jump::jsr(cpu, operand),
            Type::Lda => loadstore::lda(cpu, operand),
            Type::Ldx => loadstore::ldx(cpu, operand),
            Type::Ldy => loadstore::ldy(cpu, operand),
            Type::Lsr => shift::lsr(cpu, operand),
            Type::Nop => system::nop(cpu, operand),
            Type::Ora => logic::ora(cpu, operand),
            Type::Pha => stack::pha(cpu, operand),
            Type::Php => stack::php(cpu, operand),
            Type::Pla => stack::pla(cpu, operand),
            Type::Plp => stack::plp(cpu, operand),
            Type::Rol => shift::rol(cpu, operand),
            Type::Ror => shift::ror(cpu, operand),
            Type::Rti => system::rti(cpu, operand),
            Type::Rts => jump::rts(cpu, operand),
            Type::Sbc => arithmetic::sbc(cpu, operand),
            Type::Sec => status::sec(cpu, operand),
            Type::Sed => status::sed(cpu, operand),
            Type::Sei => status::sei(cpu, operand),
            Type::Sta => loadstore::sta(cpu, operand),
            Type::Stx => loadstore::stx(cpu, operand),
            Type::Sty => loadstore::sty(cpu, operand),
            Type::Tax => transfer::tax(cpu, operand),
            Type::Tay => transfer::tay(cpu, operand),
            Type::Tsx => stack::tsx(cpu, operand),
            Type::Txa => transfer::txa(cpu, operand),
            Type::Txs => stack::txs(cpu, operand),
            Type::Tya => transfer::tya(cpu, operand),
        }
    }

    pub fn writes_memory(self) -> bool {
        match self {
            Self::Asl | Self::Dec | Self::Inc | Self::Rol | Self::Ror | Self::Sta => true,
            _ => false,
        }
    }
}
