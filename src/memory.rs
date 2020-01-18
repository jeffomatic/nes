pub struct Memory {
    ram: [u8; 1 << 11],
}

impl Memory {
    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        self.ram[addr as usize] = v
    }
}
