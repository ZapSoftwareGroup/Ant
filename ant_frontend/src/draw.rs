use std::io::Write;
use crate::buffer::DefaultBuffer;
use termion::cursor::Goto;
use termion::{clear, color};
use termion::clear::CurrentLine;


const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);

pub fn draw_statusline(screen: &mut impl Write, buffer: &mut DefaultBuffer) {

    let (move_to_x, move_to_y) = (buffer.current_x, buffer.current_y);

    let (width, height) = termion::terminal_size().unwrap();
    write!(screen, "{}", termion::cursor::Goto(1, height-1)).unwrap();

    let width = width as usize;

    let mut file_name = &("[No Name]".to_string());
    let mut file_path = &("".to_string());
    let mut file_type = &String::from("Plain Text");

    if let Some(name) = &buffer.name {
        file_name = name;
    }
    
    if let Some(path) = &buffer.file_path {
        file_path = path;
    }

    let mut status = format!(
        "{} - {} lines",
        file_name,
        buffer.line_count
    );

    if let Some(filetype) = &buffer.filetype {
        file_type = filetype;
    }

    let buffer_indicator = format!(
        "{} | {} | {}/{}",
        file_type,
        file_path,
        buffer.current_x-5,
        buffer.current_y
    );

    let len = status.len() + buffer_indicator.len();
    status.push_str(&" ".repeat(width.saturating_sub(len)));
    status = format!("{}{}", status, buffer_indicator);
    status.truncate(width);
    write!(screen, "{}", color::Bg(STATUS_BG_COLOR)).unwrap();
    write!(screen, "{}", color::Fg(STATUS_FG_COLOR)).unwrap();
    write!(screen, "{}\r", status).unwrap();
    write!(screen, "{}", color::Bg(color::Reset)).unwrap();
    write!(screen, "{}", color::Fg(color::Reset)).unwrap();
    buffer.set_position(screen, move_to_x, move_to_y);
}

pub fn draw_line(screen: &mut impl Write, buffer: &mut DefaultBuffer, x_pos: u16, index: usize) {
     // Set cursor position to current line
     buffer.set_position(screen,1,buffer.current_y);

     let (terminal_width,_terminal_height) = termion::terminal_size().unwrap();
     
     // Redraw line
     let (line_number, text) = &buffer.lines[index];

     write!(screen, "{}{}{}{}{}{}",
         CurrentLine,
         color::Fg(color::LightYellow),
         line_number+1,
         color::Fg(color::Reset),
         Goto(buffer.first_char, buffer.current_y),
         truncate_line(text, buffer, terminal_width)).unwrap();

     buffer.set_position(screen, x_pos, buffer.current_y);

}

fn truncate_line<'a>(line: &'a str, buffer: &DefaultBuffer, terminal_width: u16) -> String {
    let allowed_chars = terminal_width - buffer.first_char + 1;
    let char_count = line.chars().count();
    // If the length of the line is less than the length of the screen...
    if char_count < (allowed_chars as usize) {
        // Just show the whole line
        if char_count <= (buffer.starting_index-1) as usize {
            String::new()
        } else {
            line[buffer.starting_index as usize-1..].to_owned()
        }
    } else {
        if !((allowed_chars as usize)+(buffer.starting_index-1) as usize >=char_count) {
            line[buffer.starting_index as usize-1..(allowed_chars as usize)+(buffer.starting_index-1) as usize].to_owned()
        } else {
            line[buffer.starting_index as usize-1..].to_owned()
        }

    }
    
} 

pub fn draw_lines(screen: &mut impl Write, buffer: &mut DefaultBuffer, height: usize) {
    let (terminal_width,terminal_height) = termion::terminal_size().unwrap();
    let start_index: usize = terminal_height as usize-2;
    write!(screen, "{}", termion::cursor::Goto(terminal_width-1, terminal_height-2)).unwrap();
    write!(screen, "{}", clear::BeforeCursor).unwrap();
    let begin_index = height-start_index;
    let end_index = if buffer.line_count<height { buffer.line_count } else { height };
    let line_iterator = &buffer.lines[begin_index..end_index];

    for (inde, (line_number, line)) in line_iterator.iter().enumerate() { 
        let index = inde+1;
        let line_number = (line_number+1) as usize;
        if line.chars().count() > buffer.longest_shown as usize {
            buffer.longest_shown=line.chars().count() as u16;
        }
        write!(screen, "{}{}{}{}{}{}",
            termion::cursor::Goto(1, index as u16),
            color::Fg(color::LightYellow),
            line_number,
            color::Fg(color::Reset),
            termion::cursor::Goto(buffer.first_char, index as u16),
            truncate_line(line, buffer, terminal_width)).unwrap();
    };
}
