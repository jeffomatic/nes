// Reference: http://obelisk.me.uk/6502/registers.html
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Registers {
    // Accumulator
    pub a: u8,

    // Index X
    // Used for counters and memory offsets for particular instructions.
    // Unlike the Y register, it can be used to copy or manipulate the stack
    // pointer.
    pub x: u8,

    // Index Y
    pub y: u8,

    // Program counter
    pub pc: u16,

    // Stack pointer
    // Stack is 256 bytes, between 0x100 and 0x1FF. The pointer is the low
    // 8 bits.
    pub s: u8,

    // Processor status
    // 0 - Carry flag: set if the last op resulted in overflow in the high bit,
    //     or underflow in the low bit.
    // 1 - Zero flag: set if the last op resulted in zero.
    // 2 - Interrupt disable: set if interrupts have been disabled by SEI, and
    //     and not yet cleared by CLI.
    // 3 - Decimal mode: no effect on the NES. For reference, this status is set
    //     by SED and cleared by CLD. When set, arithmetic operations will obey
    //     Binary Coded Decimal (BCD). A byte represents a two-digit decimal
    //     number, with the low nibble representing the low digit, and the high
    //     nibble representing the high digit.
    // 4 - Break command: set during an interrupt sequence if the interrupt
    //     occurred due to user command.
    // 5 - Expansion bit: unused
    // 6 - Overflow flag: set if the last op resulted in a value greater than
    //     127. If this flag is set, the negative flag will also be set.
    // 7 - Negative flag: set if the last op resulted in a high bit of 1.
    pub p: u8,
}
