pub trait Memoire {
    fn get_octet(&self, addr: u16) -> u8;

    fn set_octet(&mut self, addr: u16, value: u8);

    fn get_mot(&self, addr: u16) -> u16 {
        u16::from(self.get_octet(addr)) | (u16::from(self.get_octet(addr + 1)) << 8)
    }

    fn set_mot(&mut self, addr: u16, value: u16) {
        self.set_octet(addr, (value & 0xFF) as u8);
        self.set_octet(addr + 1, (value >> 8) as u8)
    }
}