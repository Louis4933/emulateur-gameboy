mod registres;

use crate::cpu::registres::Registres;
use crate::memoire::Memoire;

pub struct Cpu {
    pub registres: Registres,
    pub halted: bool,
    pub stopped: bool,
    pub ei: bool,
}

impl Cpu {
    pub fn new(memoire: Rc<RefCell<dyn Memoire>>) -> Cpu {
        Cpu {
            registres: Registres::new(),
            halted: false,
            stopped: false,
            ei: false,
        }
    }

    // Obtient le prochain octet et augmente pc
    pub fn get_octet_at_pc(&mut self) -> u8 {
        let value = self.memoire.borrow().get_octet(self.registres.pc);
        self.registres.pc += 1;
        value
    }

    // Obtient le prochain mot et augmente pc
    pub fn get_mot_at_pc(&mut self) -> u16 {
        let value = self.memoire.borrow().get_mot(self.registres.pc);
        self.registres.pc += 2;
        value
    }

    pub fn get_octet_in_memoire(&self, addr: u16) -> u8 {
        self.memoire.borrow().get_octet(addr)
    }

    pub fn get_mot_in_memoire(&self, addr: u16) -> u16 {
        self.memoire.borrow().get_mot(addr)
    }

    pub fn set_octet_in_memoire(&mut self, addr: u16, value: u8) {
        self.memoire.borrow_mut().set_octet(addr, value);
    }

    pub fn set_mot_in_memoire(&mut self, addr: u16, value: u16) {
        self.memoire.borrow_mut().set_mot(addr, value);
    }

    pub fn add_to_stack(&mut self, value: u16) {
        self.registres.sp -= 2;
        self.set_mot_in_memoire(self.registres.sp, value);
    }

    pub fn pop_stack(&mut self) -> u16 {
        let result = self.get_mot_in_memoire(self.registres.sp);
        self.registres.sp += 2;
        result
    }

    pub fn gerer_interruptions(&mut self) -> u32 {
    }

    pub fn run(&mut self) -> u32 {
    }
}

mod cb_codes;
mod instructions;
mod op_codes;

pub struct RealTimeCpu {
    pub cpu: Cpu,
    //faire un emulateur du vrai cpu de la gameboy en limitant ses capacités
}

impl RealTimeCpu {
    pub fn new() -> RealTimeCpu {
        RealTimeCpu {
        }
    }

    // Simuler la vitesse d'exécution du matériel réel
    pub fn run(&mut self) -> u32 {
    }
}
