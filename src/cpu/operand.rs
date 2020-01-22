use super::state::Cpu;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operand {
    None,
    Accumulator,
    Immediate(u8),
    Memory(u16),
}

impl Operand {
    pub fn read(self, cpu: &Cpu) -> u8 {
        match self {
            Self::Accumulator => cpu.regs.a,
            Self::Immediate(val) => val,
            Self::Memory(addr) => cpu.mem_read(addr),
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
            Self::Memory(addr) => addr,
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
