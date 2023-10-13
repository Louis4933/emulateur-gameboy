// Table d'attributs des sprites VRAM (OAM)
// Octet3 - Attributs/Flags

pub struct Attribut {
    pub palette : u8,
    pub priority : u8,
    pub flip_x : bool,
    pub flip_y : bool,
    pub palette_number : u8,
}

