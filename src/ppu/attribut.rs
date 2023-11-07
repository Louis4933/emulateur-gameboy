// Table d'attributs des sprites VRAM (OAM)
// Octet3 - Attributs/Flags

pub struct Attribut {
    pub palette : u8,
    pub priority : u8,
    pub flip_x : bool,
    pub flip_y : bool,
    pub palette_number : u8,
}

impl From<u8> for Attribut {
    fn from(byte: u8) -> Self {
        Attribut {
            priority: value & (1 << 4) != 0,
            y_flip: value & (1 << 3) != 0,
            x_flip: value & (1 << 2) != 0,
            palette_number: value as usize & (1 << 1)
        }
    }
}