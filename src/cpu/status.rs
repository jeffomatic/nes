#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode, // No effect on the NES.
    BreakCommand,
    ExpansionBit,
    Overflow,
    Negative,
}

impl Status {
    pub fn bit(self) -> usize {
        match self {
            Self::Carry => 0,
            Self::Zero => 1,
            Self::InterruptDisable => 2,
            Self::DecimalMode => 3,
            Self::BreakCommand => 4,
            Self::ExpansionBit => 5,
            Self::Overflow => 6,
            Self::Negative => 7,
        }
    }

    pub fn mask(self) -> u8 {
        1 << (self.bit() as u8)
    }
}
