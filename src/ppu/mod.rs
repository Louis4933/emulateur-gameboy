mod attribut;
mod lcd;



#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    fn new() -> Pixel {
        Pixel {
            r: 0xFF,
            g: 0xFF,
            b: 0xFF,
        }
    }

    fn from_greyscale(g: u8) -> Pixel {
        Pixel { r: g, g, b: g }
    }
}