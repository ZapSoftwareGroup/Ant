use termion::clear;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::screen::*;
use termion::cursor::DetectCursorPos;
use crate::editor::Editor;
use crate::buffer::Buffer;
use crate::draw::draw_lines;
use crate::movement;


pub fn render_blank_tui() {
    let term_size: (u16, u16) = termion::terminal_size().expect("Don't use windows!");

    let stdin = stdin();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let half_width = (term_size.0/2)-30;
    let half_height = term_size.1/2;

    write!(screen, "{}{}Ant text editor, copyleft 2021. Press any space to continue...{}",
           clear::All,
           termion::cursor::Goto(half_width, half_height),
           termion::cursor::Hide).unwrap();


    write!(screen, "").unwrap();


    screen.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => {
                write!(screen, "{}", clear::All).unwrap();
                break
            },
            _ => ()
        };

    };
}

pub fn render_tui(editor: &mut Editor<Buffer>) {
    let term_size: (u16, u16) = termion::terminal_size().expect("Don't use windows!");

    let mut stdin = stdin();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let half_width = (term_size.0/2)-30;
    let half_height = term_size.1/2;
    let height: u16 = term_size.1 - 1;

    write!(screen, "{}{}Ant text editor, copyleft 2021. Press any space to continue...",
           clear::All,
           termion::cursor::Goto(half_width, half_height),
           ).unwrap();


    match &editor.buffers[0].name {
        Some(val) => {
            write!(screen, "{}Filename: {}",
                   termion::cursor::Goto(1, height+1),
                   val).unwrap();
        },
        None => {
            write!(screen, "").unwrap();
        }

    }

    screen.flush().unwrap();
    let mut lines_drawn = false;
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(' ') => {
                if !lines_drawn {
                    writeln!(screen, "{}{}",
                           termion::cursor::Goto(1, height-1),
                           clear::BeforeCursor).unwrap();

                    draw_lines(&mut editor.buffers[0], &mut screen, height as usize - 1);
                    lines_drawn = true;
                }
            },
            Key::Ctrl('q') => {
                break
            },
            Key::Up => {
                if lines_drawn {
                    let buffer = &mut editor.buffers[0];
                    writeln!(&mut screen, "{}", termion::cursor::Goto(5, buffer.current_height-1)).unwrap();
                    buffer.current_height = buffer.current_height-1;
                }
            },
            Key::Down => {
                if lines_drawn {
                    let buffer = &mut editor.buffers[0];
                    writeln!(screen, "{}", termion::cursor::Goto(5,buffer.current_height+1)).unwrap();
                    buffer.current_height = buffer.current_height+1;
                }
            },
            Key::Right => {
                if lines_drawn {
                    let buffer = &mut editor.buffers[0];
                    writeln!(screen, "{}", termion::cursor::Goto(buffer.current_width+1,buffer.current_height)).unwrap();
                    buffer.current_width = buffer.current_width+1;
                }
            },
            Key::Left => {
                if lines_drawn {
                    let buffer = &mut editor.buffers[0];
                    writeln!(screen, "{}", termion::cursor::Goto(buffer.current_width-1,buffer.current_height)).unwrap();
                    buffer.current_width = buffer.current_width-1;
                }
            },
            _ => ()
        };
    }
}
