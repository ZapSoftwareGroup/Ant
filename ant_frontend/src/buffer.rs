use std::path::Path;
use std::path::PathBuf;
use std::fs::{self, File};
use std::str::Lines;
use linecount::count_lines;

pub struct Buffer {
    pub name: Option<String>,
    pub file_path: PathBuf,
    pub file_string: String,
    pub lines: usize
}

impl Buffer {
    pub fn new_buffer(file_path: PathBuf, name: Option<String>) -> Buffer {
        let file_string = fs::read_to_string(&file_path).expect("Could not find file path");
        let file = File::open(&file_path).unwrap();
        let lines: usize = count_lines(file).unwrap();

        Buffer {
            name,
            file_path,
            file_string,
            lines
        }
        
    }

    pub fn buffer_by_line(&self) -> Vec<String> {
        let line_iterator: Lines = self.file_string.as_str().lines();
        line_iterator.map(|x| x.to_owned()).collect()
    }
}

