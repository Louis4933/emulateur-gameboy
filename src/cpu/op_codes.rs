use crate::cpu::registres::CpuFlag;
use crate::cpu::Cpu;

// Les documents Nintendo décrivent la vitesse du CPU et des instructions en cycles de machine,
// ce document les décrit en cycles d'horloge. 1 cycle machine = 4 cycles d'horloge

//  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
const OP_CYCLES: [u32; 256] = [
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, // 0
    0, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 1
    2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 2
    2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 3
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 4
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 5
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 6
    2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 1, 1, 1, 2, 1, // 7
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 8
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 9
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // a
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // b
    2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 0, 3, 6, 2, 4, // c
    2, 3, 3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4, // d
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4, // e
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4, // f
];


// Mappage du code OP de l'UC
impl Cpu {
     pub fn execute(&mut self, op_code: u8) -> u32 {
        match op_code {
            // NOP
            0x00 => {}
            // LD BC, d16
            0x01 => {
                let value = self.get_mot_at_pc();
                self.registres.set_bc(value);
            }
            // LC (BC), A
            0x02 => {
                self.set_octet_in_memoire(self.registres.bc(), self.registres.a);
            }
            // INC BC
            0x03 => {
                let value = self.registres.bc().wrapping_add(1);
                self.registres.set_bc(value);
            }
            // INC B
            0x04 => self.registres.b = self.inst_alu_inc(self.registres.b),
            // DEC B
            0x05 => self.registres.b = self.inst_alu_dec(self.registres.b),
            // LD B, d8
            0x06 => self.registres.b = self.get_octet_at_pc(),
            // RLCA
            0x07 => {
                self.registres.a = self.inst_alu_rlc(self.registres.a);
                // Z flag should be unset (set to false)
                self.registres.set_flag_zero(CpuFlag::ZERO, false);
            }
            // LD (d16), SP
            0x08 => {
                let addr = self.get_mot_at_pc();
                self.set_mot_in_memoire(addr, self.registres.sp);
            }
            // ADD HL, BC
            0x09 => self.inst_alu_add_hl(self.registres.bc()),
            // LD A, (BC)
            0x0A => self.registres.a = self.get_octet_in_memoire(self.registres.bc()),
            // DEC BC
            0x0B => {
                let value = self.registres.bc().wrapping_sub(1);
                self.registres.set_bc(value)
            }
            // INC C
            0x0C => self.registres.c = self.inst_alu_inc(self.registres.c),
            // DEC C
            0x0D => self.registres.c = self.inst_alu_dec(self.registres.c),
            // LD C, d8
            0x0E => self.registres.c = self.get_octet_at_pc(),
            // RRCA
            0x0F => {
                self.registres.a = self.inst_alu_rrc(self.registres.a);
                // Z flag should be unset (set to false)
                self.registres.set_flag_zero(CpuFlag::ZERO, false);
            }
            // STOP
            0x10 => self.stopped = true,
            // LD DE, d16
            0x11 => {
                let value = self.get_mot_at_pc();
                self.registres.set_de(value);
            }
            // LD (DE), A
            0x12 => self.set_octet_in_memoire(self.registres.de(), self.registres.a),
            // INC DE
            0x13 => {
                let value = self.registres.de().wrapping_add(1);
                self.registres.set_de(value);
            }
            // INC D
            0x14 => self.registres.d = self.inst_alu_inc(self.registres.d),
            // DEC D
            0x15 => self.registres.d = self.inst_alu_dec(self.registres.d),
            // LD D, d8
            0x16 => self.registres.d = self.get_octet_at_pc(),
            // RLA
            0x17 => {
                self.registres.a = self.inst_alu_rl(self.registres.a);
                // Z flag should be unset (set to false)
                self.registres.set_flag_zero(CpuFlag::ZERO, false);
            }
            // JR r8
            0x18 => {
                let n = self.get_octet_at_pc();
                self.inst_alu_jr(n);
            }
            // ADD HL, DE
            0x19 => self.inst_alu_add_hl(self.registres.de()),
            // LD A, (DE)
            0x1A => self.registres.a = self.get_octet_in_memoire(self.registres.de()),
            // DEC DE
            0x1B => {
                let value = self.registres.de().wrapping_sub(1);
                self.registres.set_de(value);
            }
            // INC E
            0x1C => self.registres.e = self.inst_alu_inc(self.registres.e),
            // DEC E
            0x1D => self.registres.e = self.inst_alu_dec(self.registres.e),
            // LD E, d8
            0x1E => self.registres.e = self.get_octet_at_pc(),
            // RRA
            0x1F => {
                self.registres.a = self.inst_alu_rr(self.registres.a);
                // Z flag should be unset (set to false)
                self.registres.set_flag_zero(CpuFlag::ZERO, false);
            }
            // JR NZ, r8
            0x20 => {
                let n = self.get_octet_at_pc();
                // Not Zero
                if !self.registres.has_flag(CpuFlag::ZERO) {
                    self.inst_alu_jr(n);
                }
            }
            // LD HL, d16
            0x21 => {
                let value = self.get_mot_at_pc();
                self.registres.set_hl(value);
            }
            // LD (HL+), A
            0x22 => {
                let addr = self.registres.hl_then_inc();
                self.set_octet_in_memoire(addr, self.registres.a);
            }
            // INC HL
            0x23 => {
                let value = self.registres.hl().wrapping_add(1);
                self.registres.set_hl(value);
            }
            // INC H
            0x24 => self.registres.h = self.inst_alu_inc(self.registres.h),
            // DEC H
            0x25 => self.registres.h = self.inst_alu_dec(self.registres.h),
            // LD H, d8
            0x26 => self.registres.h = self.get_octet_at_pc(),
            // DAA
            0x27 => self.inst_alu_daa(),
            // JR Z, r8
            0x28 => {
                let n = self.get_octet_at_pc();
                // Zero
                if self.registres.has_flag(CpuFlag::ZERO) {
                    self.inst_alu_jr(n);
                }
            }
            // ADD HL, HL
            0x29 => self.inst_alu_add_hl(self.registres.hl()),
            // LD A, (HL+)
            0x2A => {
                let addr = self.registres.hl_then_inc();
                self.registres.a = self.get_octet_in_memoire(addr);
            }
            // DEC HL
            0x2B => {
                let value = self.registres.hl().wrapping_sub(1);
                self.registres.set_hl(value);
            }
            // INC L
            0x2C => self.registres.l = self.inst_alu_inc(self.registres.l),
            // DEC L
            0x2D => self.registres.l = self.inst_alu_dec(self.registres.l),
            // LD L, d8
            0x2E => self.registres.l = self.get_octet_at_pc(),
            // CPL
            0x2F => self.inst_alu_cpl(),
            // JR NC, r8
            0x30 => {
                let n = self.get_octet_at_pc();
                // Not Carry
                if !self.registres.has_flag(CpuFlag::CARRY) {
                    self.inst_alu_jr(n);
                }
            }
            // LD SP, d16
            0x31 => self.registres.sp = self.get_mot_at_pc(),
            // LD (HL-), A
            0x32 => {
                let addr = self.registres.hl_then_dec();
                self.set_octet_in_memoire(addr, self.registres.a);
            }
            // INC SP
            0x33 => {
                let value = self.registres.sp.wrapping_add(1);
                self.registres.sp = value;
            }
            // INC (HL)
            0x34 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_inc(value);
                self.set_octet_in_memoire(addr, result);
            }
            // DEC (HL)
            0x35 => {
                let addr = self.registres.hl();
                let value = self.get_octet_in_memoire(addr);
                let result = self.inst_alu_dec(value);
                self.set_octet_in_memoire(addr, result);
            }
            // LD (HL), d8
            0x36 => {
                let addr = self.registres.hl();
                let value = self.get_octet_at_pc();
                self.set_octet_in_memoire(addr, value);
            }
            // SCF
            0x37 => self.inst_alu_scf(),
            // JR C, r8
            0x38 => {
                let n = self.get_octet_at_pc();
                // Carry
                if self.registres.has_flag(CpuFlag::CARRY) {
                    self.inst_alu_jr(n);
                }
            }
            // ADD HL, SP
            0x39 => self.inst_alu_add_hl(self.registres.sp),
            // LD A, (HL-)
            0x3A => {
                let addr = self.registres.hl_then_dec();
                self.registres.a = self.get_octet_in_memoire(addr);
            }
            // DEC SP
            0x3B => {
                let value = self.registres.sp.wrapping_sub(1);
                self.registres.sp = value;
            }
            // INC A
            0x3C => self.registres.a = self.inst_alu_inc(self.registres.a),
            // DEC A
            0x3D => self.registres.a = self.inst_alu_dec(self.registres.a),
            // LD A, d8
            0x3E => self.registres.a = self.get_octet_at_pc(),
            // CCF
            0x3F => self.inst_alu_ccf(),
            // LD B, B
            0x40 => {}
            // LB B, C
            0x41 => self.registres.b = self.registres.c,
            // LD B, D
            0x42 => self.registres.b = self.registres.d,
            // LD B, E
            0x43 => self.registres.b = self.registres.e,
            // LD B, H
            0x44 => self.registres.b = self.registres.h,
            // LD B, L
            0x45 => self.registres.b = self.registres.l,
            // LD B, (HL)
            0x46 => self.registres.b = self.get_octet_in_memoire(self.registres.hl()),
            // LD B, A
            0x47 => self.registres.b = self.registres.a,
            // LD C, B
            0x48 => self.registres.c = self.registres.b,
            // LD C, C
            0x49 => {}
            // LD C, D
            0x4A => self.registres.c = self.registres.d,
            // LD C, E
            0x4B => self.registres.c = self.registres.e,
            // LD C, H
            0x4C => self.registres.c = self.registres.h,
            // LD C, L
            0x4D => self.registres.c = self.registres.l,
            // LD C, (HL)
            0x4E => self.registres.c = self.get_octet_in_memoire(self.registres.hl()),
            // LD C, A
            0x4F => self.registres.c = self.registres.a,
            // LD D, B
            0x50 => self.registres.d = self.registres.b,
            // LD D, C
            0x51 => self.registres.d = self.registres.c,
            // LD D, D
            0x52 => {}
            // LD D, E
            0x53 => self.registres.d = self.registres.e,
            // LD D, H
            0x54 => self.registres.d = self.registres.h,
            // LD D, L
            0x55 => self.registres.d = self.registres.l,
            // LD D, (HL)
            0x56 => self.registres.d = self.get_octet_in_memoire(self.registres.hl()),
            // LD D, A
            0x57 => self.registres.d = self.registres.a,
            // LD E, B
            0x58 => self.registres.e = self.registres.b,
            // LD E, C
            0x59 => self.registres.e = self.registres.c,
            // LD E, D
            0x5A => self.registres.e = self.registres.d,
            // LD E, E
            0x5B => {}
            // LD E, H
            0x5C => self.registres.e = self.registres.h,
            // LD E, L
            0x5D => self.registres.e = self.registres.l,
            // LD E, (HL)
            0x5E => self.registres.e = self.get_octet_in_memoire(self.registres.hl()),
            // LD E, A
            0x5F => self.registres.e = self.registres.a,
            // LD H, B
            0x60 => self.registres.h = self.registres.b,
            // LD H, C
            0x61 => self.registres.h = self.registres.c,
            // LD H, D
            0x62 => self.registres.h = self.registres.d,
            // LD H, E
            0x63 => self.registres.h = self.registres.e,
            // LD H, H
            0x64 => {}
            // LD H, L
            0x65 => self.registres.h = self.registres.l,
            // LD H, (HL)
            0x66 => self.registres.h = self.get_octet_in_memoire(self.registres.hl()),
            // LD H, A
            0x67 => self.registres.h = self.registres.a,
            // LD L, B
            0x68 => self.registres.l = self.registres.b,
            // LD L, C
            0x69 => self.registres.l = self.registres.c,
            // LD L, D
            0x6A => self.registres.l = self.registres.d,
            // LD L, E
            0x6B => self.registres.l = self.registres.e,
            // LD L, H
            0x6C => self.registres.l = self.registres.h,
            // LD L, L
            0x6D => {}
            // LD L, (HL)
            0x6E => self.registres.l = self.get_octet_in_memoire(self.registres.hl()),
            // LD L, A
            0x6F => self.registres.l = self.registres.a,
            // LD (HL), B
            0x70 => self.set_octet_in_memoire(self.registres.hl(), self.registres.b),
            // LD (HL), C
            0x71 => self.set_octet_in_memoire(self.registres.hl(), self.registres.c),
            // LD (HL), D
            0x72 => self.set_octet_in_memoire(self.registres.hl(), self.registres.d),
            // LD (HL), E
            0x73 => self.set_octet_in_memoire(self.registres.hl(), self.registres.e),
            // LD (HL), H
            0x74 => self.set_octet_in_memoire(self.registres.hl(), self.registres.h),
            // LD (HL), L
            0x75 => self.set_octet_in_memoire(self.registres.hl(), self.registres.l),
            // HALT
            0x76 => self.halted = true,
            // LD (HL), A
            0x77 => self.set_octet_in_memoire(self.registres.hl(), self.registres.a),
            // LD A, B
            0x78 => self.registres.a = self.registres.b,
            // LD A, C
            0x79 => self.registres.a = self.registres.c,
            // LD A, D
            0x7A => self.registres.a = self.registres.d,
            // LD A, E
            0x7B => self.registres.a = self.registres.e,
            // LD A, H
            0x7C => self.registres.a = self.registres.h,
            // LD A, L
            0x7D => self.registres.a = self.registres.l,
            // LD A, (HL)
            0x7E => self.registres.a = self.get_octet_in_memoire(self.registres.hl()),
            // LD A, A
            0x7F => {}
            // ADD A, B
            0x80 => self.inst_alu_add(self.registres.b),
            // ADD A, C
            0x81 => self.inst_alu_add(self.registres.c),
            // ADD A, D
            0x82 => self.inst_alu_add(self.registres.d),
            // ADD A, E
            0x83 => self.inst_alu_add(self.registres.e),
            // ADD A, H
            0x84 => self.inst_alu_add(self.registres.h),
            // ADD A, L
            0x85 => self.inst_alu_add(self.registres.l),
            // ADD A, (HL)
            0x86 => {
                let value = self.get_octet_in_memoire(self.registres.hl());
                self.inst_alu_add(value);
            }
            // ADD A, A
            0x87 => self.inst_alu_add(self.registres.a),
            // ADC A, B
            0x88 => self.inst_alu_adc(self.registres.b),
            // ADC A, C
            0x89 => self.inst_alu_adc(self.registres.c),
            // ADC A, D
            0x8A => self.inst_alu_adc(self.registres.d),
            // ADC A, E
            0x8B => self.inst_alu_adc(self.registres.e),
            // ADC A, H
            0x8C => self.inst_alu_adc(self.registres.h),
            // ADC A, L
            0x8D => self.inst_alu_adc(self.registres.l),
            // ADC A, (HL)
            0x8E => {
                let value = self.get_octet_in_memoire(self.registres.hl());
                self.inst_alu_adc(value);
            }
            // ADC A, A
            0x8F => self.inst_alu_adc(self.registres.a),
            // SUB B
            0x90 => self.inst_alu_sub(self.registres.b),
            // SUB C
            0x91 => self.inst_alu_sub(self.registres.c),
            // SUB D
            0x92 => self.inst_alu_sub(self.registres.d),
            // SUB E
            0x93 => self.inst_alu_sub(self.registres.e),
            // SUB H
            0x94 => self.inst_alu_sub(self.registres.h),
            // SUB L
            0x95 => self.inst_alu_sub(self.registres.l),
            // SUB (HL)
            0x96 => {
                let value = self.get_octet_in_memoire(self.registres.hl());
                self.inst_alu_sub(value);
            }
            // SUB A
            0x97 => self.inst_alu_sub(self.registres.a),
            // SBC A, B
            0x98 => self.inst_alu_sbc(self.registres.b),
            // SBC A, C
            0x99 => self.inst_alu_sbc(self.registres.c),
            // SBC A, D
            0x9A => self.inst_alu_sbc(self.registres.d),
            // SBC A, E
            0x9B => self.inst_alu_sbc(self.registres.e),
            // SBC A, H
            0x9C => self.inst_alu_sbc(self.registres.h),
            // SBC A, L
            0x9D => self.inst_alu_sbc(self.registres.l),
            // SBC A, (HL)
            0x9E => {
                let value = self.get_octet_in_memoire(self.registres.hl());
                self.inst_alu_sbc(value);
            }
            // SBC A, A
            0x9F => self.inst_alu_sbc(self.registres.a),
            // AND B
            0xA0 => self.inst_alu_and(self.registres.b),
            // AND C
            0xA1 => self.inst_alu_and(self.registres.c),
            // AND D
            0xA2 => self.inst_alu_and(self.registres.d),
            // AND E
            0xA3 => self.inst_alu_and(self.registres.e),
            // AND H
            0xA4 => self.inst_alu_and(self.registres.h),
            // AND L
            0xA5 => self.inst_alu_and(self.registres.l),
            // AND (HL)
            0xA6 => {
                let value = self.get_octet_in_memoire(self.registres.hl());
                self.inst_alu_and(value);
            }
            // AND A
            0xA7 => self.inst_alu_and(self.registres.a),
            // XOR B
            0xA8 => self.inst_alu_xor(self.registres.b),
            // XOR C
            0xA9 => self.inst_alu_xor(self.registres.c),
            // XOR D
            0xAA => self.inst_alu_xor(self.registres.d),
            // XOR E
            0xAB => self.inst_alu_xor(self.registres.e),
            // XOR H
            0xAC => self.inst_alu_xor(self.registres.h),
            // XOR L
            0xAD => self.inst_alu_xor(self.registres.l),
            // XOR (HL)
            0xAE => {
                let value = self.get_octet_in_memoire(self.registres.hl());
                self.inst_alu_xor(value);
            }
            // XOR A
            0xAF => self.inst_alu_xor(self.registres.a),
            // OR B
            0xB0 => self.inst_alu_or(self.registres.b),
            // OR C
            0xB1 => self.inst_alu_or(self.registres.c),
            // OR D
            0xB2 => self.inst_alu_or(self.registres.d),
            // OR E
            0xB3 => self.inst_alu_or(self.registres.e),
            // OR H
            0xB4 => self.inst_alu_or(self.registres.h),
            // OR L
            0xB5 => self.inst_alu_or(self.registres.l),
            // OR (HL)
            0xB6 => {
                let value = self.get_octet_in_memoire(self.registres.hl());
                self.inst_alu_or(value);
            }
            // OR A
            0xB7 => self.inst_alu_or(self.registres.a),
            // CP B
            0xB8 => self.inst_alu_cp(self.registres.b),
            // CP C
            0xB9 => self.inst_alu_cp(self.registres.c),
            // CP D
            0xBA => self.inst_alu_cp(self.registres.d),
            // CP E
            0xBB => self.inst_alu_cp(self.registres.e),
            // CP H
            0xBC => self.inst_alu_cp(self.registres.h),
            // CP L
            0xBD => self.inst_alu_cp(self.registres.l),
            // CP (HL)
            0xBE => {
                let value = self.get_octet_in_memoire(self.registres.hl());
                self.inst_alu_cp(value);
            }
            // CP A
            0xBF => self.inst_alu_cp(self.registres.a),
            // RET NZ
            0xC0 => {
                // Not Zero
                if !self.registres.has_flag(CpuFlag::ZERO) {
                    self.registres.pc = self.pop_stack();
                }
            }
            // POP BC
            0xC1 => {
                let value = self.pop_stack();
                self.registres.set_bc(value);
            }
            // JP NZ, a16
            0xC2 => {
                let pc = self.get_mot_at_pc();
                if !self.registres.has_flag(CpuFlag::ZERO) {
                    self.registres.pc = pc;
                }
            }
            // JP a16
            0xC3 => self.registres.pc = self.get_mot_at_pc(),
            // CALL NZ, a16
            0xC4 => {
                let n = self.get_mot_at_pc();
                if !self.registres.has_flag(CpuFlag::ZERO) {
                    self.add_to_stack(self.registres.pc);
                    self.registres.pc = n;
                }
            }
            // PUSH BC
            0xC5 => self.add_to_stack(self.registres.bc()),
            // ADD A, d8
            0xC6 => {
                let value = self.get_octet_at_pc();
                self.inst_alu_add(value);
            }
            // RST 00H
            0xC7 => {
                self.add_to_stack(self.registres.pc);
                self.registres.pc = 0x00;
            }
            // RET Z
            0xC8 => {
                // Not Zero
                if self.registres.has_flag(CpuFlag::ZERO) {
                    self.registres.pc = self.pop_stack();
                }
            }
            // RET
            0xC9 => self.registres.pc = self.pop_stack(),
            // JP Z, a16
            0xCA => {
                let pc = self.get_mot_at_pc();
                if self.registres.has_flag(CpuFlag::ZERO) {
                    self.registres.pc = pc;
                }
            }
            // PREFIX CB
            0xCB => {
                let cb_code = self.get_octet_at_pc();
                return self.execute_cb(cb_code);
            }
            // CALL Z, a16
            0xCC => {
                let n = self.get_mot_at_pc();
                if self.registres.has_flag(CpuFlag::ZERO) {
                    self.add_to_stack(self.registres.pc);
                    self.registres.pc = n;
                }
            }
            // CALL a16
            0xCD => {
                let n = self.get_mot_at_pc();
                self.add_to_stack(self.registres.pc);
                self.registres.pc = n;
            }
            // ADC A, d8
            0xCE => {
                let value = self.get_octet_at_pc();
                self.inst_alu_adc(value);
            }
            // RST 08H
            0xCF => {
                self.add_to_stack(self.registres.pc);
                self.registres.pc = 0x08;
            }
            // RET NC
            0xD0 => {
                // Not Carry
                if !self.registres.has_flag(CpuFlag::CARRY) {
                    self.registres.pc = self.pop_stack();
                }
            }
            // POP DE
            0xD1 => {
                let value = self.pop_stack();
                self.registres.set_de(value);
            }
            // JP NC, a16
            0xD2 => {
                let pc = self.get_mot_at_pc();
                if !self.registres.has_flag(CpuFlag::CARRY) {
                    self.registres.pc = pc;
                }
            }
            // Not Valid
            0xD3 => panic!("cpu: invalid op code 0xD3"),
            // CALL NC, a16
            0xD4 => {
                let n = self.get_mot_at_pc();
                if !self.registres.has_flag(CpuFlag::CARRY) {
                    self.add_to_stack(self.registres.pc);
                    self.registres.pc = n;
                }
            }
            // PUSH DE
            0xD5 => self.add_to_stack(self.registres.de()),
            // SUB d8
            0xD6 => {
                let value = self.get_octet_at_pc();
                self.inst_alu_sub(value);
            }
            // RST 10H
            0xD7 => {
                self.add_to_stack(self.registres.pc);
                self.registres.pc = 0x10;
            }
            // RET C
            0xD8 => {
                // Carry
                if self.registres.has_flag(CpuFlag::CARRY) {
                    self.registres.pc = self.pop_stack();
                }
            }
            // RETI
            0xD9 => {
                self.registres.pc = self.pop_stack();
                self.ei = true;
            }
            // JP C, a16
            0xDA => {
                let pc = self.get_mot_at_pc();
                if self.registres.has_flag(CpuFlag::CARRY) {
                    self.registres.pc = pc;
                }
            }
            // Not Valid
            0xDB => panic!("cpu: invalid op code 0xDB"),
            // CALL C, a16
            0xDC => {
                let n = self.get_mot_at_pc();
                if self.registres.has_flag(CpuFlag::CARRY) {
                    self.add_to_stack(self.registres.pc);
                    self.registres.pc = n;
                }
            }
            // Not Valid
            0xDD => panic!("cpu: invalid op code 0xDD"),
            // SBC A, d8
            0xDE => {
                let value = self.get_octet_at_pc();
                self.inst_alu_sbc(value);
            }
            // RST 18H
            0xDF => {
                self.add_to_stack(self.registres.pc);
                self.registres.pc = 0x18;
            }
            // LDH (a8), A
            0xE0 => {
                let addr = 0xFF00 | (self.get_octet_at_pc() as u16);
                self.set_octet_in_memoire(addr, self.registres.a);
            }
            // POP HL
            0xE1 => {
                let value = self.pop_stack();
                self.registres.set_hl(value);
            }
            // LD (C), A
            0xE2 => {
                let addr = 0xFF00 | (self.registres.c as u16);
                self.set_octet_in_memoire(addr, self.registres.a);
            }
            // Not Valid
            0xE3 => panic!("cpu: invalid op code 0xE3"),
            // Not Valid
            0xE4 => panic!("cpu: invalid op code 0xE4"),
            // PUSH HL
            0xE5 => self.add_to_stack(self.registres.hl()),
            // AND d8
            0xE6 => {
                let value = self.get_octet_at_pc();
                self.inst_alu_and(value);
            }
            // RST 20H
            0xE7 => {
                self.add_to_stack(self.registres.pc);
                self.registres.pc = 0x20;
            }
            // ADD SP, r8
            0xE8 => {
                let value = self.get_octet_at_pc();
                self.inst_alu_add_sp(value);
            }
            // JP (HL)
            0xE9 => self.registres.pc = self.registres.hl(),
            // LD (a16), A
            0xEA => {
                let addr = self.get_mot_at_pc();
                self.set_octet_in_memoire(addr, self.registres.a);
            }
            // Not Valid
            0xEB => panic!("cpu: invalid op code 0xEB"),
            // Not Valid
            0xEC => panic!("cpu: invalid op code 0xEC"),
            // Not Valid
            0xED => panic!("cpu: invalid op code 0xED"),
            // XOR d8
            0xEE => {
                let value = self.get_octet_at_pc();
                self.inst_alu_xor(value);
            }
            // RST 28H
            0xEF => {
                self.add_to_stack(self.registres.pc);
                self.registres.pc = 0x28;
            }
            // LDH A, (a8)
            0xF0 => {
                let addr = 0xFF00 | (self.get_octet_at_pc() as u16);
                self.registres.a = self.get_octet_in_memoire(addr);
            }
            // POP AF
            0xF1 => {
                let value = self.pop_stack();
                self.registres.set_af(value);
            }
            // LD A, (C)
            0xF2 => {
                let addr = 0xFF00 | (self.registres.c as u16);
                self.registres.a = self.get_octet_in_memoire(addr);
            }
            // DI
            0xF3 => self.ei = false,
            // Not Valid
            0xF4 => panic!("cpu: invalid op code 0xF4"),
            // PUSH AF
            0xF5 => self.add_to_stack(self.registres.af()),
            // OR d8
            0xF6 => {
                let value = self.get_octet_at_pc();
                self.inst_alu_or(value);
            }
            // RST 30H
            0xF7 => {
                self.add_to_stack(self.registres.pc);
                self.registres.pc = 0x30;
            }
            // LD HL, SP+r8
            0xF8 => {
                let sp = self.registres.sp;
                let value = i16::from(self.get_octet_at_pc() as i8) as u16;
                self.registres.set_flag_zero(CpuFlag::ZERO, false);
                self.registres.set_flag_zero(CpuFlag::SUB, false);
                self.registres
                    .set_flag_zero(CpuFlag::HALF_CARRY, (sp & 0x000F) + (value & 0x000F) > 0x000F);
                self.registres
                    .set_flag_zero(CpuFlag::CARRY, (sp & 0x00FF) + (value & 0x00FF) > 0x00FF);
                self.registres.set_hl(sp.wrapping_add(value));
            }
            // LD SP, HL
            0xF9 => self.registres.sp = self.registres.hl(),
            // LD A, (a16)
            0xFA => {
                let addr = self.get_mot_at_pc();
                self.registres.a = self.get_octet_in_memoire(addr);
            }
            // EI
            0xFB => self.ei = true,
            // Not Valid
            0xFC => panic!("cpu: invalid op code 0xFC"),
            // Not Valid
            0xFD => panic!("cpu: invalid op code 0xFD"),
            // CP d8
            0xFE => {
                let value = self.get_octet_at_pc();
                self.inst_alu_cp(value);
            }
            // RST 38H
            0xFF => {
                self.add_to_stack(self.registres.pc);
                self.registres.pc = 0x38;
            }
        };
        let ecycle = match op_code {
            0x20 | 0x30 => u32::from(!self.registres.has_flag(CpuFlag::ZERO)),
            0x28 | 0x38 => u32::from(self.registres.has_flag(CpuFlag::ZERO)),
            0xC0 | 0xD0 => {
                if self.registres.has_flag(CpuFlag::ZERO) {
                    0x00
                } else {
                    0x03
                }
            }
            0xC8 | 0xCC | 0xD8 | 0xDC => {
                if self.registres.has_flag(CpuFlag::ZERO) {
                    0x03
                } else {
                    0x00
                }
            }
            0xC2 | 0xD2 => u32::from(!self.registres.has_flag(CpuFlag::ZERO)),
            0xCA | 0xDA => u32::from(self.registres.has_flag(CpuFlag::ZERO)),
            0xC4 | 0xD4 => {
                if self.registres.has_flag(CpuFlag::ZERO) {
                    0x00
                } else {
                    0x03
                }
            }
            _ => 0x00,
        };
        OP_CYCLES[op_code as usize] + ecycle
    }
}
