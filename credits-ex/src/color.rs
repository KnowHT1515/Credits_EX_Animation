#[repr(u8)]
pub enum Fore {
    Black   = 30,
    Red     = 31,
    Green   = 32,
    Yellow  = 33,
    Blue    = 34,
    Magenta = 35,
    Cyan    = 36,
    White   = 37,
}

#[repr(u8)]
pub enum Style {
    Normal = 22,
    Bright = 1,
}

pub fn ansi_code(fore: Fore, style: Style) -> String {
    format!("\x1b[{};{}m", style as u8, fore as u8)
}