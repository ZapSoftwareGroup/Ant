use std::path::{PathBuf};
use std::fs;
use std::str::Lines;
use std::io::Write;


pub struct DefaultBuffer {
    pub name: Option<String>,
    pub file_path: Option<String>,
    pub file_string: Option<String>,
    pub line_count: usize,
    pub lines: Vec<(usize,String)>,
    pub current_x: u16,
    pub current_y: u16,
    pub shown_line: u16,
    pub shown_first: u16
}

pub enum Buffer {
    Anon(DefaultBuffer),
    Default(DefaultBuffer)
}

impl Buffer {
    pub fn from_buffer(file_path: PathBuf, name: Option<String>) -> DefaultBuffer {
        let file_string = fs::read_to_string(&file_path).expect("Could not find file path");

        let line_iterator: Lines = file_string.as_str().lines();

        let line_vec: Vec<(usize, String)> = line_iterator.enumerate().map(|(index, x)| (index, x.to_owned())).collect();
        let lines = line_vec.len();
        

        DefaultBuffer {
            name,
            file_path: Some(file_path.to_owned().into_os_string().into_string().unwrap()),
            file_string: Some(file_string),
            line_count: lines,
            lines: line_vec,
            current_x: 1,
            current_y: 1,
            shown_line: 0,
            shown_first: 0
        }
    }

    pub fn new_buffer() -> DefaultBuffer {
        DefaultBuffer { 
            name: None,
            file_path: None,
            file_string: None,
            line_count: 1,
            lines: vec![(1,"".to_string())],
            current_x: 1,
            current_y: 1,
            shown_line: 0,
            shown_first: 0
        }
    }
}

impl DefaultBuffer {

    pub fn current_position(&self) -> (u16, u16) {
        (self.current_x, self.current_y)
    }

    pub fn set_position(&mut self, screen: &mut impl Write, x: u16, y: u16) {
        self.current_x = x;
        self.current_y = y;
        write!(screen, "{}", termion::cursor::Goto(x, y)).unwrap(); 
    }
}

