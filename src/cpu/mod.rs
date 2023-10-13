mod registres;

use crate::cpu::registres::Registres;

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