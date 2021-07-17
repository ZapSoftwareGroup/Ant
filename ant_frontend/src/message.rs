use std::io::Write;
use crate::buffer::DefaultBuffer;
use termion::cursor::Goto;

pub fn display_message_save(screen: &mut impl Write, buffer: &mut DefaultBuffer, message: &str) {
    let (current_x, current_y) = (buffer.current_x, buffer.current_y);
    let (_width, terminal_height) = termion::terminal_size().unwrap();
    if let Some(name) = &buffer.name {
        write!(screen, "{}{} \"{}\"", Goto(1, terminal_height), message, name).unwrap();
    }
    buffer.set_position(screen, current_x, current_y);
}
