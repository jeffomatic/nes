use super::address_mode::AddressMode;
use super::registers::Registers;
use crate::math;
use crate::memory::Memory;

pub fn fetch_byte(regs: &Registers, mem: &Memory, addr_mode: AddressMode) -> u8 {
    match addr_mode {
        AddressMode::Accumulator => regs.a,
        AddressMode::Immediate => mem.read(regs.pc + 1),

        // TODO: zero page wraparound
        AddressMode::ZeroPage => mem.read(mem.read(regs.pc + 1) as u16),
        AddressMode::ZeroPageX => mem.read(mem.read(regs.pc + 1) as u16 + regs.x as u16),
        AddressMode::ZeroPageY => mem.read(mem.read(regs.pc + 1) as u16 + regs.y as u16),

        other => panic!("address mode {:?} has no fetch_byte implementation", other),
    }
}

pub fn fetch_address(regs: &Registers, mem: &Memory, addr_mode: AddressMode) -> u16 {
    match addr_mode {
        AddressMode::Relative => math::byte_addr_offset(regs.pc, mem.read(regs.pc + 1)),
        AddressMode::Absolute => mem.read16(regs.pc + 1),
        AddressMode::AbsoluteX => mem.read16(regs.pc + 1) + regs.x as u16,
        AddressMode::AbsoluteY => mem.read16(regs.pc + 1) + regs.y as u16,
        AddressMode::Indirect => mem.read16(mem.read16(regs.pc + 1)),

        // TODO: zero page wraparound
        AddressMode::IndirectX => mem.read16(mem.read(regs.pc + 1) as u16 + regs.x as u16),
        AddressMode::IndirectY => mem.read16(mem.read(regs.pc + 1) as u16) + regs.y as u16,

        other => panic!(
            "address mode {:?} has no fetch_address implementation",
            other
        ),
    }
}

#[test]
fn test_fetch_accumulator() {
    let mut regs = Registers::default();
    regs.a = 0xAB;
    let mem = Memory::new();
    assert_eq!(fetch_byte(&regs, &mem, AddressMode::Accumulator), 0xAB,);
}

#[test]
fn test_fetch_immediate() {
    let mut regs = Registers::default();
    regs.pc = 0x200;
    let mut mem = Memory::new();
    mem.write(0x201, 0xAB);
    assert_eq!(fetch_byte(&regs, &mem, AddressMode::Immediate), 0xAB);
}

#[test]
fn test_fetch_zero_page() {
    let mut regs = Registers::default();
    regs.pc = 0x200;
    let mut mem = Memory::new();
    mem.write(0xAB, 0xCD);
    mem.write(0x201, 0xAB);
    assert_eq!(fetch_byte(&regs, &mem, AddressMode::ZeroPage), 0xCD);
}

#[test]
fn test_fetch_zero_page_x() {
    let mut regs = Registers::default();
    regs.pc = 0x200;
    regs.x = 0x01;
    let mut mem = Memory::new();
    mem.write(0xAC, 0xCD);
    mem.write(0x201, 0xAB);
    assert_eq!(fetch_byte(&regs, &mem, AddressMode::ZeroPageX), 0xCD);
}

#[test]
fn test_fetch_zero_page_y() {
    let mut regs = Registers::default();
    regs.pc = 0x200;
    regs.y = 0x01;
    let mut mem = Memory::new();
    mem.write(0xAC, 0xCD);
    mem.write(0x201, 0xAB);
    assert_eq!(fetch_byte(&regs, &mem, AddressMode::ZeroPageY), 0xCD);
}

#[test]
fn test_fetch_relative() {
    let mut regs = Registers::default();
    regs.pc = 0x200;
    let mut mem = Memory::new();
    mem.write(0x201, 0xF0);
    assert_eq!(fetch_address(&regs, &mem, AddressMode::Relative), 0x1F0);
    mem.write(0x201, 0x0F);
    assert_eq!(fetch_address(&regs, &mem, AddressMode::Relative), 0x20F);
}

#[test]
fn test_fetch_absolute() {
    let mut regs = Registers::default();
    regs.pc = 0x200;
    let mut mem = Memory::new();
    mem.write(0x201, 0xCD);
    mem.write(0x202, 0xAB);
    assert_eq!(fetch_address(&regs, &mem, AddressMode::Absolute), 0xABCD);
}

#[test]
fn test_fetch_absolute_x() {
    let mut regs = Registers::default();
    regs.pc = 0x200;
    regs.x = 0x1;
    let mut mem = Memory::new();
    mem.write(0x201, 0xCD);
    mem.write(0x202, 0xAB);
    assert_eq!(fetch_address(&regs, &mem, AddressMode::AbsoluteX), 0xABCE);
}

#[test]
fn test_fetch_absolute_y() {
    let mut regs = Registers::default();
    regs.pc = 0x200;
    regs.y = 0x1;
    let mut mem = Memory::new();
    mem.write(0x201, 0xCD);
    mem.write(0x202, 0xAB);
    assert_eq!(fetch_address(&regs, &mem, AddressMode::AbsoluteY), 0xABCE);
}

#[test]
fn test_fetch_indirect() {
    let mut regs = Registers::default();
    regs.pc = 0x200;
    let mut mem = Memory::new();
    mem.write(0x201, 0xFF);
    mem.write(0x202, 0x04);
    assert_eq!(fetch_address(&regs, &mem, AddressMode::AbsoluteY), 0x4FF);
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
    assert_eq!(fetch_address(&regs, &mem, AddressMode::IndirectX), 0x1122);
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
    assert_eq!(fetch_address(&regs, &mem, AddressMode::IndirectY), 0x1123);
}
