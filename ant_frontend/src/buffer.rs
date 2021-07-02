use std::io::Write;
use std::path::PathBuf;
use std::fs::{self, File};
use std::str::Lines;

pub struct Buffer {
    pub name: Option<String>,
    pub file_path: PathBuf,
    pub file_string: String,
    pub lines: usize,
    pub current_height: u16,
    pub current_width: u16
}

impl Buffer {
    pub fn new_buffer(file_path: PathBuf, name: Option<String>) -> Buffer {
        let file_string = fs::read_to_string(&file_path).expect("Could not find file path");

        Buffer {
            name,
            file_path,
            file_string,
            lines: 0,
            current_height: 1,
            current_width: 1
        }
        
    }

    pub fn buffer_by_line(&mut self) -> Vec<String> {
        let line_iterator: Lines = self.file_string.as_str().lines();
        let line_vec: Vec<String> = line_iterator.map(|x| x.to_owned()).collect();
        self.lines = line_vec.len();
        

        line_vec
        
    } 

    pub fn current_position(&self) -> (u16, u16) {
        (self.current_width, self.current_height)
    }

}

