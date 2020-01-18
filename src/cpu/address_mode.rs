// Reference: http://obelisk.me.uk/6502/addressing.html
#[derive(Clone, Copy, Debug)]
pub enum AddressMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX, // aka "indexed indirect"
    IndirectY, // aka "indirect indexed"
}

impl AddressMode {
    pub fn op_bytes(&self) -> usize {
        match self {
            Self::Implicit => 0,
            Self::Accumulator => 0,
            Self::Immediate => 1,
            Self::ZeroPage => 1,
            Self::ZeroPageX => 1,
            Self::ZeroPageY => 1,
            Self::Relative => 1,
            Self::Absolute => 2,
            Self::AbsoluteX => 2,
            Self::AbsoluteY => 2,
            Self::Indirect => 2,
            Self::IndirectX => 1,
            Self::IndirectY => 1,
        }
    }
}
