use std::path::PathBuf;
use std::io::{Error, Write};
use std::fs;
use std::str::Lines;


pub struct DefaultBuffer {
    pub name: Option<String>,
    pub file_path: Option<String>,
    pub line_count: usize,
    pub lines: Vec<(usize,String)>,
    pub current_x: u16,
    pub current_y: u16,
    pub shown_line: u16,
    pub shown_first: u16,
    pub first_char: u16,
    pub on_last: bool
}

pub enum Buffer {
    Anon(DefaultBuffer),
    Default(DefaultBuffer)
}

pub fn find_first_char(lines: usize) -> u16 {

    if lines<10 { 5 } else if (lines>9)&(lines<100) { 6 } else if (lines>99)&(lines<1000) { 7 } else if (lines>999)&(lines<1000) { 8 } else {9}
}

fn to_vec(file_str: &str) -> Vec<(usize, String)> {
    let line_iterator: Lines = file_str.lines();

    line_iterator.enumerate().map(|(index, x)| (index, x.to_owned())).collect()
}

impl Buffer {
    pub fn from_buffer(file_path: PathBuf, name: Option<String>) -> DefaultBuffer {
        let file_string = fs::read_to_string(&file_path).expect("Could not find file path");
        
        let line_vec = to_vec(file_string.as_ref());

        let lines = line_vec.len();

        let first_char = find_first_char(lines);


        DefaultBuffer {
            name,
            file_path: Some(file_path.to_owned().into_os_string().into_string().unwrap()),
            line_count: lines,
            lines: line_vec,
            current_x: 1,
            current_y: 1,
            shown_line: 0,
            shown_first: 0,
            first_char,
            on_last: false
        }
    }

    pub fn new_buffer() -> DefaultBuffer {
        let line_vec = vec![(0, " ".to_string())];
        DefaultBuffer { 
            name: None,
            file_path: None,
            line_count: 1,
            lines: line_vec,
            current_x: 1,
            current_y: 1,
            shown_line: 0,
            shown_first: 0,
            first_char: 5,
            on_last: false
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
        screen.flush().unwrap();
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_path) = &self.file_path {
            let mut file = fs::File::create(file_path)?;
            for (_num, row) in &mut self.lines {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }

        }
        Ok(())
    }

}

