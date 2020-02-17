use super::common;

pub fn new() -> (Prg, Ppu) {
    (
        Prg(vec![0; 0x8000]),
        Ppu(vec![0; 0x3000]), // large enough for pattern data and 4 full nametables
    )
}

pub struct Prg(Vec<u8>);

impl common::Prg for Prg {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0xFFFF => (*self.0)[addr as usize - 0x8000],
            _ => panic!("invalid address: {}", addr),
        }
    }

    fn write(&mut self, addr: u16, v: u8) {
        match addr {
            0x8000..=0xFFFF => (*self.0)[addr as usize - 0x8000] = v,
            _ => panic!("invalid address: {}", addr),
        }
    }
}

pub struct Ppu(Vec<u8>);

impl common::Ppu for Ppu {
    fn read(&self, addr: u16) -> u8 {
        return (*self.0)[addr as usize];
    }

    fn write(&mut self, addr: u16, v: u8) {
        (*self.0)[addr as usize] = v;
    }
}
