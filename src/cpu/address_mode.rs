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
