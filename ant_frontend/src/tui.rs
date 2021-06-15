use termion::color;
use termion::clear;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::screen::*;

pub fn render_tui(input: Option<&str>) {
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
    match input {
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
                
            match input {
                Some(_val) => {
                    ()
                },
                None => {
                    break
                }

            };
            },
            _ => {
                write!(screen, "{}", clear::All).unwrap();
                break
            }
        };

    };
    write!(screen, "{}", termion::cursor::Show).unwrap();
}

