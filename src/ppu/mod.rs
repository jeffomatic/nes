use super::mapper;

mod palette;

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
const PALETTES_PER_SET: usize = 4;
const PALETTE_SETS: usize = 2; // BG and sprites each have a set of 4 palettes
const COLORS_PER_PALETTE: usize = 4;
const BYTES_PER_COLOR: usize = 1;
const BYTES_PER_PALETTE: usize = COLORS_PER_PALETTE * BYTES_PER_COLOR;
const ALL_PALETTES_BYTES: usize = (PALETTE_SETS * PALETTES_PER_SET) * BYTES_PER_PALETTE;

pub const SCREEN_WIDTH: usize = NAMETABLE_COLS * TILE_PIXELS;
pub const SCREEN_HEIGHT: usize = NAMETABLE_ROWS * TILE_PIXELS;
pub const SCREEN_ROW_PITCH: usize = SCREEN_WIDTH * 3; // 3 colors per pixel
pub const FRAMEBUFFER_BYTES: usize = SCREEN_HEIGHT * SCREEN_ROW_PITCH;

pub struct Ppu {
    regs: Registers,
    oam: [u8; OAM_BYTES],
    palette: [u8; ALL_PALETTES_BYTES],
    mapper: Box<dyn mapper::Ppu>,
    pub framebuf: [u8; FRAMEBUFFER_BYTES],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PixelColor {
    Transparent,
    Index(u8),
}

impl Ppu {
    pub fn new(mapper: Box<dyn mapper::Ppu>) -> Ppu {
        Ppu {
            regs: Registers::default(),
            oam: [0; OAM_BYTES],
            palette: [0; ALL_PALETTES_BYTES],
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

    pub fn mem_write_buf(&mut self, addr: u16, buf: Vec<u8>) {
        for (i, v) in buf.iter().enumerate() {
            self.mem_write(addr + i as u16, *v);
        }
    }

    // Returns the background color of the given worldspace pixel.
    fn bg_pixel_color(&self, x: usize, y: usize) -> PixelColor {
        // https://wiki.nesdev.com/w/index.php/PPU_nametables
        // quantize to tile coordinates and apply wrapping to 2x2 nametable space
        let xt = (x / 8) % (2 * NAMETABLE_COLS);
        let tile_col = x % 8;
        let yt = (y / 8) % (2 * NAMETABLE_ROWS);
        let tile_row = y % 8;

        // find nametable address and adjust tile coordinates
        let (nt_base, xt, yt) = match (xt >= NAMETABLE_COLS, yt >= NAMETABLE_ROWS) {
            (false, false) => (0x2000, xt, yt),
            (true, false) => (0x2400, xt - NAMETABLE_COLS, yt),
            (false, true) => (0x2800, xt, yt - NAMETABLE_ROWS),
            (true, true) => (0x2C00, xt - NAMETABLE_COLS, yt - NAMETABLE_ROWS),
        };

        // lookup pattern from nametable
        let nt_addr = nt_base + (yt * NAMETABLE_COLS) + xt;
        let pat_index = self.mem_read(nt_addr as u16);

        // fetch pattern bitplane rows
        let bp_base = 16 * pat_index as u16 + tile_row as u16;
        let bp_row_lo = self.mem_read(bp_base);
        let bp_row_hi = self.mem_read(bp_base + 8);

        // get index within palette
        let mask = 1 << tile_col;
        let pal_color = match (bp_row_hi & mask > 0, bp_row_lo & mask > 0) {
            (false, false) => return PixelColor::Transparent,
            (false, true) => 1,
            (true, false) => 2,
            (true, true) => 3,
        };

        // fetch palette from attribute table
        // https://wiki.nesdev.com/w/index.php/PPU_attribute_tables
        let ax = xt / 4;
        let ay = yt / 4;
        let attr_base = nt_base + 0x3C0;
        let attr_addr = attr_base + (8 * ay) + ax;
        let attr = self.mem_read(attr_addr as u16);
        let shift = match (yt % 4 >= 2, xt % 4 >= 2) {
            (false, false) => 0,
            (false, true) => 2,
            (true, false) => 4,
            (true, true) => 6,
        };
        let pal_index = (attr >> shift) & 0b11;
        let color_id_addr = 0x3F00 + (pal_index as usize * BYTES_PER_PALETTE) + pal_color;
        PixelColor::Index(self.mem_read(color_id_addr as u16))
    }
}

#[test]
fn test_bg_pixel_color() {
    // Example derived from: https://wiki.nesdev.com/w/index.php/PPU_attribute_tables#Worked_example
    let mut ppu = Ppu::new(Box::new(mapper::test::new().1));

    // palette
    ppu.mem_write_buf(
        0x3F00,
        vec![
            15, 1, 2, 3, // palette 0
            15, 4, 5, 6, // palette 1
            15, 7, 8, 9, // palette 2
            15, 10, 11, 12, // palette 3
        ],
    );

    // pattern data (tile 0x10)
    ppu.mem_write_buf(0x100, vec![0b10101010, 0b10101010]); // first two rows of low bitplane
    ppu.mem_write_buf(0x108, vec![0b11001100, 0b11001100]); // first two rows of high bitplane

    // attribute data (palette selection)
    ppu.mem_write(
        0x23C0 + (5 * NAMETABLE_COLS as u16 / 4) + 3, // attribute tile (3, 5) = nametable tiles ([12, 15], [20, 23])
        0b11100100, // NW = palette 0, NE = palette 1, SW = palette 2, SE = palette 3
    );

    // nametable data
    ppu.mem_write(
        0x2000 + (21 * NAMETABLE_COLS as u16) + 13, // tile (13, 21)
        0x10,                                       // pattern 0x10
    );
    ppu.mem_write(
        0x2000 + (21 * NAMETABLE_COLS as u16) + 14, // tile (14, 21)
        0x10,                                       // pattern 0x10
    );
    ppu.mem_write(
        0x2000 + (22 * NAMETABLE_COLS as u16) + 13, // tile (13, 22)
        0x10,                                       // pattern 0x10
    );
    ppu.mem_write(
        0x2000 + (22 * NAMETABLE_COLS as u16) + 14, // tile (14, 22)
        0x10,                                       // pattern 0x10
    );

    // NW metatile
    assert_eq!(ppu.bg_pixel_color(104, 168), PixelColor::Transparent);
    assert_eq!(ppu.bg_pixel_color(105, 168), PixelColor::Index(1));
    assert_eq!(ppu.bg_pixel_color(110, 168), PixelColor::Index(2));
    assert_eq!(ppu.bg_pixel_color(111, 169), PixelColor::Index(3));

    // NE metatile
    assert_eq!(ppu.bg_pixel_color(112, 168), PixelColor::Transparent);
    assert_eq!(ppu.bg_pixel_color(113, 168), PixelColor::Index(4));
    assert_eq!(ppu.bg_pixel_color(118, 168), PixelColor::Index(5));
    assert_eq!(ppu.bg_pixel_color(119, 169), PixelColor::Index(6));

    // SW metatile
    assert_eq!(ppu.bg_pixel_color(104, 176), PixelColor::Transparent);
    assert_eq!(ppu.bg_pixel_color(105, 176), PixelColor::Index(7));
    assert_eq!(ppu.bg_pixel_color(110, 176), PixelColor::Index(8));
    assert_eq!(ppu.bg_pixel_color(111, 177), PixelColor::Index(9));

    // SE metatile
    assert_eq!(ppu.bg_pixel_color(112, 176), PixelColor::Transparent);
    assert_eq!(ppu.bg_pixel_color(113, 176), PixelColor::Index(10));
    assert_eq!(ppu.bg_pixel_color(118, 177), PixelColor::Index(11));
    assert_eq!(ppu.bg_pixel_color(119, 177), PixelColor::Index(12));

    // Nametable wrapping
    assert_eq!(
        ppu.bg_pixel_color(105 + (0 * 256), 168 + (2 * 240)),
        PixelColor::Index(1)
    );
    assert_eq!(
        ppu.bg_pixel_color(105 + (2 * 256), 168 + (0 * 240)),
        PixelColor::Index(1)
    );
    assert_eq!(
        ppu.bg_pixel_color(105 + (2 * 256), 168 + (2 * 240)),
        PixelColor::Index(1)
    );
}
