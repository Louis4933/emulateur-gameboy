use crate::cpu::registres::CpuFlag;
use crate::cpu::Cpu;

// Instructions CPU
impl Cpu {
    // Incrémente la valeur du registre.
    pub fn inst_alu_inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres
            .set_flag_zero(CpuFlag::HALF_CARRY, (value & 0x0F) + 0x01 > 0x0F);
        result
    }

    // Décrémente la valeur du registre.
    pub fn inst_alu_dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, true);
        self.registres
            .set_flag_zero(CpuFlag::HALF_CARRY, value.trailing_zeros() >= 4);
        result
    }

    // Rotation de la valeur vers la gauche.
    pub fn inst_alu_rl(&mut self, value: u8) -> u8 {
        let has_carry = (value & 0x80) >> 7 == 0x01;
        let result = (value << 1) + (self.registres.has_flag(CpuFlag::CARRY) as u8);
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, has_carry);
        result
    }

    // Rotation de la valeur vers la gauche.
    pub fn inst_alu_rlc(&mut self, value: u8) -> u8 {
        let has_carry = (value & 0x80) >> 7 == 0x01;
        let result = (value << 1) | (has_carry as u8);
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, has_carry);
        result
    }

    // Rotation de la valeur vers la gauche.
    pub fn inst_alu_rr(&mut self, value: u8) -> u8 {
        let has_carry = value & 0x01 == 0x01;
        let result = if self.registres.has_flag(CpuFlag::CARRY) {
            0x80 | (value >> 1)
        } else {
            value >> 1
        };
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, has_carry);
        result
    }

    // Rotation de la valeur vers la droite.
    pub fn inst_alu_rrc(&mut self, value: u8) -> u8 {
        let has_carry = value & 0x01 == 0x01;
        let result = if has_carry {
            0x80 | (value >> 1)
        } else {
            value >> 1
        };
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, has_carry);
        result
    }

    // Ajoute la valeur à A
    pub fn inst_alu_add(&mut self, value: u8) {
        let curr = self.registres.a;
        let result = curr.wrapping_add(value);
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres
            .set_flag_zero(CpuFlag::HALF_CARRY, (curr & 0x0F) + (value & 0x0F) > 0x0F);
        self.registres
            .set_flag_zero(CpuFlag::CARRY, (curr as u16) + (value as u16) > 0xFF);
        self.registres.a = result;
    }

    // Soustrait la valeur à A
    pub fn inst_alu_sub(&mut self, value: u8) {
        let curr = self.registres.a;
        let result = curr.wrapping_sub(value);
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, true);
        self.registres
            .set_flag_zero(CpuFlag::HALF_CARRY, (curr & 0x0F) < (value & 0x0F));
        self.registres
            .set_flag_zero(CpuFlag::CARRY, (curr as u16) < (value as u16));
        self.registres.a = result;
    }

    // Ajoute la valeur à A + Carry flag à A.
    pub fn inst_alu_adc(&mut self, value: u8) {
        let curr = self.registres.a;
        let carry = u8::from(self.registres.has_flag(CpuFlag::CARRY));
        let result = curr.wrapping_add(value).wrapping_add(carry);
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(
            CpuFlag::HALF_CARRY,
            (curr & 0x0F) + (value & 0x0F) + (carry & 0x0F) > 0x0F,
        );
        self.registres.set_flag_zero(
            CpuFlag::CARRY,
            (curr as u16) + (value as u16) + (carry as u16) > 0xFF,
        );
        self.registres.a = result;
    }

    // Ajoute la valeur à HL
    pub fn inst_alu_add_hl(&mut self, value: u16) {
        let curr = self.registres.hl();
        let result = curr.wrapping_add(value);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres
            .set_flag_zero(CpuFlag::HALF_CARRY, (curr & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
        self.registres.set_flag_zero(CpuFlag::CARRY, curr > 0xFFFF - value);
        self.registres.set_hl(result);
    }

    // Ajouter n à l'adresse actuelle et va à l'adresse.
    // n = valeur immédiate signée d'un octet
    pub fn inst_alu_jr(&mut self, n: u8) {
        let n = n as i8;
        self.registres.pc = ((u32::from(self.registres.pc) as i32) + i32::from(n)) as u16;
    }

    // Ajuste le registre A de façon à obtenir une représentation correcte du décimal codé binaire (BCD).
    pub fn inst_alu_daa(&mut self) {
        let mut a = self.registres.a;
        let mut adjust = if self.registres.has_flag(CpuFlag::CARRY) {
            0x60
        } else {
            0x00
        };
        if self.registres.has_flag(CpuFlag::HALF_CARRY) {
            adjust |= 0x06;
        };
        if !self.registres.has_flag(CpuFlag::SUB) {
            if a & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if a > 0x99 {
                adjust |= 0x60;
            };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }
        self.registres.set_flag_zero(CpuFlag::ZERO, a == 0x00);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, adjust >= 0x60);
        self.registres.a = a;
    }

    // Complémente le registre A
    pub fn inst_alu_cpl(&mut self) {
        self.registres.a = !self.registres.a;
        self.registres.set_flag_zero(CpuFlag::SUB, true);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, true);
    }

    // Set Carry flag.
    pub fn inst_alu_scf(&mut self) {
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, true);
    }

    // Si l'indicateur C est activé, il est réinitialisé.
    pub fn inst_alu_ccf(&mut self) {
        let value = !self.registres.has_flag(CpuFlag::CARRY);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, value);
    }

    // Soustrait la valeur et le carry flag de A
    pub fn inst_alu_sbc(&mut self, value: u8) {
        let curr = self.registres.a;
        let carry = u8::from(self.registres.has_flag(CpuFlag::CARRY));
        let result = curr.wrapping_sub(value).wrapping_sub(carry);
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, true);
        self.registres
            .set_flag_zero(CpuFlag::HALF_CARRY, (curr & 0x0F) < (value & 0x0F) + carry);
        self.registres.set_flag_zero(
            CpuFlag::CARRY,
            (curr as u16) < ((value as u16) + (carry as u16)),
        );
        self.registres.a = result;
    }

    // ET logique avec A, renvoie A.
    pub fn inst_alu_and(&mut self, value: u8) {
        let result = self.registres.a & value;
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, true);
        self.registres.set_flag_zero(CpuFlag::CARRY, false);
        self.registres.a = result;
    }

    // OU logique exclusif avec A, renvoie A.
    pub fn inst_alu_xor(&mut self, value: u8) {
        let result = self.registres.a ^ value;
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, false);
        self.registres.a = result;
    }

    // OU logique avec A, renvoie A.
    pub fn inst_alu_or(&mut self, value: u8) {
        let result = self.registres.a | value;
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, false);
        self.registres.a = result;
    }

    // Compare A avec la valeur.
    pub fn inst_alu_cp(&mut self, value: u8) {
        let curr = self.registres.a;
        self.inst_alu_sub(value);
        self.registres.a = curr;
    }

    // Ajoute la valeur au stack pointeur.
    pub fn inst_alu_add_sp(&mut self, value: u8) {
        let curr = self.registres.sp;
        let val = i16::from(value as i8) as u16;
        self.registres.set_flag_zero(CpuFlag::ZERO, false);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres
            .set_flag_zero(CpuFlag::HALF_CARRY, (curr & 0x000F) + (val & 0x000F) > 0x000F);
        self.registres
            .set_flag_zero(CpuFlag::CARRY, (curr & 0x00FF) + (val & 0x00FF) > 0x00FF);
        self.registres.sp = curr.wrapping_add(val);
    }

    // Décale la valeur vers la gauche dans Carry.
    pub fn inst_alu_sla(&mut self, value: u8) -> u8 {
        let has_carry = (value & 0x80) >> 7 == 0x01;
        let result = value << 1;
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, has_carry);
        result
    }

    // Décale la valeur vers la droite dans Carry.
    pub fn inst_alu_sra(&mut self, value: u8) -> u8 {
        let has_carry = value & 0x01 == 0x01;
        let result = (value >> 1) | (value & 0x80);
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, has_carry);
        result
    }

    // Décale la valeur vers la droite dans Carry. MSB set à 0.
    pub fn inst_alu_srl(&mut self, value: u8) -> u8 {
        let has_carry = value & 0x01 == 0x01;
        let result = value >> 1;
        self.registres.set_flag_zero(CpuFlag::ZERO, result == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, has_carry);
        result
    }

    // échange des objets de valeur supérieurs et inférieurs.
    pub fn inst_alu_swap(&mut self, value: u8) -> u8 {
        self.registres.set_flag_zero(CpuFlag::ZERO, value == 0x00);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, false);
        self.registres.set_flag_zero(CpuFlag::CARRY, false);
        (value >> 4) | (value << 4)
    }

    // Test du bit dans la valeur du registre.
    pub fn inst_alu_bit(&mut self, value: u8, bit: u8) {
        let result = value & (1 << bit) == 0x00;
        self.registres.set_flag_zero(CpuFlag::ZERO, result);
        self.registres.set_flag_zero(CpuFlag::SUB, false);
        self.registres.set_flag_zero(CpuFlag::HALF_CARRY, true);
    }

    // Reset bit dans la valeur du registre.
    pub fn inst_alu_res(&mut self, value: u8, bit: u8) -> u8 {
        value & !(1 << bit)
    }

    // Set bit dans la valeur du registre.
    pub fn inst_alu_set(&mut self, value: u8, bit: u8) -> u8 {
        value | (1 << bit)
    }
}
