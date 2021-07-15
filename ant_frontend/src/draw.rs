use std::io::Write;
use crate::buffer::DefaultBuffer;


use termion::cursor::Goto;
use termion::{clear, color};
use termion::clear::CurrentLine;


const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);

pub fn draw_statusline(screen: &mut impl Write, buffer: &mut DefaultBuffer, height: u16, width: u16) {

    write!(screen, "{}", termion::cursor::Goto(1, height-1)).unwrap();

    let width = width as usize;

    let mut file_name = "[No Name]".to_string();
    let mut file_path = "".to_string();

    if let Some(name) = &buffer.name {
        file_name = name.clone();
    }
    
    if let Some(path) = &buffer.file_path {
        file_path = path.clone();
    }

    let mut status = format!(
        "{} - {} lines",
        file_name,
        buffer.line_count
    );

    let buffer_indicator = format!(
        "{} | {}/{}",
        file_path,
        buffer.current_x,
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
}

pub fn draw_line(screen: &mut impl Write, buffer: &mut DefaultBuffer, x_pos: u16, index: usize) {
     // Set cursor position to current line
     buffer.set_position(screen,1,buffer.current_y);

     // Redraw line
     let (line_number, text) = &buffer.lines[index];

     write!(screen, "{}{}{}{}{}{}",
         CurrentLine,
         color::Fg(color::LightYellow),
         line_number+1,
         color::Fg(color::Reset),
         Goto(buffer.first_char, buffer.current_y),
         text).unwrap();

     buffer.set_position(screen, x_pos, buffer.current_y);

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
        write!(screen, "{}{}{}{}{}{}",
            termion::cursor::Goto(1, index as u16),
            color::Fg(color::LightYellow),
            line_number,
            color::Fg(color::Reset),
            termion::cursor::Goto(buffer.first_char, index as u16),
            line).unwrap();
        
    };
}
