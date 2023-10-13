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
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => {
                let index = self.rom_bank * 0x4000 + addr as usize - 0x4000;
                self.rom[index]
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let index = self.ram_bank * 0x2000 + addr as usize - 0xA000;
                    self.ram[index]
                } else {
                    0x00
                }
            }
            _ => 0x00,
        }
    }

    fn set_octet(&mut self, addr: u16, value: u8) {
        match addr {
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let index = self.ram_bank * 0x2000 + addr as usize - 0xA000;
                    self.ram[index] = value;
                }
            }
            0x0000..=0x1FFF => {
                self.ram_enable = value & 0x0F == 0x0A;
            }
            0x2000..=0x3FFF => {
                self.rom_bank = (self.rom_bank & 0x100) | (value as usize);
            }
            0x4000..=0x5FFF => {
                self.ram_bank = (value & 0x0F) as usize;
            }
            _ => {}
        }
    }
}
