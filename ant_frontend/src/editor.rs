use crate::buffer;
use std::path::PathBuf;


pub struct Editor {
    pub buffers: Vec<buffer::Buffer>
}

impl Editor {
    pub fn new_buffer(&mut self, path: PathBuf, name: String) {
        let buffer = buffer::Buffer {
            name: Some(name),
            file_path: path
        };
        self.buffers.push(buffer);

    } 
}
