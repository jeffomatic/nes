use super::opcode;
use super::operand::Operand;
use super::state::Cpu;

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

pub fn execute(opcode_type: opcode::Type, cpu: &mut Cpu, operand: Operand) {
    match opcode_type {
        opcode::Type::Adc => arithmetic::adc(cpu, operand),
        opcode::Type::And => logic::and(cpu, operand),
        opcode::Type::Asl => shift::asl(cpu, operand),
        opcode::Type::Bcc => branch::bcc(cpu, operand),
        opcode::Type::Bcs => branch::bcs(cpu, operand),
        opcode::Type::Beq => branch::beq(cpu, operand),
        opcode::Type::Bit => logic::bit(cpu, operand),
        opcode::Type::Bmi => branch::bmi(cpu, operand),
        opcode::Type::Bne => branch::bne(cpu, operand),
        opcode::Type::Bpl => branch::bpl(cpu, operand),
        opcode::Type::Brk => system::brk(cpu, operand),
        opcode::Type::Bvc => branch::bvc(cpu, operand),
        opcode::Type::Bvs => branch::bvs(cpu, operand),
        opcode::Type::Clc => status::clc(cpu, operand),
        opcode::Type::Cld => status::cld(cpu, operand),
        opcode::Type::Cli => status::cli(cpu, operand),
        opcode::Type::Clv => status::clv(cpu, operand),
        opcode::Type::Cmp => arithmetic::cmp(cpu, operand),
        opcode::Type::Cpx => arithmetic::cpx(cpu, operand),
        opcode::Type::Cpy => arithmetic::cpy(cpu, operand),
        opcode::Type::Dec => incdec::dec(cpu, operand),
        opcode::Type::Dex => incdec::dex(cpu, operand),
        opcode::Type::Dey => incdec::dey(cpu, operand),
        opcode::Type::Eor => logic::eor(cpu, operand),
        opcode::Type::Inc => incdec::inc(cpu, operand),
        opcode::Type::Inx => incdec::inx(cpu, operand),
        opcode::Type::Iny => incdec::iny(cpu, operand),
        opcode::Type::Jmp => jump::jmp(cpu, operand),
        opcode::Type::Jsr => jump::jsr(cpu, operand),
        opcode::Type::Lda => loadstore::lda(cpu, operand),
        opcode::Type::Ldx => loadstore::ldx(cpu, operand),
        opcode::Type::Ldy => loadstore::ldy(cpu, operand),
        opcode::Type::Lsr => shift::lsr(cpu, operand),
        opcode::Type::Nop => system::nop(cpu, operand),
        opcode::Type::Ora => logic::ora(cpu, operand),
        opcode::Type::Pha => stack::pha(cpu, operand),
        opcode::Type::Php => stack::php(cpu, operand),
        opcode::Type::Pla => stack::pla(cpu, operand),
        opcode::Type::Plp => stack::plp(cpu, operand),
        opcode::Type::Rol => shift::rol(cpu, operand),
        opcode::Type::Ror => shift::ror(cpu, operand),
        opcode::Type::Rti => system::rti(cpu, operand),
        opcode::Type::Rts => jump::rts(cpu, operand),
        opcode::Type::Sbc => arithmetic::sbc(cpu, operand),
        opcode::Type::Sec => status::sec(cpu, operand),
        opcode::Type::Sed => status::sed(cpu, operand),
        opcode::Type::Sei => status::sei(cpu, operand),
        opcode::Type::Sta => loadstore::sta(cpu, operand),
        opcode::Type::Stx => loadstore::stx(cpu, operand),
        opcode::Type::Sty => loadstore::sty(cpu, operand),
        opcode::Type::Tax => transfer::tax(cpu, operand),
        opcode::Type::Tay => transfer::tay(cpu, operand),
        opcode::Type::Tsx => stack::tsx(cpu, operand),
        opcode::Type::Txa => transfer::txa(cpu, operand),
        opcode::Type::Txs => stack::txs(cpu, operand),
        opcode::Type::Tya => transfer::tya(cpu, operand),
    }
}
