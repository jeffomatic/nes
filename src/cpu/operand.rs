use super::address_mode::AddressMode;
use super::registers::Registers;
use crate::math;
use crate::memory::Memory;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Opval {
    Byte(u8),
    Address(u16),
}

fn fetch(regs: &Registers, mem: &Memory, addr_mode: AddressMode) -> Option<Opval> {
    match addr_mode {
        AddressMode::Implicit => None,
        AddressMode::Accumulator => Some(Opval::Byte(regs.a)),
        AddressMode::Immediate => Some(Opval::Byte(mem.read(regs.pc + 1))),

        // TODO: zero page wraparound
        AddressMode::ZeroPage => Some(Opval::Byte(mem.read(mem.read(regs.pc + 1) as u16))),
        AddressMode::ZeroPageX => Some(Opval::Byte(
            mem.read(mem.read(regs.pc + 1) as u16 + regs.x as u16),
        )),
        AddressMode::ZeroPageY => Some(Opval::Byte(
            mem.read(mem.read(regs.pc + 1) as u16 + regs.y as u16),
        )),

        AddressMode::Relative => Some(Opval::Address(math::byte_addr_offset(
            regs.pc,
            mem.read(regs.pc + 1),
        ))),
        AddressMode::Absolute => Some(Opval::Address(mem.read16(regs.pc + 1))),
        AddressMode::AbsoluteX => Some(Opval::Address(mem.read16(regs.pc + 1) + regs.x as u16)),
        AddressMode::AbsoluteY => Some(Opval::Address(mem.read16(regs.pc + 1) + regs.y as u16)),
        AddressMode::Indirect => Some(Opval::Address(mem.read16(mem.read16(regs.pc + 1)))),

        // TODO: zero page wraparound
        AddressMode::IndirectX => Some(Opval::Address(
            mem.read16(mem.read(regs.pc + 1) as u16 + regs.x as u16),
        )),
        AddressMode::IndirectY => Some(Opval::Address(
            mem.read16(mem.read(regs.pc + 1) as u16) + regs.y as u16,
        )),
    }
}

#[test]
fn test_fetch() {
    let mut regs = Registers::default();
    regs.a = 0x01;
    regs.x = 0x02;
    regs.y = 0x03;
    regs.pc = 0x200;

    let mut mem = Memory::new();
    mem.write(0xEE, 0xAA);
    mem.write(0xF0, 0xBB);
    mem.write(0xF1, 0xCC);
    mem.write(0x201, 0xEE);
    mem.write(0x202, 0x03);
    mem.write(0x3EE, 0xDD);
    mem.write(0x3EF, 0x04);

    assert_eq!(fetch(&regs, &mem, AddressMode::Implicit), None);

    assert_eq!(
        fetch(&regs, &mem, AddressMode::Accumulator),
        Some(Opval::Byte(0x01))
    );

    assert_eq!(
        fetch(&regs, &mem, AddressMode::Immediate),
        Some(Opval::Byte(0xEE))
    );

    assert_eq!(
        fetch(&regs, &mem, AddressMode::ZeroPage),
        Some(Opval::Byte(0xAA))
    );

    assert_eq!(
        fetch(&regs, &mem, AddressMode::ZeroPageX),
        Some(Opval::Byte(0xBB))
    );

    assert_eq!(
        fetch(&regs, &mem, AddressMode::ZeroPageY),
        Some(Opval::Byte(0xCC))
    );

    assert_eq!(
        fetch(&regs, &mem, AddressMode::Relative),
        Some(Opval::Address(0x1EE))
    );

    assert_eq!(
        fetch(&regs, &mem, AddressMode::Absolute),
        Some(Opval::Address(0x3EE))
    );

    assert_eq!(
        fetch(&regs, &mem, AddressMode::AbsoluteX),
        Some(Opval::Address(0x3F0))
    );

    assert_eq!(
        fetch(&regs, &mem, AddressMode::AbsoluteY),
        Some(Opval::Address(0x3F1))
    );

    assert_eq!(
        fetch(&regs, &mem, AddressMode::Indirect),
        Some(Opval::Address(0x4DD))
    );
}

#[test]
fn test_fetch_indirect_x() {
    let mut regs = Registers::default();
    regs.x = 0x02;
    regs.pc = 0x200;

    let mut mem = Memory::new();
    mem.write(0x201, 0x01);
    mem.write(0x03, 0x22);
    mem.write(0x04, 0x11);

    assert_eq!(
        fetch(&regs, &mem, AddressMode::IndirectX),
        Some(Opval::Address(0x1122))
    );
}

#[test]
fn test_fetch_indirect_y() {
    let mut regs = Registers::default();
    regs.y = 0x01;
    regs.pc = 0x200;

    let mut mem = Memory::new();
    mem.write(0x201, 0x01);
    mem.write(0x01, 0x22);
    mem.write(0x02, 0x11);

    assert_eq!(
        fetch(&regs, &mem, AddressMode::IndirectY),
        Some(Opval::Address(0x1123))
    );
}
