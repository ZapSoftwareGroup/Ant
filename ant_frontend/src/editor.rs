use termion::screen::AlternateScreen;
use std::io::{stdin, stdout};
use crate::buffer::{DefaultBuffer};
use std::path::PathBuf;
use termion::raw::IntoRawMode;
use crate::tui;

pub struct Editor<T> {
    pub terminal_height: u16,
    pub terminal_width: u16,
    pub buffers: Vec<T>
}

impl Editor<DefaultBuffer> {
    pub fn new() -> Editor<DefaultBuffer> {
        let buffer = DefaultBuffer::new_buffer();
        let term_size: (u16, u16) = termion::terminal_size().expect("Don't use windows!");

        Editor {
            terminal_height: term_size.1,
            terminal_width: term_size.0,
            buffers: vec![buffer]
        }
    }

    pub fn from(name: String, path: PathBuf) -> Editor<DefaultBuffer> {
        let buffer = DefaultBuffer::from_buffer(path, Some(name));
        let term_size: (u16, u16) = termion::terminal_size().expect("Don't use windows!");

        Editor {
            terminal_height: term_size.1,
            terminal_width: term_size.0,
            buffers: vec![buffer]
        }
    }

    pub fn new_buffer(&mut self, path: PathBuf, name: Option<String>) {
        let buffer = DefaultBuffer::from_buffer(path, name);
        self.buffers.push(buffer);
    }

    pub fn run(&mut self) {

        let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        let mut stdin = stdin();
        tui::render_tui(self, &mut screen, &mut stdin);
    }
}
