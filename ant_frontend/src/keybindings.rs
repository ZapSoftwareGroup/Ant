use std::io::{Stdin, Write};
use crate::buffer::DefaultBuffer;
use termion::event::Key;
use termion::input::TermRead;
use crate::movement::*;
use crate::text::*;

pub fn get_key(screen: &mut impl Write, stdin: &mut Stdin, buffer: &mut DefaultBuffer) {
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => {
                break
            },
            Key::Down => {
                move_down(screen, buffer);
            },
            Key::Up => {
                move_up(screen, buffer);
            },
            Key::Left => {
                move_left(screen, buffer);
            },
            Key::Right => {
                move_right(screen, buffer);
            },
            Key::Backspace => {
                delete_char_or_newline(screen, buffer);
            },
            Key::Char(x) => {
                if x == '\n' {
                    insert_newline(screen, buffer);
                    move_down(screen, buffer);
                } else {
                    insert_char_at_pos(screen, buffer, x);
                    move_right(screen, buffer);
                }
            },
            _ => (),
        };

    }; 
}
