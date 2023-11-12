pub mod timer {
    use crate::memoire::Memoire;
    use crate::mmu::InterruptFlag;

    #[derive(Debug, Copy, Clone)]
    struct Registers {
        div: u8,
        tima: u8,
        tma: u8,
        tac: u8,
    }

    impl Registers {
        pub fn new() -> Self {
            Self {
                div: 0x00,
                tima: 0x00,
                tma: 0x00,
                tac: 0x00,
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Timer {
        registers: Registers,
        pub interrupt: u8,
    }

    impl Timer {
        pub fn new() -> Self {
            Self {
                registers: Registers::new(),
                interrupt: InterruptFlag::None as u8,
            }
        }

        pub fn run_cycles(&mut self, cycles: u32) {
            if self.registers.tac & 0x04 != 0 {
                for _ in 0..cycles {
                    self.registers.tima = self.registers.tima.wrapping_add(1);
                    if self.registers.tima == 0 {
                        self.registers.tima = self.registers.tma;
                        self.interrupt |= InterruptFlag::Timer as u8;
                    }
                }
            }
        }
    }

    impl Memoire for Timer {
        fn get_octet(&self, addr: u16) -> u8 {
            match addr {
                0xFF04 => self.registers.div,
                0xFF05 => self.registers.tima,
                0xFF06 => self.registers.tma,
                0xFF07 => self.registers.tac,
                _ => 0x00,
            }
        }

        fn set_octet(&mut self, addr: u16, value: u8) {
            match addr {
                0xFF04 => self.registers.div = 0x00,
                0xFF05 => self.registers.tima = value,
                0xFF06 => self.registers.tma = value,
                _ => {}
            }
        }
    }
}
