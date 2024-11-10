

#[derive(Copy, Clone)]
pub struct TerminalColor {
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
}

pub enum Color {
    Red,
    Green,
    Yellow,
}

impl TerminalColor {
    pub fn new() -> TerminalColor {
        let terminal_color = TerminalColor {
            red: "\x1b[31m",
            green: "\x1b[32m",
            yellow: "\x1b[33m",
        };
        terminal_color
    }

    pub fn color(&self, msg: &str, color: Color) -> String {
        const TAIL: &str = "\x1b[0m";
        match color {
            Color::Red => format!("{}{}{}", self.red, &msg, TAIL),
            Color::Green => format!("{}{}{}", self.green, &msg, TAIL),
            Color::Yellow => format!("{}{}{}", self.yellow, &msg, TAIL),
        }
    }
}
