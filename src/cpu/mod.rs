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

    // Le flag IME (interrupt master enable) est réinitialisé et interdit toutes les interruptions.
    pub fn gerer_interruptions(&mut self) -> u32 {
        if !self.halted && !self.ei {
            return 0;
        }
        let interruptions_asserted = self.get_octet_in_memoire(0xFF0F);
        let interruptions_enabled = self.get_octet_in_memoire(0xFFFF);
        let interruptions = interruptions_asserted & interruptions_enabled;
        if interruptions == 0x00 {
            return 0;
        }
        self.halted = false;
        if !self.ei {
            return 0;
        }
        self.ei = false;
        // Consomme une interruption et écrit le reste en mémoire
        let n = interruptions.trailing_zeros();
        let interruptions_asserted = interruptions_asserted & !(1 << n);
        self.set_octet_in_memoire(0xFF0F, interruptions_asserted);
        self.add_to_stack(self.registres.pc);
        // Régle le PC pour qu'il corresponde au programme d'interruption du process
        self.registres.pc = 0x0040 | ((n as u16) << 3);
        4
    }

    pub fn run(&mut self) -> u32 {
        let cycles = {
            match self.gerer_interruptions() {
                0 => {}
                n => return n,
            }
            if self.halted {
                1
            } else {
                let op_code = self.get_octet_at_pc();
                self.execute(op_code)
            }
        };
        cycles * 4
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
