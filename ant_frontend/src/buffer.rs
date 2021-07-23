use std::ffi::OsStr;
use std::path::PathBuf;
use std::io::{Error, Write};
use std::fs;
use std::str::Lines;
use std::collections::HashMap;
use crate::message::display_message;
use crate::prompt::*;


pub struct DefaultBuffer {
    pub name: Option<String>,
    pub file_path: Option<String>,
    pub line_count: usize,
    pub lines: Vec<(usize,String)>,
    pub current_x: u16,
    pub current_y: u16,
    pub shown_line: u16,
    pub shown_first: u16,
    pub longest_shown: u16,
    pub first_char: u16,
    pub on_last: bool,
    pub starting_index: u16,
    pub filetype: Option<String>,
    pub in_prompt: bool,
    pub current_prompt: &'static str,
    pub prompt: String
}

pub fn find_first_char(lines: usize) -> u16 {

    if lines<10{ 
        5
    } else if (lines>9)&(lines<100) {
        6 
    } else if (lines>99)&(lines<1000) {
        7 
    } else if (lines>999)&(lines<10000) {
        8 
    } else if (lines>9999)&(lines<100000) {
        9
    } else if (lines>99999)&(lines<1000000) { 
        10
    } else if (lines>999999)&(lines<10000000) {
        11 
    } else if (lines>9999999)&(lines<100000000) { 
        12 
    } else { 13 }
}

fn to_vec(file_str: &str) -> Vec<(usize, String)> {
    let line_iterator: Lines = file_str.lines();

    line_iterator.enumerate().map(|(index, x)| (index, x.to_owned())).collect()
}

fn find_file_type(extension: &str) -> String {
    let mut file_map = HashMap::new();
    file_map.insert("", "Plain Text");
    file_map.insert("md", "Markdown");
    file_map.insert("org", "Emacs org-mode");
    file_map.insert("toml", "TOML");
    file_map.insert("rs", "Rust");
    file_map.insert("py", "Python");
    file_map.insert("c", "C");
    file_map.insert("h", "Header File");
    file_map.insert("cpp", "C++");
    file_map.insert("asm", "ChadLang");
    file_map.insert("js", "Javascript");
    file_map.insert("html", "HTML");
    file_map.insert("css", "CSS");
    file_map.insert("java", "Java");
    file_map.insert("ada", "Ada");
    file_map.insert("sh", "Shell Script");
    file_map.insert("cs", "C#");
    file_map.insert("go", "GoLang");
    file_map.insert("lua", "Lua");
    file_map.insert("php", "PHP");
    file_map.insert("pl", "Perl");
    file_map.insert("ebuild", "Gentoo Ebuild");
    file_map.insert("txt", "Plain Text");

    match file_map.get(extension) {
        Some(file) => file.to_string(),
        None => "Plain Text".to_string()
    }
}

impl DefaultBuffer {
    pub fn from_buffer(file_path: PathBuf, name: Option<String>) -> DefaultBuffer {
        let file_string = fs::read_to_string(&file_path).expect("Could not find file path");
        
        let line_vec = to_vec(file_string.as_ref());

        let lines = line_vec.len();

        let first_char = find_first_char(lines);
        
        let file_clone = file_path.clone();
        
        let filetype = match file_clone.extension().and_then(OsStr::to_str) {
            Some(extension) => find_file_type(extension),
            None => "Plain Text".to_string()
        };

        DefaultBuffer {
            name,
            file_path: Some(file_path.into_os_string().into_string().unwrap()),
            line_count: lines,
            lines: line_vec,
            current_x: 1,
            current_y: 1,
            shown_line: 0,
            shown_first: 0,
            longest_shown: 0,
            first_char,
            on_last: false,
            starting_index: 1,
            filetype: Some(filetype),
            in_prompt: false,
            current_prompt: "",
            prompt: String::new()
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
            longest_shown: 0,
            on_last: false,
            starting_index: 1,
            filetype: None,
            in_prompt: false,
            current_prompt: "",
            prompt: String::new()
        }
    }

    pub fn current_position(&self) -> (u16, u16) {
        (self.current_x, self.current_y)
    }

    pub fn set_position(&mut self, screen: &mut impl Write, x: u16, y: u16) {
        self.current_x = x;
        self.current_y = y;
        write!(screen, "{}", termion::cursor::Goto(x, y)).unwrap(); 
        screen.flush().unwrap();
    }

    pub fn save(&mut self, screen: &mut impl Write) -> Result<(), Error> {
        if let Some(file_path) = &self.file_path {
            let mut file = fs::File::create(file_path)?;
            for (_num, row) in &mut self.lines {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            let save_message;
            if let Some(name) = &self.name {
                save_message = format!("Successfully wrote to file \"{}\"", name);
            } else {
                save_message = format!("Couldn't write to unnamed file");
            }

            let (current_x, current_y) = (self.current_x, self.current_y);
            display_message(screen, save_message.as_str());
            self.set_position(screen, current_x, current_y);

        } else {
            self.in_prompt = true; 
            self.current_prompt = "Create a file: ";
            prompt(screen, self, self.current_prompt);
        }
        Ok(())
    }

}

