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

pub fn is_negative(x: u8) -> bool {
    x & 0b1000_0000 != 0
}

pub fn u16_to_bytes_le(x: u16) -> [u8; 2] {
    [x as u8, (x >> 8) as u8]
}
