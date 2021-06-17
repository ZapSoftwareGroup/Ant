use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Buffer {
    pub name: Option<String>,
    pub file_path: PathBuf
}

impl Buffer {
    pub fn buffer_by_line(&self) -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open(self.file_path.clone())?;
        Ok(io::BufReader::new(file).lines())
    }
}

