use std::io::Write;
use termion::cursor::Goto;

pub fn display_message(screen: &mut impl Write, message: &str) {
    let (_width, terminal_height) = termion::terminal_size().unwrap();
    write!(screen, "{}{}", Goto(1, terminal_height), message).unwrap();
}
