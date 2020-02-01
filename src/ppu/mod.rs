#[derive(Default)]
pub struct Registers {
    pub ppuctrl: u8,
    pub ppumask: u8,
    pub ppustatus: u8,
    pub oamaddr: u8,
    pub oamdata: u8,
    pub ppuscroll: u8,
    pub ppuaddr: u8,
    pub ppudata: u8,
    pub oamdma: u8,
}

#[derive(Default)]
pub struct Ppu {
    pub regs: Registers,
}
