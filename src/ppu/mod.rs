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

const TILE_ROWS: usize = 8; // 8x8 tiles
const TILE_BITPLANES: usize = 2; // 2 bitplanes per tile
const TILE_BYTES: usize = TILE_ROWS * TILE_BITPLANES; // 16 bytes per tile
const PATTERN_TABLE_TILES: usize = 256; // 256 tiles per pattern table
const PATTERN_TABLE_BYTES: usize = TILE_BYTES * PATTERN_TABLE_TILES; // 4096 bytes (0x1000)
const NAMETABLE_COLS: usize = 32;
const NAMETABLE_ROWS: usize = 30;
const ATTRIBUTE_TABLE_BYTES: usize = 64;
const NAMETABLE_BYTES: usize = (NAMETABLE_ROWS * NAMETABLE_COLS) + ATTRIBUTE_TABLE_BYTES; // 1024 bytes (0x400)
const OAM_BYTES: usize = 256;

pub struct Ppu {
    pub regs: Registers,

    // CIRAM has two nametables' worth of memory, which are mirrored either
    // horizontally or vertically based on a cartridge setting.
    nametables: [[u8; NAMETABLE_BYTES]; 2],
    oam: [u8; OAM_BYTES],
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            regs: Registers::default(),
            nametables: [[0; NAMETABLE_BYTES]; 2],
            oam: [0; OAM_BYTES],
        }
    }
}
