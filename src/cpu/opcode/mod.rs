use crate::cpu::operand::Operand;
use crate::cpu::state::Cpu;

mod adc;
mod and;
mod asl;
mod bit;
mod branch;
mod brk;
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
    pub fn execute(self, cpu: &mut Cpu, operand: Operand) {
        match self {
            Opcode::Adc => adc::execute(cpu, operand),
            Opcode::And => and::execute(cpu, operand),
            Opcode::Asl => asl::execute(cpu, operand),
            Opcode::Bcc => branch::bcc(cpu, operand),
            Opcode::Bcs => branch::bcs(cpu, operand),
            Opcode::Beq => branch::beq(cpu, operand),
            Opcode::Bit => bit::execute(cpu, operand),
            Opcode::Bmi => branch::bmi(cpu, operand),
            Opcode::Bne => branch::bne(cpu, operand),
            Opcode::Bpl => branch::bpl(cpu, operand),
            Opcode::Brk => brk::execute(cpu, operand),
            Opcode::Bvc => branch::bvc(cpu, operand),
            Opcode::Bvs => branch::bvs(cpu, operand),
            Opcode::Clc => clc::execute(cpu, operand),
            Opcode::Cld => cld::execute(cpu, operand),
            Opcode::Cli => cli::execute(cpu, operand),
            Opcode::Clv => clv::execute(cpu, operand),
            Opcode::Cmp => cmp::execute(cpu, operand),
            Opcode::Cpx => cpx::execute(cpu, operand),
            Opcode::Cpy => cpy::execute(cpu, operand),
            Opcode::Dec => dec::execute(cpu, operand),
            Opcode::Dex => dex::execute(cpu, operand),
            Opcode::Dey => dey::execute(cpu, operand),
            Opcode::Eor => eor::execute(cpu, operand),
            Opcode::Inc => inc::execute(cpu, operand),
            Opcode::Inx => inx::execute(cpu, operand),
            Opcode::Iny => iny::execute(cpu, operand),
            Opcode::Jmp => jmp::execute(cpu, operand),
            Opcode::Jsr => jsr::execute(cpu, operand),
            Opcode::Lda => lda::execute(cpu, operand),
            Opcode::Ldx => ldx::execute(cpu, operand),
            Opcode::Ldy => ldy::execute(cpu, operand),
            Opcode::Lsr => lsr::execute(cpu, operand),
            Opcode::Nop => (),
            Opcode::Ora => ora::execute(cpu, operand),
            Opcode::Pha => pha::execute(cpu, operand),
            Opcode::Php => php::execute(cpu, operand),
            Opcode::Pla => pla::execute(cpu, operand),
            Opcode::Plp => plp::execute(cpu, operand),
            Opcode::Rol => rol::execute(cpu, operand),
            Opcode::Ror => ror::execute(cpu, operand),
            Opcode::Rti => rti::execute(cpu, operand),
            Opcode::Rts => rts::execute(cpu, operand),
            Opcode::Sbc => sbc::execute(cpu, operand),
            Opcode::Sec => sec::execute(cpu, operand),
            Opcode::Sed => sed::execute(cpu, operand),
            Opcode::Sei => sei::execute(cpu, operand),
            Opcode::Sta => sta::execute(cpu, operand),
            Opcode::Stx => stx::execute(cpu, operand),
            Opcode::Sty => sty::execute(cpu, operand),
            Opcode::Tax => tax::execute(cpu, operand),
            Opcode::Tay => tay::execute(cpu, operand),
            Opcode::Tsx => tsx::execute(cpu, operand),
            Opcode::Txa => txa::execute(cpu, operand),
            Opcode::Txs => txs::execute(cpu, operand),
            Opcode::Tya => tya::execute(cpu, operand),
        }
    }

    pub fn writes_memory(self) -> bool {
        match self {
            Self::Asl | Self::Dec | Self::Inc | Self::Rol | Self::Ror | Self::Sta => true,
            _ => false,
        }
    }
}
