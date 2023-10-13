#[derive(Debug, Copy, Clone)]
pub struct Registres {
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

impl Registres {
    pub fn new() -> Registres {
        let registres = Registres {
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
        registres
    }

    //get, set, et switch de registres
}