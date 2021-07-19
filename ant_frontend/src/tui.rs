use std::io::Stdin;
use std::io::Write;
use crate::buffer::DefaultBuffer;
use crate::editor::Editor;
use crate::draw::{draw_lines, draw_statusline};
use crate::keybindings::get_key;


pub fn render_tui(editor: &mut Editor<DefaultBuffer>, screen: &mut impl Write, stdin: &mut Stdin) {
    let height = editor.terminal_height;
    let buffer = &mut editor.buffers[0]; 

    draw_lines(screen, buffer, (height-2).into());
    buffer.set_position(screen, buffer.first_char, 1);
    buffer.shown_line = height-2;
    buffer.shown_first = 1;

    draw_statusline(screen, buffer);

    screen.flush().unwrap();


    get_key(screen, stdin, buffer);
}
