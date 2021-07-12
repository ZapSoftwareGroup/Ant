use std::io::Write;
use crate::buffer::DefaultBuffer;
use std::char;
use crate::draw::draw_lines;

pub fn insert_newline(screen: &mut impl Write, buffer: &mut DefaultBuffer, char: char) {
    let current_line = buffer.current_y+buffer.shown_first-1;
    buffer.lines.insert((current_line) as usize, ((current_line) as usize, String::from(" ")));
    for (line_number, _text) in &mut buffer.lines {
        if line_number>&mut (current_line as usize) {
            *line_number += 1;
        }
    }

    draw_lines(screen, buffer, buffer.shown_line as usize);
}
pub fn insert_char_at_pos(screen: &mut impl Write, buffer: &mut DefaultBuffer, char: char) {
    return;
}
