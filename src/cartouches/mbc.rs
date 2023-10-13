use crate::memoire::Memoire;

pub struct MemoireBankController {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    ram_enable: bool,
}

impl MemoireBankController {
    pub fn new(rom: Vec<u8>, ram: Vec<u8>) -> Self {
        MemoireBankController {
            rom,
            ram,
            rom_bank: 1,
            ram_bank: 0,
            ram_enable: false,
        }
    }
}

impl Memoire for MemoireBankController {
    fn get_octet(&self, addr: u16) -> u8 {
        //TODO
    }
}
