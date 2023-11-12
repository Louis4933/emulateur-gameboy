mod joypad;

#[derive(Clone, Copy)]
pub enum GameboyButton {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

impl From<GameboyButton> for joypad::JoypadKey {
    fn from(value: GameboyButton) -> joypad::JoypadKey {
        match value {
            GameboyButton::A => joypad::JoypadKey::A,
            GameboyButton::B => joypad::JoypadKey::B,
            GameboyButton::Right => joypad::JoypadKey::Right,
            GameboyButton::Left => joypad::JoypadKey::Left,
            GameboyButton::Up => joypad::JoypadKey::Up,
            GameboyButton::Down => joypad::JoypadKey::Down,
            GameboyButton::Select => joypad::JoypadKey::Select,
            GameboyButton::Start => joypad::JoypadKey::Start,
        }
    }
}

impl Gameboy {

    

}