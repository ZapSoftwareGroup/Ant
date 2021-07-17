use std::io::Write;
use crate::buffer::{DefaultBuffer, find_first_char};
use std::char;
use crate::draw::{draw_lines, draw_line};
use crate::movement::*;


pub fn insert_newline(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    let current_line: usize = (buffer.current_y+buffer.shown_first-1) as usize;
    let current_position = buffer.current_x-buffer.first_char;
    let possible_width = buffer.lines[buffer.shown_first as usize+buffer.current_y as usize-2].1.chars().count();

    if current_position == possible_width as u16 {
        buffer.lines.insert(current_line, (current_line, "".to_string()));
        buffer.line_count+=1;
        buffer.first_char = find_first_char(buffer.line_count);
    } else {
        // delete original end of line
        let substring = buffer.lines[current_line-1].1[current_position as usize..possible_width].to_string();
        buffer.lines[current_line-1].1.replace_range(current_position as usize..possible_width, "");
        buffer.lines.insert(current_line, (current_line, substring));
        buffer.line_count+=1;
        buffer.first_char = find_first_char(buffer.line_count);
    }


    // Add 1 to line number of all lines after newly inserted one
    let mut counter = 0;
    for (line_number, _text) in &mut buffer.lines {
        *line_number = counter;
        counter += 1;
    }
    
    // Draw lines
    draw_lines(screen, buffer, (buffer.shown_line) as usize);

}

pub fn insert_tab(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    // Insert a tab
    let current_line = (buffer.current_y+buffer.shown_first-2) as usize;
    let current_position = (buffer.current_x-buffer.first_char) as usize;
    for _count in 0..4 {
        buffer.lines[current_line].1.insert(current_position, ' ');
        move_right(screen, buffer);
    }


    draw_line(screen, buffer, buffer.current_x, current_line); 
}

pub fn insert_char_at_pos(screen: &mut impl Write, buffer: &mut DefaultBuffer, char: char) {
    let current_line = (buffer.current_y+buffer.shown_first-2) as usize;
    let current_position = (buffer.current_x-buffer.first_char) as usize;
    buffer.lines[current_line].1.insert(current_position, char);

    move_right(screen, buffer);


    draw_line(screen, buffer, buffer.current_x, current_line); 
}

pub fn delete_char_or_newline(screen: &mut impl Write, buffer: &mut DefaultBuffer) {

    let current_line = (buffer.current_y+buffer.shown_first-1) as isize;
    let (_line_num, text) = &buffer.lines[current_line as usize-1];

    if (buffer.current_x==buffer.first_char)&(text==&"".to_string())&(buffer.current_y!=1) {
        let current_line = (buffer.current_y+buffer.shown_first-3) as usize;
        buffer.lines.remove(current_line+1);
        buffer.line_count -= 1;
        buffer.first_char = find_first_char(buffer.line_count);


        let mut counter = 0;
        for (line_number, _text) in &mut buffer.lines {
            *line_number = counter;
            counter += 1;
        }

        draw_lines(screen, buffer, buffer.shown_line as usize);
        move_up(screen, buffer);
        return;
    }

    let current_line = (buffer.current_y+buffer.shown_first-2) as usize;
    let current_position = (buffer.current_x-buffer.first_char) as usize;
    if buffer.current_x != buffer.first_char {
        buffer.lines[current_line].1.remove(current_position-1);

        draw_line(screen, buffer, buffer.current_x, current_line); 
      

        move_left(screen, buffer);
     } else if (buffer.current_x == buffer.first_char)&(buffer.shown_first+buffer.current_y-1!=1) {
        // remove this clone
        let substring = &buffer.lines[current_line].1.clone();
        buffer.lines[current_line-1].1.push_str(substring.as_ref());
        let length = buffer.lines[current_line-1].1.len();
        buffer.lines.remove(current_line);
        buffer.line_count -= 1;
        buffer.first_char = find_first_char(buffer.line_count);

        let mut counter = 0;
        for (line_number, _text) in &mut buffer.lines {
            *line_number = counter;
            counter += 1;
        }

        draw_lines(screen, buffer, buffer.shown_line as usize);
        move_up(screen, buffer);
        for _i in 0..length as u16 {
            move_right(screen, buffer);
        }
    }  
}
