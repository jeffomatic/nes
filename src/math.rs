pub fn bytes_to_u16_le(bytes: [u8; 2]) -> u16 {
    ((bytes[1] as u16) << 8) | (bytes[0] as u16)
}

#[test]
pub fn test_bytes_to_u16_le() {
    assert_eq!(bytes_to_u16_le([0xFF, 0xFF]), 0xFFFF);
    assert_eq!(bytes_to_u16_le([0x00, 0xFF]), 0xFF00);
    assert_eq!(bytes_to_u16_le([0xFF, 0x00]), 0xFF);
}

pub fn byte_addr_offset(src: u16, offset: u8) -> u16 {
    (src as i32 + (offset as i8 as i32)) as u16
}

#[test]
pub fn test_byte_addr_offset() {
    assert_eq!(byte_addr_offset(0xFF, 0xFF), 0xFE);
    assert_eq!(byte_addr_offset(0xFF, 0x80), 0x7F);
    assert_eq!(byte_addr_offset(0xFF, 0x7F), 0x17E);
    assert_eq!(byte_addr_offset(0x80, 0x80), 0);
    assert_eq!(byte_addr_offset(0x80, 0x7F), 0xFF);
    assert_eq!(byte_addr_offset(0xFF80, 0x7F), 0xFFFF);
}

pub fn same_sign(x: u8, y: u8) -> bool {
    x & 0b1000_0000 == y & 0b1000_0000
}

pub fn u16_to_bytes_le(x: u16) -> [u8; 2] {
    [x as u8, (x >> 8) as u8]
}

pub fn u16_hi(x: u16) -> u8 {
    (x >> 8) as u8
}

pub fn u16_lo(x: u16) -> u8 {
    x as u8
}

pub fn u16_set_hi(x: u16, hi: u8) -> u16 {
    (x & 0xFF) | (hi as u16) << 8
}

pub fn u16_set_lo(x: u16, lo: u8) -> u16 {
    (x & 0xFF00) | (lo as u16)
}

#[test]
pub fn test_hi_lo() {
    assert_eq!(u16_hi(0xABCD), 0xAB);
    assert_eq!(u16_lo(0xABCD), 0xCD);
    assert_eq!(u16_set_hi(0xABCD, 0xFF), 0xFFCD);
    assert_eq!(u16_set_lo(0xABCD, 0xFF), 0xABFF);
}

pub fn page_crossing(x: u16, y: u16) -> bool {
    x & 0xFF00 != y & 0xFF00
}
