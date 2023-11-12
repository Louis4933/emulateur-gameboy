mod attribut;
mod lcd;

use crate::ppu::attribut::Attribut;
use crate::ppu::lcd::{LcdControl, LcdStatus};
use crate::memoire::Memoire;
use crate::mmu::InterruptFlag;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

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

#[derive(Debug, Copy, Clone)]
pub struct Ppu {
    pub data: [Pixel; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub interrupt: u8,
    pub vblank: bool,
    pub hblank: bool,
    lcd_control: LcdControl,
    lcd_status: LcdStatus,

    // Spécifie la position dans la carte BG de 256x256 pixels (32x32 carreaux) qui doit être affichée en haut/à gauche.
    // Des valeurs comprises entre 0 et 255 peuvent être utilisées pour chaque X/Y, le contrôleur vidéo se chargeant automatiquement de la position de l'écran LCD.
    scroll_y: u8,
    scroll_x: u8,

    // Le LY : la ligne verticale sur laquelle les données actuelles sont transférées au pilote LCD.
    // Le LY peut prendre n'importe quelle valeur entre 0 et 153.
    lcdc_y: u8,
    
    // La Gameboy compare en permanence la valeur des registres LYC et LY. Lorsque les deux valeurs sont identiques,
    // le bit de coïncidence dans le registre STAT est activé et (s'il est activé) une interruption STAT est demandée.
    ly_compare: u8,
    
    // Spécifie les positions supérieure et gauche de la zone de la fenêtre.
    window_y: u8,
    window_x: u8,

    // Ce registre attribue des nuances de gris aux numéros de couleur des carreaux BG et Window.
    //   0  Blanc
    //   1  Gris clair
    //   2  Gris foncé
    //   3  Noir
    bg_palette: u8,

    // Ce registre attribue des nuances de gris à la palette de sprites 0.
    object_pallete_0: u8,

    // Ce registre attribue des nuances de gris à la palette de sprites 1. 
    object_pallete_1: u8,
    vram: [u8; 0x4000],
   
    // Ce registre à 1 bit sélectionne la banque de mémoire vidéo (VRAM) actuelle.
    vram_bank: usize,

    // Table d'attributs des sprites VRAM (OAM)
    //  Octet 0 - Position Y
    //      Spécifie la position verticale du sprite sur l'écran (moins 16).
    //  Octet 1 - Position X
    //      Spécifie la position horizontale du sprite sur l'écran (moins 8).
    //  Octet 2 - Numéro du carreau/motif
    //      Spécifie le numéro de tuile des sprites (00-FF).
    //  Octet 3 - Attributs/Flags
    oam: [u8; 0xA0],
    priorities: [(bool, usize); SCREEN_WIDTH],
    dots: u32,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            data: [Pixel::new(); SCREEN_WIDTH * SCREEN_HEIGHT],
            interrupt: InterruptFlag::None as u8,
            vblank: false,
            hblank: false,
            lcd_control: LcdControl::new(),
            lcd_status: LcdStatus::new(),
            scroll_x: 0x00,
            scroll_y: 0x00,
            lcdc_y: 0x00,
            ly_compare: 0x00,
            window_y: 0x00,
            window_x: 0x00,
            bg_palette: 0x00,
            object_pallete_0: 0x00,
            object_pallete_1: 0x01,
            vram: [0x00; 0x4000],
            vram_bank: 0x00,
            oam: [0x00; 0xA0],
            priorities: [(true, 0); SCREEN_WIDTH],
            dots: 0,
        }
    }

    pub fn run_cycles(&mut self, cycles: u32) {
        if !self.lcd_control.has_bit(7) {
            return;
        }
        self.hblank = false;
        if cycles == 0 {
            return;
        }
        let c = (cycles - 1) / 80 + 1;
        for i in 0..c {
            if i == (c - 1) {
                self.dots += cycles % 80
            } else {
                self.dots += 80
            }
            let d = self.dots;
            self.dots %= 456;
            if d != self.dots {
                self.lcdc_y = (self.lcdc_y + 1) % 154;
                if self.lcd_status.lyc_interrupt_enabled && self.lcdc_y == self.ly_compare {
                    self.interrupt |= InterruptFlag::LCDStat as u8;
                }
            }
            if self.lcdc_y >= 144 {
                if self.lcd_status.mode == 1 {
                    continue;
                }
                self.lcd_status.mode = 1;
                self.vblank = true;
                self.interrupt |= InterruptFlag::VBlank as u8;
                if self.lcd_status.m1_vblank_interrupt_enabled {
                    self.interrupt |= InterruptFlag::LCDStat as u8;
                }
            } else if self.dots <= 80 {
                if self.lcd_status.mode == 2 {
                    continue;
                }
                self.lcd_status.mode = 2;
                if self.lcd_status.m2_oam_interrupt_enabled {
                    self.interrupt |= InterruptFlag::LCDStat as u8;
                }
            } else if self.dots <= (80 + 172) {
                self.lcd_status.mode = 3;
            } else {
                if self.lcd_status.mode == 0 {
                    continue;
                }
                self.lcd_status.mode = 0;
                self.hblank = true;
                if self.lcd_status.m0_hblank_interrupt_enabled {
                    self.interrupt |= InterruptFlag::LCDStat as u8;
                }
                // Rendu de la ligne de balayage
                if self.lcd_control.has_bit(0) {
                    self.dessiner_arriere_plan();
                }
                if self.lcd_control.has_bit(1) {
                    self.dessiner_sprites();
                }
            }
        }
    }

    fn get_vram(&self, num: u8, addr: u16) -> u8 {
        match num {
            0 => self.vram[addr as usize - 0x8000],
            1 => self.vram[addr as usize - 0x6000],
            _ => panic!("ppu: invalid vram number"),
        }
    }

    // Ce registre attribue des nuances de gris aux numéros de couleur des carreaux BG et Window.
    //  Bit 7-6 - Nuance pour la couleur numéro 3
    //  Bit 5-4 - Nuance pour la couleur numéro 2
    //  Bit 3-2 - Nuance pour la couleur numéro 1
    //  Bit 1-0 - Nuance pour la couleur numéro 0
    fn get_nuance_de_gris(&self, value: u8, i: usize) -> u8 {
        match value >> (2 * i) & 0x03 {
            0x00 => 0xFF,
            0x01 => 0xC0,
            0x02 => 0x60,
            _ => 0x00,
        }
    }

    fn set_nuances_de_gris(&mut self, index: usize, g: u8) {
        self.data[(self.lcdc_y as usize * SCREEN_WIDTH) + index] = Pixel::from_greyscale(g);
    }

    fn dessiner_arriere_plan(&mut self) {
        let show_window = self.lcd_control.has_bit(5) && self.window_y <= self.lcdc_y;
        let tile_base = if self.lcd_control.has_bit(4) {
            0x8000
        } else {
            0x8800
        };
        let window_x = self.window_x.wrapping_sub(7);
        let picture_y = if show_window {
            self.lcdc_y.wrapping_sub(self.window_y)
        } else {
            self.scroll_y.wrapping_add(self.lcdc_y)
        };
        let tile_y = (u16::from(picture_y) >> 3) & 31;

        for x in 0..SCREEN_WIDTH {
            let picture_x = if show_window && x as u8 >= window_x {
                x as u8 - window_x
            } else {
                self.scroll_x.wrapping_add(x as u8)
            };
            let tile_x = (u16::from(picture_x) >> 3) & 31;
            let background_base_addr = if show_window && x as u8 >= window_x {
                if self.lcd_control.has_bit(6) {
                    0x9C00
                } else {
                    0x9800
                }
            } else if self.lcd_control.has_bit(3) {
                0x9C00
            } else {
                0x9800
            };
            let tile_addr = background_base_addr + tile_y * 32 + tile_x;
            let tile_number = self.get_vram(0, tile_addr);
            let tile_offset = if self.lcd_control.has_bit(4) {
                i16::from(tile_number)
            } else {
                i16::from(tile_number as i8) + 128
            } as u16
                * 16;
            let tile_location = tile_base + tile_offset;
            let tile_attribut = Attribut::from(self.get_vram(1, tile_addr));
            let tile_y = if tile_attribut.flip_y {
                7 - picture_y % 8
            } else {
                picture_y % 8
            };
            let tile_y_data: [u8; 2] =
                {
                    let a = self.get_vram(0, tile_location + u16::from(tile_y * 2));
                    let b = self.get_vram(0, tile_location + u16::from(tile_y * 2) + 1);
                    [a, b]
                };
            let tile_x = if tile_attribut.flip_x {
                7 - picture_x % 8
            } else {
                picture_x % 8
            };
            let color_low = usize::from(tile_y_data[0] & (0x80 >> tile_x) != 0);
            let color_high = if tile_y_data[1] & (0x80 >> tile_x) != 0 {
                2
            } else {
                0
            };
            let color = color_high | color_low;
            self.priorities[x] = (tile_attribut.priority, color);
            let color = self.get_nuance_de_gris(self.bg_palette, color);
            self.set_nuances_de_gris(x, color);
        }
    }

    fn dessiner_sprites(&mut self) {
        // Taille des tuiles de sprites 8x8 ou 8x16 (2 empilées verticalement).
        let sprite_size = if self.lcd_control.has_bit(2) { 16 } else { 8 };
        for i in 0..40 {
            let sprite_addr = 0xFE00 + (i as u16) * 4;
            let picture_y = self.get_octet(sprite_addr).wrapping_sub(16);
            let picture_x = self.get_octet(sprite_addr + 1).wrapping_sub(8);
            let tile_number = self.get_octet(sprite_addr + 2)
                & if self.lcd_control.has_bit(2) {
                    0xFE
                } else {
                    0xFF
                };
            let tile_attribut = Attribut::from(self.get_octet(sprite_addr + 3));

            // Si c'est le cas, la ligne de balayage est en dehors de la zone qui nous intéresse.
            if picture_y <= 0xFF - sprite_size + 1 {
                if self.lcdc_y < picture_y || self.lcdc_y > picture_y + sprite_size - 1 {
                    continue;
                }
            } else if self.lcdc_y > picture_y.wrapping_add(sprite_size) - 1 {
                continue;
            }
            if picture_x >= (SCREEN_WIDTH as u8) && picture_x <= (0xFF - 7) {
                continue;
            }

            let tile_y = if tile_attribut.flip_y {
                sprite_size - 1 - self.lcdc_y.wrapping_sub(picture_y)
            } else {
                self.lcdc_y.wrapping_sub(picture_y)
            };
            let tile_y_addr = 0x8000u16 + u16::from(tile_number) * 16 + u16::from(tile_y) * 2;
            let tile_y_data: [u8; 2] =
                {
                    let b1 = self.get_vram(0, tile_y_addr);
                    let b2 = self.get_vram(0, tile_y_addr + 1);
                    [b1, b2]
                };

            for x in 0..8 {
                if picture_x.wrapping_add(x) >= (SCREEN_WIDTH as u8) {
                    continue;
                }
                let tile_x = if tile_attribut.flip_x { 7 - x } else { x };
                let color_low = usize::from(tile_y_data[0] & (0x80 >> tile_x) != 0);
                let color_high = if tile_y_data[1] & (0x80 >> tile_x) != 0 {
                    2
                } else {
                    0
                };
                let color = color_high | color_low;
                if color == 0 {
                    continue;
                }

                // Confirme la priorité de l'arrière-plan et du sprite.
                let priority = self.priorities[picture_x.wrapping_add(x) as usize];
                let skip = if priority.0 {
                    priority.1 != 0
                } else {
                    tile_attribut.priority && priority.1 != 0
                };
                if skip {
                    continue;
                }

                let color = if tile_attribut.palette_number == 1 {
                    self.get_nuance_de_gris(self.object_pallete_1, color)
                } else {
                    self.get_nuance_de_gris(self.object_pallete_0, color)
                };
                self.set_nuances_de_gris(picture_x.wrapping_add(x) as usize, color);
            }
        }
    }
}

impl Memoire for Ppu {
    fn get_octet(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[self.vram_bank * 0x2000 + addr as usize - 0x8000],
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00],
            0xFF40 => self.lcd_control.data,
            0xFF41 => {
                let bit6 = if self.lcd_status.lyc_interrupt_enabled {
                    0x40
                } else {
                    0x00
                };
                let bit5 = if self.lcd_status.m2_oam_interrupt_enabled {
                    0x20
                } else {
                    0x00
                };
                let bit4 = if self.lcd_status.m1_vblank_interrupt_enabled {
                    0x10
                } else {
                    0x00
                };
                let bit3 = if self.lcd_status.m0_hblank_interrupt_enabled {
                    0x08
                } else {
                    0x00
                };
                let bit2 = if self.lcdc_y == self.ly_compare {
                    0x04
                } else {
                    0x00
                };
                bit6 | bit5 | bit4 | bit3 | bit2 | self.lcd_status.mode
            }
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.lcdc_y,
            0xFF45 => self.ly_compare,
            0xFF47 => self.bg_palette,
            0xFF48 => self.object_pallete_0,
            0xFF49 => self.object_pallete_1,
            0xFF4A => self.window_y,
            0xFF4B => self.window_x,
            0xFF4F => 0xFE | self.vram_bank as u8,
            _ => panic!("ppu: invalid address {:#06X?}", addr),
        }
    }

    fn set_octet(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[self.vram_bank * 0x2000 + addr as usize - 0x8000] = value,
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00] = value,
            0xFF40 => {
                self.lcd_control.data = value;
                if !self.lcd_control.has_bit(7) {
                    self.dots = 0;
                    self.lcdc_y = 0;
                    self.lcd_status.mode = 0;
                    // Clean l'écran
                    self.data = [Pixel::new(); SCREEN_WIDTH * SCREEN_HEIGHT];
                    self.vblank = true;
                }
            }
            0xFF41 => {
                self.lcd_status.lyc_interrupt_enabled = value & 0x40 != 0x00;
                self.lcd_status.m2_oam_interrupt_enabled = value & 0x20 != 0x00;
                self.lcd_status.m1_vblank_interrupt_enabled = value & 0x10 != 0x00;
                self.lcd_status.m0_hblank_interrupt_enabled = value & 0x08 != 0x00;
            }
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF44 => {}
            0xFF45 => self.ly_compare = value,
            0xFF47 => self.bg_palette = value, 
            0xFF48 => self.object_pallete_0 = value,
            0xFF49 => self.object_pallete_1 = value,
            0xFF4A => self.window_y = value,
            0xFF4B => self.window_x = value,
            0xFF4F => self.vram_bank = (value & 0x01) as usize,
            _ => panic!("ppu: invalid address {:#06X?}", addr),
        }
    }
}
