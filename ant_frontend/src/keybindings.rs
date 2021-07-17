use std::io::{Stdin, Write};
use crate::buffer::DefaultBuffer;
use termion::event::Key;
use termion::input::TermRead;
use crate::movement::*;
use crate::text::*;
use crate::prompt::*;

pub fn get_key(screen: &mut impl Write, stdin: &mut Stdin, buffer: &mut DefaultBuffer) {
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => {
                break
            },
            Key::Down => {
                if !buffer.in_prompt {
                    move_down(screen, buffer);
                }
            },
            Key::Up => {
                if !buffer.in_prompt {
                    move_up(screen, buffer);
                }
            },
            Key::Left => {
                if !buffer.in_prompt {
                    move_left(screen, buffer);
                }
            },
            Key::Right => {
                if !buffer.in_prompt {
                    move_right(screen, buffer);
                }
            },
            Key::Backspace => {
                if !buffer.in_prompt {
                    delete_char_or_newline(screen, buffer);
                } else {
                    prompt_delete_char(screen, buffer); 
                }
            },
            Key::Char(x) => {
                if !buffer.in_prompt {
                    if x == '\n' {
                        insert_newline(screen, buffer);
                        move_down(screen, buffer);
                        buffer.set_position(screen, buffer.first_char, buffer.current_y);
                    } else if x == '\t' {
                        insert_tab(screen, buffer);
                    } else {
                        insert_char_at_pos(screen, buffer, x);
                    }
                } else {
                    if x == '\n' {
                        prompt_return_prompt(screen, buffer);
                    } else {
                        prompt_insert_char(screen, buffer, x)
                    }
                }
            },
            Key::Ctrl('s') => {
                if !buffer.in_prompt {
                    buffer.save(screen).unwrap();
                }
            },
            _ => (),
        };

    }; 
}
