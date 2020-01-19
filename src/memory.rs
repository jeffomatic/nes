use super::math;

#[derive(Clone)]
pub struct Memory {
    ram: [u8; 1 << 11],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; 1 << 11] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let addr = addr as usize;
        math::bytes_to_u16_le([self.ram[addr], self.ram[addr + 1]])
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        self.ram[addr as usize] = v
    }
}

#[test]
fn test() {
    let mut m = Memory::new();
    m.write(0, 0xFF);
    m.write(1, 0xEE);
    m.write(2, 0xDD);
    m.write((1 << 11) - 2, 0xCC);
    m.write((1 << 11) - 1, 0xBB);

    assert_eq!(m.read(0), 0xFF);
    assert_eq!(m.read((1 << 11) - 1), 0xBB);
    assert_eq!(m.read16(0), 0xEEFF);
    assert_eq!(m.read16(1), 0xDDEE);
    assert_eq!(m.read16((1 << 11) - 2), 0xBBCC);
}
