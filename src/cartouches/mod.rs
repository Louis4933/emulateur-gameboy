mod mbc;
mod rom;

use crate::cartouches::mbc::MemoireBankController;
use crate::cartouches::rom::RomOnly;
use crate::memoire::Memoire;

pub trait Cartouche: Memoire + Send {}

impl Cartouche for MemoireBankController {}

pub fn new(rom: Vec<u8>) -> Box<dyn Cartouche> {
    let cartouche: Box<dyn Cartouche> = match rom[0x0147] {
        0x00 => Box::new(RomOnly::new(rom)),
        0x01 | 0x0F | 0x11 | 0x19 => Box::new(MemoireBankController::new(rom, vec![])),
        0x02 | 0x05 | 0x12 | 0x1A | 0x03 | 0x06 | 0x10 | 0x13 | 0x1B  => {
            let ram_size = get_taille_ram(rom.as_ref());
            Box::new(MemoireBankController::new(rom, vec![0; ram_size]))
        }
        byte => panic!("cartouche: unsupported type {:#04X?}", byte),
    };
    cartouche
}

pub fn get_taille_ram(rom: &[u8]) -> usize {
    let ram_size_addr = 0x149;
    match rom[ram_size_addr] {
        0x00 => 0,
        0x01 => 1024 * 2,
        0x02 => 1024 * 8,
        0x03 => 1024 * 32,
        0x04 => 1024 * 128,
        0x05 => 1024 * 64,
        byte => panic!("cartouche: unsupported ram size {:#04X?}", byte),
    }
}