use termion::color;
use termion::clear;
use termion::raw::IntoRawMode;
use std::io::Stdin;
use std::io::{Write, stdout, stdin};
use std::str::Lines;
use termion::event::Key;
use termion::input::TermRead;
use termion::screen::*;
use crate::editor::Editor;
use crate::buffer::Buffer;


pub fn draw_lines(screen: &mut impl Write, line_iterator: Vec<String>) {
    for (line_number, line) in line_iterator.iter().enumerate() {
        let line_number: u16 = (line_number+1) as u16;
        writeln!(screen, "{}{}  {}",
            termion::cursor::Goto(1, line_number),
            line_number,
            line).unwrap();
    }
    print!("{}",
        termion::cursor::Goto(1, 1)
    );
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
            Key::Ctrl('q') => {
                write!(screen, "{}", clear::All).unwrap();
                break
            },
            _ => ()
        };

    };
}

pub fn render_tui(editor: Editor<Buffer>) {
    let term_size: (u16, u16) = termion::terminal_size().expect("Don't use windows!");

    let stdin = stdin();

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let half_width = (term_size.0/2)-30;
    let half_height = term_size.1/2;
    let height = term_size.1;

    write!(screen, "{}{}Ant text editor, copyleft 2021. Press any space to continue...",
           clear::All,
           termion::cursor::Goto(half_width, half_height),
           ).unwrap();


    match &editor.buffers[0].name {
        Some(val) => {
            write!(screen, "{}Filename: {}",
                   termion::cursor::Goto(1, height),
                   val).unwrap();
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
                draw_lines(&mut screen, editor.buffers[0].buffer_by_line());

            
            },
            Key::Ctrl('q') => {
                write!(screen, "{}{}", clear::All, termion::cursor::Show).unwrap();
                break
            },
            _ => ()
        };
    }
}

//fn manage_keypress(stdin: &mut Stdin, stdout: &mut impl Write) {
//    for c in stdin.keys() {
//        match c.unwrap() {
//            Key::Char(' ') => {
//                writeln!(screen, "{}{}",
//                       termion::cursor::Goto(1, height-1),
//                       clear::BeforeCursor).unwrap();
//                draw_lines(&mut screen, editor.buffers[0].buffer_by_line());
//
//            
//            },
//           Key::Ctrl('q') => {
//                write!(screen, "{}{}", clear::All, termion::cursor::Show).unwrap();
//                break
//            },
//            _ => ()
//        };
//    }
// }
