// A mapper models a cartridge, which contains the following:
// - extensions to the CPU memory map, including executable program data
// - extensions to the PPU memory map
pub trait Prg {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, v: u8);
}

pub trait Ppu {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, v: u8);
}
