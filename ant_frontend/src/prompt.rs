use std::{char, io::Write};
use crate::buffer::DefaultBuffer;
use termion::cursor::Goto;


pub fn prompt(screen: &mut impl Write, buffer: &mut DefaultBuffer, prompt: &str) {
    let (_terminal_width, terminal_height) = termion::terminal_size().unwrap();   
    buffer.prompt.clear();
    buffer.prompt.push_str(prompt);
    buffer.set_position(screen, 1, terminal_height);
    write!(screen, "{}{}", termion::clear::CurrentLine, buffer.prompt).unwrap();
    screen.flush().unwrap();
}

pub fn prompt_insert_char(screen: &mut impl Write, buffer: &mut DefaultBuffer, c: char) {
    let (_terminal_width, terminal_height) = termion::terminal_size().unwrap();
    buffer.prompt.push(c); 
    buffer.set_position(screen, 1, terminal_height);
    write!(screen, "{}{}", termion::clear::CurrentLine, buffer.prompt).unwrap();
    screen.flush().unwrap();
}

pub fn prompt_delete_char(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    let (_terminal_width, terminal_height) = termion::terminal_size().unwrap();
    buffer.prompt.pop(); 
    buffer.set_position(screen, 1, terminal_height);
    write!(screen, "{}{}", termion::clear::CurrentLine, buffer.prompt).unwrap();
    screen.flush().unwrap();
}

pub fn prompt_return_prompt(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    buffer.in_prompt = false;
    buffer.prompt.replace_range(..buffer.current_prompt.len(), "");
    buffer.name = Some(buffer.prompt.to_string());
    buffer.file_path = Some(buffer.prompt.to_string());
    buffer.save(screen).unwrap();
    buffer.set_position(screen, buffer.first_char, 1);
}
