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

