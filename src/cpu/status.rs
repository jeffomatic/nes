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

    pub fn set_into(self, bitfield: u8, on: bool) -> u8 {
        if on {
            bitfield | self.mask()
        } else {
            bitfield & !self.mask()
        }
    }

    pub fn check(self, bitfield: u8) -> bool {
        bitfield & self.mask() != 0
    }

    pub fn with_zero_negative(bitfield: u8, val: u8) -> u8 {
        Status::Negative.set_into(
            Status::Zero.set_into(bitfield, val == 0),
            val & 0b1000_0000 != 0,
        )
    }
}
