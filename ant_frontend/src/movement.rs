use std::io::Write;
use termion::cursor;
use crate::buffer::Buffer;


pub fn move_down(screen: &mut impl Write, buffer: &mut Buffer) -> Result<(), String> {
    let current_height = buffer.current_height;
    let current_width = buffer.current_width;
    writeln!(screen, "{}", cursor::Goto(current_width, current_height-1)).unwrap();
    buffer.current_height = current_height-1;
    Ok(())
}
