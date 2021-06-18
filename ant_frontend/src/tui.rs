use termion::color;
use termion::clear;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin, self, BufRead};
use termion::event::Key;
use termion::input::TermRead;
use termion::screen::*;
use crate::editor::Editor;
use std::fs::File;


pub fn draw_lines(screen: &mut impl Write, line_iterator: &mut io::Result<io::Lines<io::BufReader<File>>>) {
    if let Ok(lines) = line_iterator {
        for (i, line) in lines.enumerate() {
            let c = i+1;
            if let Ok(read) = line {
                writeln!(screen, "{}{}  {}",
                    termion::cursor::Goto(1, c as u16),
                    c,
                    read).unwrap();
            }
        }
    }
}

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
            _ => {
                write!(screen, "{}", clear::All).unwrap();
                break
            }
        };

    };
}

pub fn render_tui(editor: Editor) {
    let term_size: (u16, u16) = termion::terminal_size().expect("Don't use windows!");

    let stdin = stdin();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let half_width = (term_size.0/2)-30;
    let half_height = term_size.1/2;
    let height = term_size.1;

    write!(screen, "{}{}Ant text editor, copyleft 2021. Press any space to continue...{}",
           clear::All,
           termion::cursor::Goto(half_width, half_height),
           termion::cursor::Hide).unwrap();


    match &editor.buffers[0].name {
        Some(val) => {
            write!(screen, "{}Filename: {}{}",
                   termion::cursor::Goto(1, height),
                   val,
                   termion::cursor::Hide).unwrap();
        },
        None => {
            write!(screen, "").unwrap();
        }

    }

    screen.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(' ') => {
                writeln!(screen, "{}{}",
                       termion::cursor::Goto(1, height-1),
                       clear::BeforeCursor).unwrap();

            match &editor.buffers[0].name {
                Some(_val) => {
                    draw_lines(&mut screen, &mut editor.buffers[0].buffer_by_line());
                },
                None => {
                    break
                }

            };
            },
            _ => {
                write!(screen, "{}{}", clear::All, termion::cursor::Show).unwrap();
                break
            }
        };

    };
}

