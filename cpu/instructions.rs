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
}