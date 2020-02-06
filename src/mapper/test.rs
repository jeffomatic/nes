use super::common;

pub fn new() -> (Prg, Chr) {
    (Prg(vec![0; 0x8000]), Chr(vec![0; 0x800]))
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

pub struct Chr(Vec<u8>);

impl common::Chr for Chr {
    fn read(&self, addr: u16) -> u8 {
        return (*self.0)[addr as usize];
    }

    fn write(&mut self, addr: u16, v: u8) {
        (*self.0)[addr as usize] = v;
    }
}
