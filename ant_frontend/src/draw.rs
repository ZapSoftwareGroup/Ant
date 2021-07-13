use std::io::Write;
use crate::buffer::DefaultBuffer;
use termion::color;
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

pub fn draw_line(screen: &mut impl Write, buffer: &mut DefaultBuffer, y_position: usize, index: usize) {
     // Set cursor position to current line
     buffer.set_position(screen, 1, y_position as u16);

     // Redraw line
     let (line_number, text) = &buffer.lines[index];

     if (line_number>=&10)&(line_number<&100) {
        
         write!(screen, "{}{}{}{}   {}",
             CurrentLine,
             color::Fg(color::LightYellow),
             line_number,
             color::Fg(color::Reset),
             text).unwrap();
     } else if (line_number>=&100)&(line_number<&1000) {
            write!(screen, "{}{}{}{} {}",
                CurrentLine,
                color::Fg(color::LightYellow),
                line_number,
                color::Fg(color::Reset),
                text).unwrap();
        } else {
            write!(screen, "{}{}{}{}   {}",
                CurrentLine,
                color::Fg(color::LightYellow),
                line_number,
                color::Fg(color::Reset),
                text).unwrap();
        }

}

pub fn draw_lines(screen: &mut impl Write, buffer: &mut DefaultBuffer, height: usize) {
    let (_terminal_width, terminal_height) = termion::terminal_size().unwrap();
    // buffer.set_position(screen, terminal_width, terminal_height-2);
    // write!(screen, "{}", termion::clear::BeforeCursor).unwrap();
    let start_index: usize = terminal_height as usize-2;

    let begin_index = height-start_index;
    let end_index = if buffer.line_count<height { buffer.line_count } else { height };
    let line_iterator = &buffer.lines[begin_index..end_index];

    for (inde, (line_number, line)) in line_iterator.iter().enumerate() { 
        let index = inde as i16+1;
        let line_number = (line_number+1) as usize;
        if (line_number>=10)&(line_number<100) {
            write!(screen, "{}{}{}{}{}  {}",
                termion::cursor::Goto(1, index as u16),
                CurrentLine,
                color::Fg(color::LightYellow),
                line_number,
                color::Fg(color::Reset),
                line).unwrap();
        } else if (line_number>=100)&(line_number<1000) {
            write!(screen, "{}{}{}{}{} {}",
                termion::cursor::Goto(1, index as u16),
                CurrentLine,
                color::Fg(color::LightYellow),
                line_number,
                color::Fg(color::Reset),
                line).unwrap();
        } else {
            write!(screen, "{}{}{}{}{}   {}",
                termion::cursor::Goto(1, index as u16),
                CurrentLine,
                color::Fg(color::LightYellow),
                line_number,
                color::Fg(color::Reset),
                line).unwrap();
        }
    };
}

pub fn draw_lines_anon(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    let (_terminal_width, terminal_height) = termion::terminal_size().unwrap();
    buffer.set_position(screen, 1, terminal_height-2);
    write!(screen, "{}", termion::clear::BeforeCursor).unwrap();

    let line_iterator = &buffer.lines;

    for (inde, (line_number, line)) in line_iterator.iter().enumerate() { 
        let index = inde as i16+1;
        if (line_number>=&10)&(line_number<&100) {
            write!(screen, "{}{}{}{}  {}",
                termion::cursor::Goto(1, index as u16),
                color::Fg(color::LightYellow),
                line_number,
                color::Fg(color::Reset),
                line).unwrap();
        } else if (line_number>=&100)&(line_number<&1000) {
            write!(screen, "{}{}{}{} {}",
                termion::cursor::Goto(1, index as u16),
                color::Fg(color::LightYellow),
                line_number,
                color::Fg(color::Reset),
                line).unwrap();
        } else {
            write!(screen, "{}{}{}{}   {}",
                termion::cursor::Goto(1, index as u16),
                color::Fg(color::LightYellow),
                line_number,
                color::Fg(color::Reset),
                line).unwrap();
        }
    };
    buffer.set_position(screen, 5, 1);
}
