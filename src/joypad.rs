use crate::memoire::Memoire;
use crate::mmu::InterruptFlag;

pub enum JoypadKey {
    Right = 0b0000_0001,
    Left = 0b0000_0010,
    Up = 0b0000_0100,
    Down = 0b0000_1000,
    A = 0b0001_0000,
    B = 0b0010_0000,
    Select = 0b0100_0000,
    Start = 0b1000_0000,
}

pub struct Joypad {
    pub matrix: u8,
    pub select: u8,
    pub interrupt: u8,
}

impl Joypad {
    // Constructeur du Joypad
    pub fn new() -> Joypad {
        Joypad {
            matrix: 0xFF,
            select: 0x00,
            interrupt: InterruptFlag::None as u8,
        }
    }

    // Méthode pour signaler qu'une touche est enfoncée
    pub fn keydown(&mut self, key: JoypadKey) {
        self.matrix &= !(key as u8);
        self.interrupt |= InterruptFlag::Joypad as u8;
    }

    // Méthode pour signaler qu'une touche est relâchée
    pub fn keyup(&mut self, key: JoypadKey) {
        self.matrix |= key as u8;
    }
}

impl Memoire for Joypad {
    fn get_octet(&self, addr: u16) -> u8 {
        assert_eq!(addr, 0xFF00);
        if (self.select & 0b0001_0000) == 0x00 {
            return self.select | (self.matrix & 0x0F);
        }
        if (self.select & 0b0010_0000) == 0x00 {
            return self.select | (self.matrix >> 4);
        }
        self.select
    }
}
