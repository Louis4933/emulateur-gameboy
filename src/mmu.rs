use crate::cartouches::Cartouche;
use crate::joypad::Joypad;
use crate::memoire::Memoire;
use crate::ppu::Ppu;
use crate::timer::timer::Timer;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Vitesse {
    Normal = 0x01,
    Double = 0x02,
}

#[derive(Eq, PartialEq)]
pub enum InterruptFlag {
    VBlank = 0b0000_0001,
    LCDStat = 0b0000_0010,
    Timer = 0b0000_0100,
    Joypad = 0b0001_0000,
    None = 0b0000_0000,
}

const HRAM_SIZE: usize = 0x7F;
const WRAM_SIZE: usize = 0x8000;
const WRAM_BANK_SIZE: usize = 0x1000;

pub struct Mmu {
    pub cartouche: Box<dyn Cartouche>,
    pub ppu: Ppu,
    pub joypad: Joypad,
    timer: Timer,
    vitesse: Vitesse,
    prepare_vitesse_switch: bool,
    hram: [u8; HRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    wram_bank: usize,
    interruptions_asserted: u8,
    interruptions_enabled: u8,
}

impl Mmu {
    pub fn new(cartouche: Box<dyn Cartouche>) -> Mmu {
        let mut mmu = Mmu {
            cartouche,
            ppu: Ppu::new(),
            joypad: Joypad::new(),
            timer: Timer::new(),
            vitesse: Vitesse::Normal,
            prepare_vitesse_switch: false,
            hram: [0x00; HRAM_SIZE],
            wram: [0x00; WRAM_SIZE],
            wram_bank: 0x01,
            interruptions_asserted: InterruptFlag::None as u8,
            interruptions_enabled: 0x00,
        };

        mmu.set_octet(0xFF05, 0x00);
        mmu.set_octet(0xFF06, 0x00);
        mmu.set_octet(0xFF07, 0x00);
        mmu.set_octet(0xFF10, 0x80);
        mmu.set_octet(0xFF11, 0xBF);
        mmu.set_octet(0xFF12, 0xF3);
        mmu.set_octet(0xFF14, 0xBF);
        mmu.set_octet(0xFF16, 0x3F);
        mmu.set_octet(0xFF17, 0x00);
        mmu.set_octet(0xFF19, 0xBF);
        mmu.set_octet(0xFF1A, 0x7F);
        mmu.set_octet(0xFF1B, 0xFF);
        mmu.set_octet(0xFF1C, 0x9F);
        mmu.set_octet(0xFF1E, 0xFF);
        mmu.set_octet(0xFF20, 0xFF);
        mmu.set_octet(0xFF21, 0x00);
        mmu.set_octet(0xFF22, 0x00);
        mmu.set_octet(0xFF23, 0xBF);
        mmu.set_octet(0xFF24, 0x77);
        mmu.set_octet(0xFF25, 0xF3);
        mmu.set_octet(0xFF26, 0xF1);
        mmu.set_octet(0xFF40, 0x91);
        mmu.set_octet(0xFF42, 0x00);
        mmu.set_octet(0xFF43, 0x00);
        mmu.set_octet(0xFF45, 0x00);
        mmu.set_octet(0xFF47, 0xFC);
        mmu.set_octet(0xFF48, 0xFF);
        mmu.set_octet(0xFF49, 0xFF);
        mmu.set_octet(0xFF4A, 0x00);
        mmu.set_octet(0xFF4B, 0x00);
    }

    pub fn perform_vitesse_switch(&mut self) {
        if self.prepare_vitesse_switch {
            self.vitesse = if self.vitesse == Vitesse::Double {
                Vitesse::Normal
            } else {
                Vitesse::Double
            }
        }
        self.prepare_vitesse_switch = false;
    }
}

impl Memoire for Mmu {
    fn get_octet(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cartouche.get_octet(addr),
                        0x8000..=0x9FFF => self.ppu.get_octet(addr),
            0xA000..=0xBFFF => self.cartouche.get_octet(addr),
            0xC000..=0xDFFF => match addr {
                0xC000..=0xCFFF => self.wram[addr as usize - 0xC000],
                0xD000..=0xDFFF => {
                    self.wram[addr as usize - 0xD000 + WRAM_BANK_SIZE * self.wram_bank]
                }
                _ => 0x00,
            },
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80],
            0xFFFF => self.interruptions_enabled,
        }
    }

    fn set_octet(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => self.cartouche.set_octet(addr, value),
            0x8000..=0x9FFF => self.ppu.set_octet(addr, value),
            0xA000..=0xBFFF => self.cartouche.set_octet(addr, value),
            0xC000..=0xDFFF => match addr {
                0xC000..=0xCFFF => self.wram[addr as usize - 0xC000] = value,
                0xD000..=0xDFFF => {
                    self.wram[addr as usize - 0xD000 + WRAM_BANK_SIZE * self.wram_bank] = value
                }
                _ => {}
            },
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80] = value,
            0xFFFF => self.interruptions_enabled = value,
        }
    }
}