use crate::buffer::Buffer;
use std::io::{Stdin, Write};
use termion::style::Blink;


pub fn draw_lines(buffer: &mut Buffer, screen: &mut impl Write, last: usize) {
    let line_iterator = buffer.buffer_by_line();

    for (line_number, line) in line_iterator.iter().enumerate() { 
        let line_number = (line_number+1) as usize;
        if line_number>last {
            continue;
        } else {
            if (line_number>=10)&(line_number<100) {
                writeln!(screen, "{}{}  {}",
                    termion::cursor::Goto(1, line_number as u16),
                    line_number,
                    line).unwrap();
            } else if (line_number>=100)&(line_number<1000) {
                writeln!(screen, "{}{} {}",
                    termion::cursor::Goto(1, line_number as u16),
                    line_number,
                    line).unwrap();
            } else {
                writeln!(screen, "{}{}   {}",
                    termion::cursor::Goto(1, line_number as u16),
                    line_number,
                    line).unwrap();
            }
        }
    };

    writeln!(screen, "{}",
        termion::cursor::Goto(5, 1)
    ).unwrap();
    buffer.current_height = 1;
    buffer.current_width = 5;
 }
