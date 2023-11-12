use crate::cpu::Cpu;

const CB_CYCLES: [u32; 256] = [
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2,
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2,
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2,
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, 
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, 
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, 
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, 
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, 
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, 
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, 
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, 
];

impl Cpu {
    pub fn execute_cb(&mut self, cb_code: u8) -> u32 {
        match cb_code {
            0x00 => self.registres.b = self.inst_alu_rlc(self.registres.b),
            0x01 => self.registres.c = self.inst_alu_rlc(self.registres.c),
            0x02 => self.registres.d = self.inst_alu_rlc(self.registres.d),
            0x03 => self.registres.e = self.inst_alu_rlc(self.registres.e),
            0x04 => self.registres.h = self.inst_alu_rlc(self.registres.h),
            0x05 => self.registres.l = self.inst_alu_rlc(self.registres.l),
            0x06 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_rlc(value);
                self.set_octet_in_memoire(addr, result);
            }
            0x07 => self.registres.a = self.inst_alu_rlc(self.registres.a),
            0x08 => self.registres.b = self.inst_alu_rrc(self.registres.b),
            0x09 => self.registres.c = self.inst_alu_rrc(self.registres.c),
            0x0A => self.registres.d = self.inst_alu_rrc(self.registres.d),
            0x0B => self.registres.e = self.inst_alu_rrc(self.registres.e),
            0x0C => self.registres.h = self.inst_alu_rrc(self.registres.h),
            0x0D => self.registres.l = self.inst_alu_rrc(self.registres.l),
            0x0E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_rrc(value);
                self.set_octet_in_memoire(addr, result);
            }
            0x0F => self.registres.a = self.inst_alu_rrc(self.registres.a),
            0x10 => self.registres.b = self.inst_alu_rl(self.registres.b),
            0x11 => self.registres.c = self.inst_alu_rl(self.registres.c),
            0x12 => self.registres.d = self.inst_alu_rl(self.registres.d),
            0x13 => self.registres.e = self.inst_alu_rl(self.registres.e),
            0x14 => self.registres.h = self.inst_alu_rl(self.registres.h),
            0x15 => self.registres.l = self.inst_alu_rl(self.registres.l),
            0x16 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_rl(value);
                self.set_octet_in_memoire(addr, result);
            }
            0x17 => self.registres.a = self.inst_alu_rl(self.registres.a),
            0x18 => self.registres.b = self.inst_alu_rr(self.registres.b),
            0x19 => self.registres.c = self.inst_alu_rr(self.registres.c),
            0x1A => self.registres.d = self.inst_alu_rr(self.registres.d),
            0x1B => self.registres.e = self.inst_alu_rr(self.registres.e),
            0x1C => self.registres.h = self.inst_alu_rr(self.registres.h),
            0x1D => self.registres.l = self.inst_alu_rr(self.registres.l),
            0x1E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_rr(value);
                self.set_octet_in_memoire(addr, result);
            }
            0x1F => self.registres.a = self.inst_alu_rr(self.registres.a),
            0x20 => self.registres.b = self.inst_alu_sla(self.registres.b),
            0x21 => self.registres.c = self.inst_alu_sla(self.registres.c),
            0x22 => self.registres.d = self.inst_alu_sla(self.registres.d),
            0x23 => self.registres.e = self.inst_alu_sla(self.registres.e),
            0x24 => self.registres.h = self.inst_alu_sla(self.registres.h),
            0x25 => self.registres.l = self.inst_alu_sla(self.registres.l),
            0x26 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_sla(value);
                self.set_octet_in_memoire(addr, result);
            }
            0x27 => self.registres.a = self.inst_alu_sla(self.registres.a),
            0x28 => self.registres.b = self.inst_alu_sra(self.registres.b),
            0x29 => self.registres.c = self.inst_alu_sra(self.registres.c),
            0x2A => self.registres.d = self.inst_alu_sra(self.registres.d),
            0x2B => self.registres.e = self.inst_alu_sra(self.registres.e),
            0x2C => self.registres.h = self.inst_alu_sra(self.registres.h),
            0x2D => self.registres.l = self.inst_alu_sra(self.registres.l),
            0x2E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_sra(value);
                self.set_octet_in_memoire(addr, result);
            }
            0x2F => self.registres.a = self.inst_alu_sra(self.registres.a),
            0x30 => self.registres.b = self.inst_alu_swap(self.registres.b),
            0x31 => self.registres.c = self.inst_alu_swap(self.registres.c),
            0x32 => self.registres.d = self.inst_alu_swap(self.registres.d),
            0x33 => self.registres.e = self.inst_alu_swap(self.registres.e),
            0x34 => self.registres.h = self.inst_alu_swap(self.registres.h),
            0x35 => self.registres.l = self.inst_alu_swap(self.registres.l),
            0x36 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_swap(value);
                self.set_octet_in_memoire(addr, result);
            }
            0x37 => self.registres.a = self.inst_alu_swap(self.registres.a),
            0x38 => self.registres.b = self.inst_alu_srl(self.registres.b),
            0x39 => self.registres.c = self.inst_alu_srl(self.registres.c),
            0x3A => self.registres.d = self.inst_alu_srl(self.registres.d),
            0x3B => self.registres.e = self.inst_alu_srl(self.registres.e),
            0x3C => self.registres.h = self.inst_alu_srl(self.registres.h),
            0x3D => self.registres.l = self.inst_alu_srl(self.registres.l),
            0x3E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_srl(value);
                self.set_octet_in_memoire(addr, result);
            }
            0x3F => self.registres.a = self.inst_alu_srl(self.registres.a),
            0x40 => self.inst_alu_bit(self.registres.b, 0),
            0x41 => self.inst_alu_bit(self.registres.c, 0),
            0x42 => self.inst_alu_bit(self.registres.d, 0),
            0x43 => self.inst_alu_bit(self.registres.e, 0),
            0x44 => self.inst_alu_bit(self.registres.h, 0),
            0x45 => self.inst_alu_bit(self.registres.l, 0),
            0x46 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                self.inst_alu_bit(value, 0);
            }
            0x47 => self.inst_alu_bit(self.registres.a, 0),
            0x48 => self.inst_alu_bit(self.registres.b, 1),
            0x49 => self.inst_alu_bit(self.registres.c, 1),
            0x4A => self.inst_alu_bit(self.registres.d, 1),
            0x4B => self.inst_alu_bit(self.registres.e, 1),
            0x4C => self.inst_alu_bit(self.registres.h, 1),
            0x4D => self.inst_alu_bit(self.registres.l, 1),
            0x4E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                self.inst_alu_bit(value, 1);
            }
            0x4F => self.inst_alu_bit(self.registres.a, 1),
            0x50 => self.inst_alu_bit(self.registres.b, 2),
            0x51 => self.inst_alu_bit(self.registres.c, 2),
            0x52 => self.inst_alu_bit(self.registres.d, 2),
            0x53 => self.inst_alu_bit(self.registres.e, 2),
            0x54 => self.inst_alu_bit(self.registres.h, 2),
            0x55 => self.inst_alu_bit(self.registres.l, 2),
            0x56 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                self.inst_alu_bit(value, 2);
            }
            0x57 => self.inst_alu_bit(self.registres.a, 2),
            0x58 => self.inst_alu_bit(self.registres.b, 3),
            0x59 => self.inst_alu_bit(self.registres.c, 3),
            0x5A => self.inst_alu_bit(self.registres.d, 3),
            0x5B => self.inst_alu_bit(self.registres.e, 3),
            0x5C => self.inst_alu_bit(self.registres.h, 3),
            0x5D => self.inst_alu_bit(self.registres.l, 3),
            0x5E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                self.inst_alu_bit(value, 3);
            }
            0x5F => self.inst_alu_bit(self.registres.a, 3),
            0x60 => self.inst_alu_bit(self.registres.b, 4),
            0x61 => self.inst_alu_bit(self.registres.c, 4),
            0x62 => self.inst_alu_bit(self.registres.d, 4),
            0x63 => self.inst_alu_bit(self.registres.e, 4),
            0x64 => self.inst_alu_bit(self.registres.h, 4),
            0x65 => self.inst_alu_bit(self.registres.l, 4),
            0x66 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                self.inst_alu_bit(value, 4);
            }
            0x67 => self.inst_alu_bit(self.registres.a, 4),
            0x68 => self.inst_alu_bit(self.registres.b, 5),
            0x69 => self.inst_alu_bit(self.registres.c, 5),
            0x6A => self.inst_alu_bit(self.registres.d, 5),
            0x6B => self.inst_alu_bit(self.registres.e, 5),
            0x6C => self.inst_alu_bit(self.registres.h, 5),
            0x6D => self.inst_alu_bit(self.registres.l, 5),
            0x6E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                self.inst_alu_bit(value, 5);
            }
            0x6F => self.inst_alu_bit(self.registres.a, 5),
            0x70 => self.inst_alu_bit(self.registres.b, 6),
            0x71 => self.inst_alu_bit(self.registres.c, 6),
            0x72 => self.inst_alu_bit(self.registres.d, 6),
            0x73 => self.inst_alu_bit(self.registres.e, 6),
            0x74 => self.inst_alu_bit(self.registres.h, 6),
            0x75 => self.inst_alu_bit(self.registres.l, 6),
            0x76 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                self.inst_alu_bit(value, 6);
            }
            0x77 => self.inst_alu_bit(self.registres.a, 6),
            0x78 => self.inst_alu_bit(self.registres.b, 7),
            0x79 => self.inst_alu_bit(self.registres.c, 7),
            0x7A => self.inst_alu_bit(self.registres.d, 7),
            0x7B => self.inst_alu_bit(self.registres.e, 7),
            0x7C => self.inst_alu_bit(self.registres.h, 7),
            0x7D => self.inst_alu_bit(self.registres.l, 7),
            0x7E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                self.inst_alu_bit(value, 7);
            }
            0x7F => self.inst_alu_bit(self.registres.a, 7),
            0x80 => self.registres.b = self.inst_alu_res(self.registres.b, 0),
            0x81 => self.registres.c = self.inst_alu_res(self.registres.c, 0),
            0x82 => self.registres.d = self.inst_alu_res(self.registres.d, 0),
            0x83 => self.registres.e = self.inst_alu_res(self.registres.e, 0),
            0x84 => self.registres.h = self.inst_alu_res(self.registres.h, 0),
            0x85 => self.registres.l = self.inst_alu_res(self.registres.l, 0),
            0x86 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_res(value, 0);
                self.set_octet_in_memoire(addr, result);
            }
            0x87 => self.registres.a = self.inst_alu_res(self.registres.a, 0),
            0x88 => self.registres.b = self.inst_alu_res(self.registres.b, 1),
            0x89 => self.registres.c = self.inst_alu_res(self.registres.c, 1),
            0x8A => self.registres.d = self.inst_alu_res(self.registres.d, 1),
            0x8B => self.registres.e = self.inst_alu_res(self.registres.e, 1),
            0x8C => self.registres.h = self.inst_alu_res(self.registres.h, 1),
            0x8D => self.registres.l = self.inst_alu_res(self.registres.l, 1),
            0x8E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_res(value, 1);
                self.set_octet_in_memoire(addr, result);
            }
            0x8F => self.registres.a = self.inst_alu_res(self.registres.a, 1),
            0x90 => self.registres.b = self.inst_alu_res(self.registres.b, 2),
            0x91 => self.registres.c = self.inst_alu_res(self.registres.c, 2),
            0x92 => self.registres.d = self.inst_alu_res(self.registres.d, 2),
            0x93 => self.registres.e = self.inst_alu_res(self.registres.e, 2),
            0x94 => self.registres.h = self.inst_alu_res(self.registres.h, 2),
            0x95 => self.registres.l = self.inst_alu_res(self.registres.l, 2),
            0x96 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_res(value, 2);
                self.set_octet_in_memoire(addr, result);
            }
            0x97 => self.registres.a = self.inst_alu_res(self.registres.a, 2),
            0x98 => self.registres.b = self.inst_alu_res(self.registres.b, 3),
            0x99 => self.registres.c = self.inst_alu_res(self.registres.c, 3),
            0x9A => self.registres.d = self.inst_alu_res(self.registres.d, 3),
            0x9B => self.registres.e = self.inst_alu_res(self.registres.e, 3),
            0x9C => self.registres.h = self.inst_alu_res(self.registres.h, 3),
            0x9D => self.registres.l = self.inst_alu_res(self.registres.l, 3),
            0x9E => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_res(value, 3);
                self.set_octet_in_memoire(addr, result);
            }
            0x9F => self.registres.a = self.inst_alu_res(self.registres.a, 3),
            0xA0 => self.registres.b = self.inst_alu_res(self.registres.b, 4),
            0xA1 => self.registres.c = self.inst_alu_res(self.registres.c, 4),
            0xA2 => self.registres.d = self.inst_alu_res(self.registres.d, 4),
            0xA3 => self.registres.e = self.inst_alu_res(self.registres.e, 4),
            0xA4 => self.registres.h = self.inst_alu_res(self.registres.h, 4),
            0xA5 => self.registres.l = self.inst_alu_res(self.registres.l, 4),
            0xA6 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_res(value, 4);
                self.set_octet_in_memoire(addr, result);
            }
            0xA7 => self.registres.a = self.inst_alu_res(self.registres.a, 4),
            0xA8 => self.registres.b = self.inst_alu_res(self.registres.b, 5),
            0xA9 => self.registres.c = self.inst_alu_res(self.registres.c, 5),
            0xAA => self.registres.d = self.inst_alu_res(self.registres.d, 5),
            0xAB => self.registres.e = self.inst_alu_res(self.registres.e, 5),
            0xAC => self.registres.h = self.inst_alu_res(self.registres.h, 5),
            0xAD => self.registres.l = self.inst_alu_res(self.registres.l, 5),
            0xAE => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_res(value, 5);
                self.set_octet_in_memoire(addr, result);
            }
            0xAF => self.registres.a = self.inst_alu_res(self.registres.a, 5),
            0xB0 => self.registres.b = self.inst_alu_res(self.registres.b, 6),
            0xB1 => self.registres.c = self.inst_alu_res(self.registres.c, 6),
            0xB2 => self.registres.d = self.inst_alu_res(self.registres.d, 6),
            0xB3 => self.registres.e = self.inst_alu_res(self.registres.e, 6),
            0xB4 => self.registres.h = self.inst_alu_res(self.registres.h, 6),
            0xB5 => self.registres.l = self.inst_alu_res(self.registres.l, 6),
            0xB6 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_res(value, 6);
                self.set_octet_in_memoire(addr, result);
            }
            0xB7 => self.registres.a = self.inst_alu_res(self.registres.a, 6),
            0xB8 => self.registres.b = self.inst_alu_res(self.registres.b, 7),
            0xB9 => self.registres.c = self.inst_alu_res(self.registres.c, 7),
            0xBA => self.registres.d = self.inst_alu_res(self.registres.d, 7),
            0xBB => self.registres.e = self.inst_alu_res(self.registres.e, 7),
            0xBC => self.registres.h = self.inst_alu_res(self.registres.h, 7),
            0xBD => self.registres.l = self.inst_alu_res(self.registres.l, 7),
            0xBE => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_res(value, 7);
                self.set_octet_in_memoire(addr, result);
            }
            0xBF => self.registres.a = self.inst_alu_res(self.registres.a, 7),
            0xC0 => self.registres.b = self.inst_alu_set(self.registres.b, 0),
            0xC1 => self.registres.c = self.inst_alu_set(self.registres.c, 0),
            0xC2 => self.registres.d = self.inst_alu_set(self.registres.d, 0),
            0xC3 => self.registres.e = self.inst_alu_set(self.registres.e, 0),
            0xC4 => self.registres.h = self.inst_alu_set(self.registres.h, 0),
            0xC5 => self.registres.l = self.inst_alu_set(self.registres.l, 0),
            0xC6 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_set(value, 0);
                self.set_octet_in_memoire(addr, result);
            }
            0xC7 => self.registres.a = self.inst_alu_set(self.registres.a, 0),
            0xC8 => self.registres.b = self.inst_alu_set(self.registres.b, 1),
            0xC9 => self.registres.c = self.inst_alu_set(self.registres.c, 1),
            0xCA => self.registres.d = self.inst_alu_set(self.registres.d, 1),
            0xCB => self.registres.e = self.inst_alu_set(self.registres.e, 1),
            0xCC => self.registres.h = self.inst_alu_set(self.registres.h, 1),
            0xCD => self.registres.l = self.inst_alu_set(self.registres.l, 1),
            0xCE => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_set(value, 1);
                self.set_octet_in_memoire(addr, result);
            }
            0xCF => self.registres.a = self.inst_alu_set(self.registres.a, 1),
            0xD0 => self.registres.b = self.inst_alu_set(self.registres.b, 2),
            0xD1 => self.registres.c = self.inst_alu_set(self.registres.c, 2),
            0xD2 => self.registres.d = self.inst_alu_set(self.registres.d, 2),
            0xD3 => self.registres.e = self.inst_alu_set(self.registres.e, 2),
            0xD4 => self.registres.h = self.inst_alu_set(self.registres.h, 2),
            0xD5 => self.registres.l = self.inst_alu_set(self.registres.l, 2),
            0xD6 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_set(value, 2);
                self.set_octet_in_memoire(addr, result);
            }
            0xD7 => self.registres.a = self.inst_alu_set(self.registres.a, 2),
            0xD8 => self.registres.b = self.inst_alu_set(self.registres.b, 3),
            0xD9 => self.registres.c = self.inst_alu_set(self.registres.c, 3),
            0xDA => self.registres.d = self.inst_alu_set(self.registres.d, 3),
            0xDB => self.registres.e = self.inst_alu_set(self.registres.e, 3),
            0xDC => self.registres.h = self.inst_alu_set(self.registres.h, 3),
            0xDD => self.registres.l = self.inst_alu_set(self.registres.l, 3),
            0xDE => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_set(value, 3);
                self.set_octet_in_memoire(addr, result);
            }
            0xDF => self.registres.a = self.inst_alu_set(self.registres.a, 3),
            0xE0 => self.registres.b = self.inst_alu_set(self.registres.b, 4),
            0xE1 => self.registres.c = self.inst_alu_set(self.registres.c, 4),
            0xE2 => self.registres.d = self.inst_alu_set(self.registres.d, 4),
            0xE3 => self.registres.e = self.inst_alu_set(self.registres.e, 4),
            0xE4 => self.registres.h = self.inst_alu_set(self.registres.h, 4),
            0xE5 => self.registres.l = self.inst_alu_set(self.registres.l, 4),
            0xE6 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_set(value, 4);
                self.set_octet_in_memoire(addr, result);
            }
            0xE7 => self.registres.a = self.inst_alu_set(self.registres.a, 4),
            0xE8 => self.registres.b = self.inst_alu_set(self.registres.b, 5),
            0xE9 => self.registres.c = self.inst_alu_set(self.registres.c, 5),
            0xEA => self.registres.d = self.inst_alu_set(self.registres.d, 5),
            0xEB => self.registres.e = self.inst_alu_set(self.registres.e, 5),
            0xEC => self.registres.h = self.inst_alu_set(self.registres.h, 5),
            0xED => self.registres.l = self.inst_alu_set(self.registres.l, 5),
            0xEE => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_set(value, 5);
                self.set_octet_in_memoire(addr, result);
            }
            0xEF => self.registres.a = self.inst_alu_set(self.registres.a, 5),
            0xF0 => self.registres.b = self.inst_alu_set(self.registres.b, 6),
            0xF1 => self.registres.c = self.inst_alu_set(self.registres.c, 6),
            0xF2 => self.registres.d = self.inst_alu_set(self.registres.d, 6),
            0xF3 => self.registres.e = self.inst_alu_set(self.registres.e, 6),
            0xF4 => self.registres.h = self.inst_alu_set(self.registres.h, 6),
            0xF5 => self.registres.l = self.inst_alu_set(self.registres.l, 6),
            0xF6 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_set(value, 6);
                self.set_octet_in_memoire(addr, result);
            }
            0xF7 => self.registres.a = self.inst_alu_set(self.registres.a, 6),
            0xF8 => self.registres.b = self.inst_alu_set(self.registres.b, 7),
            0xF9 => self.registres.c = self.inst_alu_set(self.registres.c, 7),
            0xFA => self.registres.d = self.inst_alu_set(self.registres.d, 7),
            0xFB => self.registres.e = self.inst_alu_set(self.registres.e, 7),
            0xFC => self.registres.h = self.inst_alu_set(self.registres.h, 7),
            0xFD => self.registres.l = self.inst_alu_set(self.registres.l, 7),
            0xFE => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_set(value, 7);
                self.set_octet_in_memoire(addr, result);
            }
            0xFF => self.registres.a = self.inst_alu_set(self.registres.a, 7),
        };
        CB_CYCLES[cb_code as usize]
    }
}
