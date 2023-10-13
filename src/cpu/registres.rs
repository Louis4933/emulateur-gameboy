#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,

    pub flags: u8,
}

#[derive(Copy, Clone)]
pub enum CpuFlag {
    ZERO = 0b10000000,
    SUB = 0b01000000,
    #[allow(non_camel_case_types)]
    HALF_CARRY = 0b00100000,
    CARRY = 0b00010000,
}

impl Registers {
    pub fn new() -> Registers {
        let registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            pc: 0x100,
            sp: 0xFFFE,

            flags: 0,
        };
        registers
    }

    pub fn get_msb(&self, a: u8, b: u8) -> u16 {
        return ((a as u16) << 8) | (b as u16);
    }

    pub fn af(&self) -> u16 {
        return self.get_msb(self.a, self.flags & 0xF0);
    }

    pub fn bc(&self) -> u16 {
        return self.get_msb(self.b, self.c);
    }

    pub fn de(&self) -> u16 {
        return self.get_msb(self.d, self.e);
    }

    pub fn hl(&self) -> u16 {
        return self.get_msb(self.h, self.l);
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.flags = (value & 0x00F0) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn hl_then_inc(&mut self) -> u16 {
        let res = self.hl();
        self.set_hl(res + 1);
        res
    }

    pub fn hl_then_dec(&mut self) -> u16 {
        let res = self.hl();
        self.set_hl(res - 1);
        res
    }

    pub fn set_flag_zero(&mut self, flag: CpuFlag, set: bool) {
        let mask = flag as u8;
        if set {
            self.flags |= mask;
        } else {
            self.flags &= !mask;
        }
    }

    pub fn has_flag(&self, flag: CpuFlag) -> bool {
        let mask = flag as u8;
        self.flags & mask > 0
    }
}