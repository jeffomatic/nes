use super::super::memory::Memory;
use super::registers::Registers;
use super::status::Status;

#[derive(Clone)]
pub struct State {
    pub regs: Registers,
    pub mem: Memory,
}

impl State {
    pub fn new() -> State {
        State {
            regs: Registers::default(),
            mem: Memory::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Update {
    Accumulator(u8),
    Status(Status, bool),
    Memory(u16, u8),
}

impl Update {
    pub fn apply(&self, state: &mut State) {
        match self {
            Self::Accumulator(n) => state.regs.a = *n,
            Self::Status(s, on) => state.regs.p = s.set_into(state.regs.p, *on),
            Self::Memory(addr, val) => state.mem.write(*addr, *val),
        }
    }
}

#[test]
fn test_update() {
    struct Case {
        current: Registers,
        update: Update,
        want: Registers,
    }

    for (i, c) in [
        Case {
            current: Registers {
                a: 0,
                x: 1,
                y: 2,
                pc: 3,
                s: 4,
                p: 0b1000_0000,
            },
            update: Update::Accumulator(5),
            want: Registers {
                a: 5,
                x: 1,
                y: 2,
                pc: 3,
                s: 4,
                p: 0b1000_0000,
            },
        },
        Case {
            current: Registers {
                a: 0,
                x: 1,
                y: 2,
                pc: 3,
                s: 4,
                p: 0b1000_0000,
            },
            update: Update::Status(Status::Negative, false),
            want: Registers {
                a: 0,
                x: 1,
                y: 2,
                pc: 3,
                s: 4,
                p: 0b0000_0000,
            },
        },
    ]
    .iter()
    .enumerate()
    {
        let mut state = State::new();
        state.regs = c.current.clone();
        c.update.apply(&mut state);
        assert_eq!(state.regs, c.want, "case {}", i);
    }
}
