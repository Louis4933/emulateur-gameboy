use crate::lib::GameboyButton;
use crate::lib::Gameboy;
use std::fs::File;
use std::io::Read;

use argparse::{ArgumentParser, Store};
use minifb::{Key, Scale, Window, WindowOptions};

const KEY_MAPPINGS: [(Key, GameboyButton); 8] = [
    (Key::Right, GameboyButton::Right),
    (Key::Up, GameboyButton::Up),
    (Key::Left, GameboyButton::Left),
    (Key::Down, GameboyButton::Down),
    (Key::Z, GameboyButton::A),
    (Key::X, GameboyButton::B),
    (Key::Space, GameboyButton::Select),
    (Key::Enter, GameboyButton::Start),
];

fn main() {
    let mut rom_path = String::from("");
    {
        let mut arg_parser = ArgumentParser::new();
        arg_parser.set_description("Emulateur de Gameboy");
        arg_parser
            .refer(&mut rom_path)
            .add_argument("rom", Store, "Chemin")
            .required();
        arg_parser.parse_args_or_exit();
    }

    let window_options = WindowOptions {
        resize: true,
        scale: Scale::X2,
        ..Default::default()
    };

    let mut file = File::open(rom_path.clone()).unwrap();
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap();

    let mut gameboy = Gameboy::new(rom);

    let mut window = Window::new(
        &format!("Gameboy"),
        gameboy.get_screen_dimension()[1],
        gameboy.get_screen_dimension()[0],
        window_options,
    )
    .unwrap();
    let mut window_buffer = vec![0x00; gameboy.get_screen_dimension()[0] * gameboy.get_screen_dimension()[1]];
    window
        .update_with_buffer(window_buffer.as_slice(), gameboy.get_screen_dimension()[1], gameboy.get_screen_dimension()[0])
        .unwrap();

    while window.is_open() {
        gameboy.step();
        if gameboy.has_screen_updated() {
            for (i, pixel) in gameboy.get_screen_data().iter().enumerate() {
                let r = u32::from(pixel.r) << 16;
                let g = u32::from(pixel.g) << 8;
                let b = u32::from(pixel.b);
                let a = 0xFF00_0000;
                window_buffer[i] = a | r | g | b;
            }
            window
                .update_with_buffer(window_buffer.as_slice(), gameboy.get_screen_dimension()[1], gameboy.get_screen_dimension()[0])
                .unwrap();
        }
        if gameboy.can_take_input() {
            for (physical_key, gameboy_button) in &KEY_MAPPINGS {
                if window.is_key_down(*physical_key) {
                    gameboy.gerer_keydown(*gameboy_button);
                } else {
                    gameboy.gerer_keyup(*gameboy_button);
                }
            }
        }
    }
}
