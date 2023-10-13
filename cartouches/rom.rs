use crate::cartouches::Cartouche;
use crate::memoire::Memoire;

pub struct RomOnly {
    rom: Vec<u8>,
}

impl RomOnly {
    pub fn new(rom: Vec<u8>) -> RomOnly {
        RomOnly { rom }
    }
}

impl Memoire for RomOnly {
    fn get_octet(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn set_octet(&mut self, _: u16, _: u8) {}
}

impl Cartouche for RomOnly {}