use termion::color;
use termion::clear;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;

pub fn render_blank_tui() {
    let term_size: (u16, u16) = termion::terminal_size().expect("Don't use windows!");

    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();

    let half_width = (term_size.0/2)-30;
    let half_height = term_size.1/2;

    write!(stdout, "{}{}Ant text editor, copyleft 2021. Press any key to continue...{}",
           clear::All,
           termion::cursor::Goto(half_width, half_height),
           termion::cursor::Hide).unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            _ => {
                termion::cursor::Goto(term_size.0, term_size.1);
                termion::cursor::Show;
                write!(stdout, "{}", clear::All).unwrap();
                break
            },
        }

    };
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

