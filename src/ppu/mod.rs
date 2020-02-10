use super::mapper;

// Reference: https://wiki.nesdev.com/w/index.php/PPU_programmer_reference
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

    // Writing $XX will upload 256 bytes of data from CPU page $XX00-$XXFF to
    // the internal PPU OAM. This page is typically located in internal RAM,
    // commonly $0200-$02FF, but cartridge RAM or ROM can be used as well.
    pub oamdma: u8,
}

const TILE_PIXELS: usize = 8; // 8 pixels per tile
const TILE_ROWS: usize = 8; // 8x8 tiles
const TILE_BITPLANES: usize = 2; // 2 bitplanes per tile
const TILE_BYTES: usize = TILE_ROWS * TILE_BITPLANES; // 16 bytes per tile
const NAMETABLE_COLS: usize = 32;
const NAMETABLE_ROWS: usize = 30;
const ATTRIBUTE_TABLE_BYTES: usize = 64;
const NAMETABLE_BYTES: usize = (NAMETABLE_ROWS * NAMETABLE_COLS) + ATTRIBUTE_TABLE_BYTES; // 1024 bytes (0x400)
const OAM_BYTES: usize = 256;
const BG_SPRITE_PALETTES: usize = 8;
const COLORS_PER_PALETTE: usize = 4;
const PALETTE_BYTES: usize = BG_SPRITE_PALETTES * COLORS_PER_PALETTE;

pub const SCREEN_WIDTH: usize = NAMETABLE_COLS * TILE_PIXELS;
pub const SCREEN_HEIGHT: usize = NAMETABLE_ROWS * TILE_PIXELS;
pub const FRAMEBUFFER_BYTES: usize = SCREEN_HEIGHT * SCREEN_WIDTH * 3; // 3 colors per pixel

pub struct Ppu {
    regs: Registers,
    oam: [u8; OAM_BYTES],
    palette: [u8; PALETTE_BYTES],
    mapper: Box<dyn mapper::Ppu>,
    framebuf: [u8; FRAMEBUFFER_BYTES],
}

impl Ppu {
    pub fn new(mapper: Box<dyn mapper::Ppu>) -> Ppu {
        Ppu {
            regs: Registers::default(),
            oam: [0; OAM_BYTES],
            palette: [0; PALETTE_BYTES],
            mapper: mapper,
            framebuf: [0; FRAMEBUFFER_BYTES],
        }
    }

    // https://wiki.nesdev.com/w/index.php/PPU_memory_map
    pub fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x2FFF => self.mapper.read(addr),
            0x3000..=0x3EFF => self.mem_read(addr - 0x1000),
            0x3F00..=0x3F1F => self.palette[addr as usize - 0x3F00],
            0x3F20..=0x3F3F => self.palette[addr as usize - 0x3F20],
            _ => panic!("invalid address {}", addr),
        }
    }

    pub fn mem_write(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x2FFF => self.mapper.write(addr, v),
            0x3000..=0x3EFF => self.mem_write(addr - 0x1000, v),
            0x3F00..=0x3F1F => self.palette[addr as usize - 0x3F00] = v,
            0x3F20..=0x3F3F => self.palette[addr as usize - 0x3F20] = v,
            _ => panic!("invalid address {}", addr),
        }
    }
}
