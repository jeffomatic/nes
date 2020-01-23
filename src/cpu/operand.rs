use super::opcode::Opcode;
use super::state::Cpu;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operand {
    None,
    Accumulator,
    Immediate(u8),
    Memory(u16),

    // Calling read() on a MemoryIndexReadOnly variant will consume one cycle.
    // This is because the base opcode/address mode cost is a discounted cost,
    // which assumes an optimization that isn't always possible to apply. If the
    // optimization isn't available, we have to consume the extra cycle.
    //
    // Some opcode/addressing mode combinations perform an indexed read, in which
    // an 8-bit offset is added to an 16-bit address. The adder is only 8 bits,
    // so a full add requires two cycles, in order to account for a possible
    // carry after adding the low bytes. However, we can pre-emptivly read the
    // summed address after the low bytes are processed. If there is no carry,
    // then the address we read is correct. If there _is_ carry, then we need to
    // burn another cycle to read the address immediately after the one we just
    // read.
    //
    // Cases where we burn an extra cycle are referred to as "page crossings",
    // because the high byte of the summed address will be one greater than the
    // high byte of the base address. For example, there is a page crossing
    // between 0x1FF and 0x200.
    //
    // The early-out read optimization is only available for read-only memory
    // access. We can pre-emptively read from memory, but we cannot pre-emptively
    // write, so both write-only and read/write operations must complete the
    // full addition operation in all cases.
    MemoryIndexedReadOnly(u16),
}

impl Operand {
    pub fn new_indexed(opcode: Opcode, addr: u16) -> Self {
        match opcode {
            Opcode::Asl | Opcode::Dec | Opcode::Inc | Opcode::Rol | Opcode::Ror | Opcode::Sta => {
                Self::Memory(addr)
            }
            _ => Self::MemoryIndexedReadOnly(addr),
        }
    }

    pub fn read(self, cpu: &Cpu) -> u8 {
        match self {
            Self::Accumulator => cpu.regs.a,
            Self::Immediate(val) => val,
            Self::Memory(addr) => cpu.mem_read(addr),
            Self::MemoryIndexedReadOnly(addr) => cpu.mem_read(addr), // TODO: increment cycle count on page crossing
            other => panic!("no readable value for {:?} operand", other),
        }
    }

    pub fn write(self, cpu: &mut Cpu, val: u8) {
        match self {
            Self::Accumulator => cpu.regs.a = val,
            Self::Memory(addr) => cpu.mem_write(addr, val),
            other => panic!("no writable value for {:?} operand", other),
        }
    }

    pub fn address(self) -> u16 {
        match self {
            Self::Memory(addr) | Self::MemoryIndexedReadOnly(addr) => addr,
            other => panic!("no address for {:?} operand", other),
        }
    }
}

#[test]
fn test_operand_accumulator() {
    let mut cpu = Cpu::new();
    cpu.regs.a = 0xAB;

    let op = Operand::Accumulator;
    assert_eq!(op.read(&cpu), 0xAB);

    op.write(&mut cpu, 0xCD);
    assert_eq!(cpu.regs.a, 0xCD);
}

#[test]
fn test_operand_immediate() {
    let op = Operand::Immediate(0xAB);
    assert_eq!(op.read(&Cpu::new()), 0xAB);
}

#[test]
fn test_operand_memory() {
    let mut cpu = Cpu::new();
    cpu.mem_write(0x1F, 0xAB);

    let op = Operand::Memory(0x1F);
    assert_eq!(op.read(&cpu), 0xAB);

    op.write(&mut cpu, 0xCD);
    assert_eq!(cpu.mem_read(0x1F), 0xCD);
}
