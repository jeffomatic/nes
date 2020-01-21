use crate::cpu::operand::Operand;
use crate::cpu::state::State;

mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod brk;
mod bvc;
mod bvs;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
mod inc;
mod inx;
mod iny;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
mod lsr;
mod ora;
mod pha;
mod php;
mod pla;
mod plp;
mod rol;
mod ror;
mod rti;
mod rts;
mod sbc;
mod sec;
mod sed;
mod sei;
mod sta;
mod stx;
mod sty;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

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
    pub fn execute(&self, state: &mut State, operand: Operand) {
        match self {
            Opcode::Adc => adc::execute(state, operand),
            Opcode::And => and::execute(state, operand),
            Opcode::Asl => asl::execute(state, operand),
            Opcode::Bcc => bcc::execute(state, operand),
            Opcode::Bcs => bcs::execute(state, operand),
            Opcode::Beq => beq::execute(state, operand),
            Opcode::Bit => bit::execute(state, operand),
            Opcode::Bmi => bmi::execute(state, operand),
            Opcode::Bne => bne::execute(state, operand),
            Opcode::Bpl => bpl::execute(state, operand),
            Opcode::Brk => brk::execute(state, operand),
            Opcode::Bvc => bvc::execute(state, operand),
            Opcode::Bvs => bvs::execute(state, operand),
            Opcode::Clc => clc::execute(state, operand),
            Opcode::Cld => cld::execute(state, operand),
            Opcode::Cli => cli::execute(state, operand),
            Opcode::Clv => clv::execute(state, operand),
            Opcode::Cmp => cmp::execute(state, operand),
            Opcode::Cpx => cpx::execute(state, operand),
            Opcode::Cpy => cpy::execute(state, operand),
            Opcode::Dec => dec::execute(state, operand),
            Opcode::Dex => dex::execute(state, operand),
            Opcode::Dey => dey::execute(state, operand),
            Opcode::Eor => eor::execute(state, operand),
            Opcode::Inc => inc::execute(state, operand),
            Opcode::Inx => inx::execute(state, operand),
            Opcode::Iny => iny::execute(state, operand),
            Opcode::Jmp => jmp::execute(state, operand),
            Opcode::Jsr => jsr::execute(state, operand),
            Opcode::Lda => lda::execute(state, operand),
            Opcode::Ldx => ldx::execute(state, operand),
            Opcode::Ldy => ldy::execute(state, operand),
            Opcode::Lsr => lsr::execute(state, operand),
            Opcode::Nop => (),
            Opcode::Ora => ora::execute(state, operand),
            Opcode::Pha => pha::execute(state, operand),
            Opcode::Php => php::execute(state, operand),
            Opcode::Pla => pla::execute(state, operand),
            Opcode::Plp => plp::execute(state, operand),
            Opcode::Rol => rol::execute(state, operand),
            Opcode::Ror => ror::execute(state, operand),
            Opcode::Rti => rti::execute(state, operand),
            Opcode::Rts => rts::execute(state, operand),
            Opcode::Sbc => sbc::execute(state, operand),
            Opcode::Sec => sec::execute(state, operand),
            Opcode::Sed => sed::execute(state, operand),
            Opcode::Sei => sei::execute(state, operand),
            Opcode::Sta => sta::execute(state, operand),
            Opcode::Stx => stx::execute(state, operand),
            Opcode::Sty => sty::execute(state, operand),
            Opcode::Tax => tax::execute(state, operand),
            Opcode::Tay => tay::execute(state, operand),
            Opcode::Tsx => tsx::execute(state, operand),
            Opcode::Txa => txa::execute(state, operand),
            Opcode::Txs => txs::execute(state, operand),
            Opcode::Tya => tya::execute(state, operand),
        }
    }
}
