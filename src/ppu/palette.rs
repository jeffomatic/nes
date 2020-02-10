pub const PALETTE_COLORS: usize = 64;

// The palettes in this file are taken from the Nestopia project:
// https://github.com/libretro/nestopia/blob/master/libretro/libretro.cpp

pub const CXA2025AS: [[u8; 3]; PALETTE_COLORS] = [
    [0x58, 0x58, 0x58],
    [0x00, 0x23, 0x8C],
    [0x00, 0x13, 0x9B],
    [0x2D, 0x05, 0x85],
    [0x5D, 0x00, 0x52],
    [0x7A, 0x00, 0x17],
    [0x7A, 0x08, 0x00],
    [0x5F, 0x18, 0x00],
    [0x35, 0x2A, 0x00],
    [0x09, 0x39, 0x00],
    [0x00, 0x3F, 0x00],
    [0x00, 0x3C, 0x22],
    [0x00, 0x32, 0x5D],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xA1, 0xA1, 0xA1],
    [0x00, 0x53, 0xEE],
    [0x15, 0x3C, 0xFE],
    [0x60, 0x28, 0xE4],
    [0xA9, 0x1D, 0x98],
    [0xD4, 0x1E, 0x41],
    [0xD2, 0x2C, 0x00],
    [0xAA, 0x44, 0x00],
    [0x6C, 0x5E, 0x00],
    [0x2D, 0x73, 0x00],
    [0x00, 0x7D, 0x06],
    [0x00, 0x78, 0x52],
    [0x00, 0x69, 0xA9],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
    [0x1F, 0xA5, 0xFE],
    [0x5E, 0x89, 0xFE],
    [0xB5, 0x72, 0xFE],
    [0xFE, 0x65, 0xF6],
    [0xFE, 0x67, 0x90],
    [0xFE, 0x77, 0x3C],
    [0xFE, 0x93, 0x08],
    [0xC4, 0xB2, 0x00],
    [0x79, 0xCA, 0x10],
    [0x3A, 0xD5, 0x4A],
    [0x11, 0xD1, 0xA4],
    [0x06, 0xBF, 0xFE],
    [0x42, 0x42, 0x42],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
    [0xA0, 0xD9, 0xFE],
    [0xBD, 0xCC, 0xFE],
    [0xE1, 0xC2, 0xFE],
    [0xFE, 0xBC, 0xFB],
    [0xFE, 0xBD, 0xD0],
    [0xFE, 0xC5, 0xA9],
    [0xFE, 0xD1, 0x8E],
    [0xE9, 0xDE, 0x86],
    [0xC7, 0xE9, 0x92],
    [0xA8, 0xEE, 0xB0],
    [0x95, 0xEC, 0xD9],
    [0x91, 0xE4, 0xFE],
    [0xAC, 0xAC, 0xAC],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
];

pub const PAL: [[u8; 3]; PALETTE_COLORS] = [
    [0x80, 0x80, 0x80],
    [0x00, 0x00, 0xBA],
    [0x37, 0x00, 0xBF],
    [0x84, 0x00, 0xA6],
    [0xBB, 0x00, 0x6A],
    [0xB7, 0x00, 0x1E],
    [0xB3, 0x00, 0x00],
    [0x91, 0x26, 0x00],
    [0x7B, 0x2B, 0x00],
    [0x00, 0x3E, 0x00],
    [0x00, 0x48, 0x0D],
    [0x00, 0x3C, 0x22],
    [0x00, 0x2F, 0x66],
    [0x00, 0x00, 0x00],
    [0x05, 0x05, 0x05],
    [0x05, 0x05, 0x05],
    [0xC8, 0xC8, 0xC8],
    [0x00, 0x59, 0xFF],
    [0x44, 0x3C, 0xFF],
    [0xB7, 0x33, 0xCC],
    [0xFE, 0x33, 0xAA],
    [0xFE, 0x37, 0x5E],
    [0xFE, 0x37, 0x1A],
    [0xD5, 0x4B, 0x00],
    [0xC4, 0x62, 0x00],
    [0x3C, 0x7B, 0x00],
    [0x1D, 0x84, 0x15],
    [0x00, 0x95, 0x66],
    [0x00, 0x84, 0xC4],
    [0x11, 0x11, 0x11],
    [0x09, 0x09, 0x09],
    [0x09, 0x09, 0x09],
    [0xFE, 0xFE, 0xFE],
    [0x00, 0x95, 0xFF],
    [0x6F, 0x84, 0xFF],
    [0xD5, 0x6F, 0xFF],
    [0xFE, 0x77, 0xCC],
    [0xFE, 0x6F, 0x99],
    [0xFE, 0x7B, 0x59],
    [0xFE, 0x91, 0x5F],
    [0xFE, 0xA2, 0x33],
    [0xA6, 0xBF, 0x00],
    [0x51, 0xD9, 0x6A],
    [0x4D, 0xD5, 0xAE],
    [0x00, 0xD9, 0xFF],
    [0x66, 0x66, 0x66],
    [0x0D, 0x0D, 0x0D],
    [0x0D, 0x0D, 0x0D],
    [0xFE, 0xFE, 0xFE],
    [0x84, 0xBF, 0xFF],
    [0xBB, 0xBB, 0xFF],
    [0xD0, 0xBB, 0xFF],
    [0xFE, 0xBF, 0xEA],
    [0xFE, 0xBF, 0xCC],
    [0xFE, 0xC4, 0xB7],
    [0xFE, 0xCC, 0xAE],
    [0xFE, 0xD9, 0xA2],
    [0xCC, 0xE1, 0x99],
    [0xAE, 0xEE, 0xB7],
    [0xAA, 0xF8, 0xEE],
    [0xB3, 0xEE, 0xFF],
    [0xDD, 0xDD, 0xDD],
    [0x11, 0x11, 0x11],
    [0x11, 0x11, 0x11],
];

pub const COMPOSITE_DIRECT_FBX: [[u8; 3]; PALETTE_COLORS] = [
    [0x65, 0x65, 0x65],
    [0x00, 0x12, 0x7D],
    [0x18, 0x00, 0x8E],
    [0x36, 0x00, 0x82],
    [0x56, 0x00, 0x5D],
    [0x5A, 0x00, 0x18],
    [0x4F, 0x05, 0x00],
    [0x38, 0x19, 0x00],
    [0x1D, 0x31, 0x00],
    [0x00, 0x3D, 0x00],
    [0x00, 0x41, 0x00],
    [0x00, 0x3B, 0x17],
    [0x00, 0x2E, 0x55],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xAF, 0xAF, 0xAF],
    [0x19, 0x4E, 0xC8],
    [0x47, 0x2F, 0xE3],
    [0x6B, 0x1F, 0xD7],
    [0x93, 0x1B, 0xAE],
    [0x9E, 0x1A, 0x5E],
    [0x99, 0x32, 0x00],
    [0x7B, 0x4B, 0x00],
    [0x5B, 0x67, 0x00],
    [0x26, 0x7A, 0x00],
    [0x00, 0x82, 0x00],
    [0x00, 0x7A, 0x3E],
    [0x00, 0x6E, 0x8A],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
    [0x64, 0xA9, 0xFF],
    [0x8E, 0x89, 0xFF],
    [0xB6, 0x76, 0xFF],
    [0xE0, 0x6F, 0xFF],
    [0xEF, 0x6C, 0xC4],
    [0xF0, 0x80, 0x6A],
    [0xD8, 0x98, 0x2C],
    [0xB9, 0xB4, 0x0A],
    [0x83, 0xCB, 0x0C],
    [0x5B, 0xD6, 0x3F],
    [0x4A, 0xD1, 0x7E],
    [0x4D, 0xC7, 0xCB],
    [0x4C, 0x4C, 0x4C],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
    [0xC7, 0xE5, 0xFF],
    [0xD9, 0xD9, 0xFF],
    [0xE9, 0xD1, 0xFF],
    [0xF9, 0xCE, 0xFF],
    [0xFF, 0xCC, 0xF1],
    [0xFF, 0xD4, 0xCB],
    [0xF8, 0xDF, 0xB1],
    [0xED, 0xEA, 0xA4],
    [0xD6, 0xF4, 0xA4],
    [0xC5, 0xF8, 0xB8],
    [0xBE, 0xF6, 0xD3],
    [0xBF, 0xF1, 0xF1],
    [0xB9, 0xB9, 0xB9],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
];

pub const PVM_STYLE_D93_FBX: [[u8; 3]; PALETTE_COLORS] = [
    [0x69, 0x6B, 0x63],
    [0x00, 0x17, 0x74],
    [0x1E, 0x00, 0x87],
    [0x34, 0x00, 0x73],
    [0x56, 0x00, 0x57],
    [0x5E, 0x00, 0x13],
    [0x53, 0x1A, 0x00],
    [0x3B, 0x24, 0x00],
    [0x24, 0x30, 0x00],
    [0x06, 0x3A, 0x00],
    [0x00, 0x3F, 0x00],
    [0x00, 0x3B, 0x1E],
    [0x00, 0x33, 0x4E],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xB9, 0xBB, 0xB3],
    [0x14, 0x53, 0xB9],
    [0x4D, 0x2C, 0xDA],
    [0x67, 0x1E, 0xDE],
    [0x98, 0x18, 0x9C],
    [0x9D, 0x23, 0x44],
    [0xA0, 0x3E, 0x00],
    [0x8D, 0x55, 0x00],
    [0x65, 0x6D, 0x00],
    [0x2C, 0x79, 0x00],
    [0x00, 0x81, 0x00],
    [0x00, 0x7D, 0x42],
    [0x00, 0x78, 0x8A],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
    [0x69, 0xA8, 0xFF],
    [0x96, 0x91, 0xFF],
    [0xB2, 0x8A, 0xFA],
    [0xEA, 0x7D, 0xFA],
    [0xF3, 0x7B, 0xC7],
    [0xF2, 0x8E, 0x59],
    [0xE6, 0xAD, 0x27],
    [0xD7, 0xC8, 0x05],
    [0x90, 0xDF, 0x07],
    [0x64, 0xE5, 0x3C],
    [0x45, 0xE2, 0x7D],
    [0x48, 0xD5, 0xD9],
    [0x4E, 0x50, 0x48],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
    [0xD2, 0xEA, 0xFF],
    [0xE2, 0xE2, 0xFF],
    [0xE9, 0xD8, 0xFF],
    [0xF5, 0xD2, 0xFF],
    [0xF8, 0xD9, 0xEA],
    [0xFA, 0xDE, 0xB9],
    [0xF9, 0xE8, 0x9B],
    [0xF3, 0xF2, 0x8C],
    [0xD3, 0xFA, 0x91],
    [0xB8, 0xFC, 0xA8],
    [0xAE, 0xFA, 0xCA],
    [0xCA, 0xF3, 0xF3],
    [0xBE, 0xC0, 0xB8],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
];

pub const NTSC_HARDWARE_FBX: [[u8; 3]; PALETTE_COLORS] = [
    [0x6A, 0x6D, 0x6A],
    [0x00, 0x13, 0x80],
    [0x1E, 0x00, 0x8A],
    [0x39, 0x00, 0x7A],
    [0x55, 0x00, 0x56],
    [0x5A, 0x00, 0x18],
    [0x4F, 0x10, 0x00],
    [0x38, 0x21, 0x00],
    [0x21, 0x33, 0x00],
    [0x00, 0x3D, 0x00],
    [0x00, 0x40, 0x00],
    [0x00, 0x39, 0x24],
    [0x00, 0x2E, 0x55],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xB9, 0xBC, 0xB9],
    [0x18, 0x50, 0xC7],
    [0x4B, 0x30, 0xE3],
    [0x73, 0x22, 0xD6],
    [0x95, 0x1F, 0xA9],
    [0x9D, 0x28, 0x5C],
    [0x96, 0x3C, 0x00],
    [0x7A, 0x51, 0x00],
    [0x5B, 0x67, 0x00],
    [0x22, 0x77, 0x00],
    [0x02, 0x7E, 0x02],
    [0x00, 0x76, 0x45],
    [0x00, 0x6E, 0x8A],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
    [0x68, 0xA6, 0xFF],
    [0x92, 0x99, 0xFF],
    [0xB0, 0x85, 0xFF],
    [0xD9, 0x75, 0xFD],
    [0xE3, 0x77, 0xB9],
    [0xE5, 0x8D, 0x68],
    [0xCF, 0xA2, 0x2C],
    [0xB3, 0xAF, 0x0C],
    [0x7B, 0xC2, 0x11],
    [0x55, 0xCA, 0x47],
    [0x46, 0xCB, 0x81],
    [0x47, 0xC1, 0xC5],
    [0x4A, 0x4D, 0x4A],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFF, 0xFF, 0xFF],
    [0xCC, 0xEA, 0xFF],
    [0xDD, 0xDE, 0xFF],
    [0xEC, 0xDA, 0xFF],
    [0xF8, 0xD7, 0xFE],
    [0xFC, 0xD6, 0xF5],
    [0xFD, 0xDB, 0xCF],
    [0xF9, 0xE7, 0xB5],
    [0xF1, 0xF0, 0xAA],
    [0xDA, 0xFA, 0xA9],
    [0xC9, 0xFF, 0xBC],
    [0xC3, 0xFB, 0xD7],
    [0xC4, 0xF6, 0xF6],
    [0xBE, 0xC1, 0xBE],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
];

pub const NES_CLASSIC_FBX_FS: [[u8; 3]; PALETTE_COLORS] = [
    [0x60, 0x61, 0x5F],
    [0x00, 0x00, 0x83],
    [0x1D, 0x01, 0x95],
    [0x34, 0x08, 0x75],
    [0x51, 0x05, 0x5E],
    [0x56, 0x00, 0x0F],
    [0x4C, 0x07, 0x00],
    [0x37, 0x23, 0x08],
    [0x20, 0x3A, 0x0B],
    [0x0F, 0x4B, 0x0E],
    [0x19, 0x4C, 0x16],
    [0x02, 0x42, 0x1E],
    [0x02, 0x31, 0x54],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xA9, 0xAA, 0xA8],
    [0x10, 0x4B, 0xBF],
    [0x47, 0x12, 0xD8],
    [0x63, 0x00, 0xCA],
    [0x88, 0x00, 0xA9],
    [0x93, 0x0B, 0x46],
    [0x8A, 0x2D, 0x04],
    [0x6F, 0x52, 0x06],
    [0x5C, 0x71, 0x14],
    [0x1B, 0x8D, 0x12],
    [0x19, 0x95, 0x09],
    [0x17, 0x84, 0x48],
    [0x20, 0x6B, 0x8E],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFB, 0xFB, 0xFB],
    [0x66, 0x99, 0xF8],
    [0x89, 0x74, 0xF9],
    [0xAB, 0x58, 0xF8],
    [0xD5, 0x57, 0xEF],
    [0xDE, 0x5F, 0xA9],
    [0xDC, 0x7F, 0x59],
    [0xC7, 0xA2, 0x24],
    [0xA7, 0xBE, 0x03],
    [0x75, 0xD7, 0x03],
    [0x60, 0xE3, 0x4F],
    [0x3C, 0xD6, 0x8D],
    [0x56, 0xC9, 0xCC],
    [0x41, 0x42, 0x40],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xFB, 0xFB, 0xFB],
    [0xBE, 0xD4, 0xFA],
    [0xC9, 0xC7, 0xF9],
    [0xD7, 0xBE, 0xFA],
    [0xE8, 0xB8, 0xF9],
    [0xF5, 0xBA, 0xE5],
    [0xF3, 0xCA, 0xC2],
    [0xDF, 0xCD, 0xA7],
    [0xD9, 0xE0, 0x9C],
    [0xC9, 0xEB, 0x9E],
    [0xC0, 0xED, 0xB8],
    [0xB5, 0xF4, 0xC7],
    [0xB9, 0xEA, 0xE9],
    [0xAB, 0xAB, 0xAB],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
];