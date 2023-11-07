#[derive(Debug, Copy, Clone)]
pub struct LcdControl {
    pub data: u8,
}

impl LcdControl {
    pub fn new() -> LcdControl {
        LcdControl { data: 0b0100_1000 }
    }

    pub fn has_bit(&self, bit: u8) -> bool {
        (self.data & (1 << bit)) != 0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LcdStatus {
    pub lyc_interrupt_enabled: bool,
    pub m2_oam_interrupt_enabled: bool,
    pub m1_vblank_interrupt_enabled: bool,
    pub m0_hblank_interrupt_enabled: bool,
    pub mode: u8,
}

impl LcdStatus {
    pub fn new() -> LcdStatus {
        LcdStatus {
            lyc_interrupt_enabled: false,
            m2_oam_interrupt_enabled: false,
            m1_vblank_interrupt_enabled: false,
            m0_hblank_interrupt_enabled: false,
            mode: 0x00,
        }
    }
}
