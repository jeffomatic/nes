use super::state::State;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operand {
    None,
    Accumulator,
    Immediate(u8),
    Memory(u16),
}

impl Operand {
    pub fn read(&self, state: &State) -> u8 {
        match self {
            Self::Accumulator => state.regs.a,
            Self::Immediate(val) => *val,
            Self::Memory(addr) => state.mem.read(*addr),
            other => panic!("no readable value for {:?} operand", other),
        }
    }

    pub fn write(&self, state: &mut State, val: u8) {
        match self {
            Self::Accumulator => state.regs.a = val,
            Self::Memory(addr) => state.mem.write(*addr, val),
            other => panic!("no writable value for {:?} operand", other),
        }
    }
}

#[test]
fn test_operand_accumulator() {
    let mut state = State::new();
    state.regs.a = 0xAB;

    let op = Operand::Accumulator;
    assert_eq!(op.read(&state), 0xAB);

    op.write(&mut state, 0xCD);
    assert_eq!(state.regs.a, 0xCD);
}

#[test]
fn test_operand_immediate() {
    let op = Operand::Immediate(0xAB);
    assert_eq!(op.read(&State::new()), 0xAB);
}

#[test]
fn test_operand_memory() {
    let mut state = State::new();
    state.mem.write(0x1F, 0xAB);

    let op = Operand::Memory(0x1F);
    assert_eq!(op.read(&state), 0xAB);

    op.write(&mut state, 0xCD);
    assert_eq!(state.mem.read(0x1F), 0xCD);
}
