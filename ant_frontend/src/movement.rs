use std::io::Write;
use crate::buffer::DefaultBuffer;


pub fn move_down(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    buffer.set_position(screen, buffer.current_x, buffer.current_y+1);
    screen.flush().unwrap();
    
}

pub fn move_up(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    if buffer.current_y!=1 {
        buffer.set_position(screen, buffer.current_x, buffer.current_y-1);
        screen.flush().unwrap();
    }
}

pub fn move_left(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    buffer.set_position(screen, buffer.current_x-1, buffer.current_y);
    screen.flush().unwrap();
}

pub fn move_right(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    buffer.set_position(screen, buffer.current_x+1, buffer.current_y);
    screen.flush().unwrap();
}
