#[derive(Debug, strum_macros::Display, Clone, Copy)]
pub enum Colours {
    White,
    Blue,
    Red,
    Yellow,
    Magenta,
    Cyan,
}

impl Colours {
    pub fn as_str(&self) -> &str {
        // TODO: use as_char to build this string
        match self {
            Colours::White => "\x1b[37mW\x1b[0m",   // White
            Colours::Blue => "\x1b[34mB\x1b[0m",    // Blue
            Colours::Red => "\x1b[31mR\x1b[0m",     // Red
            Colours::Yellow => "\x1b[33mY\x1b[0m",  // Yellow
            Colours::Magenta => "\x1b[35mM\x1b[0m", // Magenta
            Colours::Cyan => "\x1b[36mC\x1b[0m",    // Cyan
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Colours::White => 'W',   // White
            Colours::Blue => 'B',    // Blue
            Colours::Red => 'R',     // Red
            Colours::Yellow => 'Y',  // Yellow
            Colours::Magenta => 'M', // Magenta
            Colours::Cyan => 'C',    // Cyan
        }
    }
}

impl From<u8> for Colours {
    fn from(value: u8) -> Self {
        match value {
            0 => Colours::White,
            1 => Colours::Blue,
            2 => Colours::Red,
            3 => Colours::Yellow,
            4 => Colours::Magenta,
            5 => Colours::Cyan,
            _ => panic!("Invalid colour"),
        }
    }
}

impl From<Colours> for u8 {
    fn from(value: Colours) -> u8 {
        match value {
            Colours::White => 0,
            Colours::Blue => 1,
            Colours::Red => 2,
            Colours::Yellow => 3,
            Colours::Magenta => 4,
            Colours::Cyan => 5,
        }
    }
}

impl From<Colours> for crossterm::style::Color {
    fn from(val: Colours) -> Self {
        match val {
            Colours::White => crossterm::style::Color::White,
            Colours::Blue => crossterm::style::Color::Blue,
            Colours::Red => crossterm::style::Color::Red,
            Colours::Yellow => crossterm::style::Color::Yellow,
            Colours::Magenta => crossterm::style::Color::Magenta,
            Colours::Cyan => crossterm::style::Color::Cyan,
        }
    }
}
