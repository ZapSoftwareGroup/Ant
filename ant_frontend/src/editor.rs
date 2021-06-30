use crate::buffer::Buffer;
use std::path::PathBuf;


pub struct Editor<T> {
    pub buffers: Vec<T>
}

impl Editor<Buffer> {
    pub fn new_buffer(&mut self, path: PathBuf, name: String) {
        let buffer = Buffer::new_buffer(path, Some(name));
        self.buffers.push(buffer);
        

    } 
}
