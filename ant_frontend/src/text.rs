use std::io::Write;
use crate::buffer::DefaultBuffer;
use std::char;
use crate::draw::draw_lines;

pub fn insert_newline(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    let current_line: usize = (buffer.current_y+buffer.shown_first-1) as usize;
    buffer.lines.insert(current_line, (current_line, "".to_string()));

    // Add 1 to line number of all lines after newly inserted one
    let mut counter = 0;
    for (line_number, _text) in &mut buffer.lines {
        *line_number = counter;
        counter += 1;
    }
    
    // Draw lines
    draw_lines(screen, buffer, (buffer.shown_line) as usize);
    buffer.set_position(screen, 5, buffer.current_y+1);
    screen.flush().unwrap();
}
pub fn insert_char_at_pos(screen: &mut impl Write, buffer: &mut DefaultBuffer, char: char) {
    return;
}
