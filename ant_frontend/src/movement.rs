use std::io::Write;
use crate::buffer::DefaultBuffer;


pub fn move_down(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    if buffer.current_y!=buffer.line_count as u16 {

        let height = buffer.current_y as usize;
        let width = buffer.current_x as usize;
        
        let possible_width = buffer.lines[height].chars().count();

        if !(width>=possible_width+5) {

            buffer.set_position(screen, buffer.current_x, buffer.current_y+1);

            screen.flush().unwrap();
        } else if possible_width==0 {
            buffer.set_position(screen, 5, buffer.current_y+1);
            screen.flush().unwrap();
        } 
    }
    
}

pub fn move_up(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    if buffer.current_y!=1 {
        buffer.set_position(screen, buffer.current_x, buffer.current_y-1);
        
        screen.flush().unwrap();

    }
}

pub fn move_left(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    if buffer.current_x!=5 {
        buffer.set_position(screen, buffer.current_x-1, buffer.current_y);

        screen.flush().unwrap();

    }
}

pub fn move_right(screen: &mut impl Write, buffer: &mut DefaultBuffer) {
    let height = buffer.current_y as usize;
    let width = buffer.current_x as usize;

    let possible_width = buffer.lines[height-1].chars().count();

    if width!=possible_width+4 {
        buffer.set_position(screen, buffer.current_x+1, buffer.current_y);

        screen.flush().unwrap();
    }
}
