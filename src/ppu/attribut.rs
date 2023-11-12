pub struct Attribut {
    pub priority : bool,
    pub flip_x : bool,
    pub flip_y : bool,
    pub palette_number : usize,
}

impl From<u8> for Attribut {
    fn from(byte: u8) -> Attribut {
        Attribut {
            priority: byte & (1 << 4) != 0,
            flip_y: byte & (1 << 3) != 0,
            flip_x: byte & (1 << 2) != 0,
            palette_number: byte as usize & (1 << 1)
        }
    }
}