mod cartouches;
mod cpu;
mod joypad;
mod memoire;
mod mmu;
mod ppu;
mod timer;

use std::cell::RefCell;
use std::rc::Rc;

use crate::memoire::Memoire;
#[derive(Clone, Copy)]
pub enum GameboyButton {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

impl From<GameboyButton> for joypad::JoypadKey {
    fn from(value: GameboyButton) -> joypad::JoypadKey {
        match value {
            GameboyButton::A => joypad::JoypadKey::A,
            GameboyButton::B => joypad::JoypadKey::B,
            GameboyButton::Right => joypad::JoypadKey::Right,
            GameboyButton::Left => joypad::JoypadKey::Left,
            GameboyButton::Up => joypad::JoypadKey::Up,
            GameboyButton::Down => joypad::JoypadKey::Down,
            GameboyButton::Select => joypad::JoypadKey::Select,
            GameboyButton::Start => joypad::JoypadKey::Start,
        }
    }
}

pub struct Gameboy {
    mmu: Rc<RefCell<mmu::Mmu>>,
    cpu: cpu::RealTimeCpu,
}

impl Gameboy {

    pub fn new(rom: Vec<u8>) -> Gameboy {
        let cartouche = cartouches::new(rom);
        let mmu = Rc::new(RefCell::new(mmu::Mmu::new(cartouche)));
        let cpu = cpu::RealTimeCpu::new(mmu.clone());
        Gameboy { mmu, cpu }
    }

    pub fn step(&mut self) -> u32 {
        if self.mmu.borrow().get_octet(self.cpu.cpu.registres.pc) == 0x10 {
            self.mmu.borrow_mut().perform_vitesse_switch();
        }
        let cycles = self.cpu.run();
        self.mmu.borrow_mut().run_cycles(cycles);
        cycles
    }

    pub fn has_screen_updated(&mut self) -> bool {
        let result = self.mmu.borrow().ppu.vblank;
        self.mmu.borrow_mut().ppu.vblank = false;
        result
    }

    pub fn get_screen_dimension(&self) -> [usize;2] {
        return [ppu::SCREEN_HEIGHT, ppu::SCREEN_WIDTH];
    }
    
    pub fn get_screen_data(&self) -> [ppu::Pixel; ppu::SCREEN_WIDTH * ppu::SCREEN_HEIGHT] {
        self.mmu.borrow().ppu.data
    }

    pub fn can_take_input(&mut self) -> bool {
        self.cpu.flip()
    }

    pub fn gerer_keyup(&mut self, button: GameboyButton) {
        self.mmu.borrow_mut().joypad.keyup(button.into());
    }

    pub fn gerer_keydown(&mut self, button: GameboyButton) {
        self.mmu.borrow_mut().joypad.keydown(button.into());
    }
}
