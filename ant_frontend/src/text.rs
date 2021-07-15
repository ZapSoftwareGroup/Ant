use std::io::Write;
use crate::buffer::DefaultBuffer;
use std::char;
use crate::draw::{draw_lines, draw_line};
use crate::movement::*;


pub fn insert_newline(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    let current_line: usize = (buffer.current_y+buffer.shown_first-1) as usize;
    buffer.lines.insert(current_line, (current_line, "".to_string()));
    buffer.line_count+=1;

    // Add 1 to line number of all lines after newly inserted one
    let mut counter = 0;
    for (line_number, _text) in &mut buffer.lines {
        *line_number = counter;
        counter += 1;
    }
    
    // Draw lines
    draw_lines(screen, buffer, (buffer.shown_line) as usize);

}

pub fn insert_newline_anon(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    let current_line: usize = (buffer.current_y+buffer.shown_first-1) as usize;
    buffer.lines.insert(current_line, (current_line, "".to_string()));
    buffer.line_count+=1;

    // Add 1 to line number of all lines after newly inserted one
    let mut counter = 0;
    for (line_number, _text) in &mut buffer.lines {
        *line_number = counter;
        counter += 1;
    }
    
    // Draw lines
    draw_lines(screen, buffer, (buffer.shown_line) as usize);
    screen.flush().unwrap();

}
pub fn insert_char_at_pos(screen: &mut impl Write, buffer: &mut DefaultBuffer, char: char) {
    let current_line = (buffer.current_y+buffer.shown_first-2) as usize;
    let current_position = (buffer.current_x-5) as usize;
    buffer.lines[current_line].1.insert(current_position, char);


    // FIXME: Change this to draw_line function, instead of redrawing everything.
    draw_line(screen, buffer, (current_position+5) as u16, current_line); 
    screen.flush().unwrap();
}

pub fn delete_char_or_newline(screen: &mut impl Write, buffer: &mut DefaultBuffer) {

    let current_line = (buffer.current_y+buffer.shown_first-1) as isize;
    let (_line_num, text) = &buffer.lines[current_line as usize-1];

    if (buffer.current_x==5)&(text==&"".to_string())&(buffer.current_y!=1) {
        let current_line = (buffer.current_y+buffer.shown_first-2) as usize;
        buffer.lines.remove(current_line);
        buffer.line_count -= 1;


        let mut counter = 0;
        for (line_number, _text) in &mut buffer.lines {
            *line_number = counter;
            counter += 1;
        }

        draw_lines(screen, buffer, (buffer.shown_line) as usize);
        move_up(screen, buffer);
        return;
    }

    let current_line = (buffer.current_y+buffer.shown_first-2) as usize;
    let current_position = (buffer.current_x-5) as usize;
    if buffer.current_x != 5 {
        buffer.lines[current_line].1.remove(current_position-1);

        draw_line(screen, buffer, (current_position+5) as u16, current_line); 
      

        move_left(screen, buffer);
     }

}
