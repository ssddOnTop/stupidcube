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
        match self {
            Colours::White => "\x1b[37mW\x1b[0m",   // White
            Colours::Blue => "\x1b[34mB\x1b[0m",    // Blue
            Colours::Red => "\x1b[31mR\x1b[0m",     // Red
            Colours::Yellow => "\x1b[33mY\x1b[0m",  // Yellow
            Colours::Magenta => "\x1b[35mO\x1b[0m", // Magenta
            Colours::Cyan => "\x1b[36mN\x1b[0m",    // Cyan
        }
    }
}
