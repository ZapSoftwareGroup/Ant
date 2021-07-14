use clap::{Arg, App};
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::{self, Write};


pub fn cli_matches() -> String {
    let matches = App::new("Ant Text Editor")
                          .version("0.0.1")
                          .about("Another text editor, written in Rust and C")
                          .arg(Arg::with_name("INPUT")
        .help("Set file to edit")
                              .required(false)
                              .index(1))
                          .get_matches();
                              
    let input: &str = matches.value_of("INPUT").unwrap_or("");

    input.to_string()
}

pub fn find_full_path(path: &str) -> Result<PathBuf, io::Error> {
    let new_path = Path::new(path);

    if new_path.exists() {
        Ok(new_path.to_path_buf())
    } else {
        let mut file = File::create(new_path)?;

        file.write_all(b"\n")?;
        Ok(new_path.to_path_buf())
    }
}

pub fn find_name(input: &PathBuf) -> Option<String> {
    match input.file_name() {
        Some(val) => {
            match val.to_str() {
                Some(val) => {
                    Some(val.to_owned())
                },
                None => None
            }
        },
        None => {
            None
        }
    }
}
