// http://wiki.nesdev.com/w/index.php/NROM

use super::common;

pub const PRG_SIZE: usize = 1 << 13;
pub const CHR_SIZE: usize = 1 << 12;

pub fn new(prg: &Vec<u8>, chr: &Vec<u8>) -> (Prg, Chr) {
    (Prg(prg.clone()), Chr(chr.clone()))
}

pub struct Prg(Vec<u8>);

impl common::Prg for Prg {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0xBFFF => (*self.0)[addr as usize - 0x8000],
            0xC000..=0xFFFF => (*self.0)[addr as usize - 0xC000],
            _ => panic!("invalid address: {}", addr),
        }
    }

    fn write(&mut self, addr: u16, v: u8) {
        panic!("not writeable");
    }
}

pub struct Chr(Vec<u8>);

impl common::Chr for Chr {
    fn read(&self, addr: u16) -> u8 {
        return (*self.0)[addr as usize];
    }

    fn write(&mut self, addr: u16, v: u8) {
        panic!("not writeable");
    }
}
