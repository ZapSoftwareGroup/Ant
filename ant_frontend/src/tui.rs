use std::io::Stdin;
use std::io::Write;
use crate::buffer::Buffer;
use crate::editor::Editor;
use crate::draw::{draw_lines, draw_statusline};
use crate::keybindings::get_key;


pub fn render_tui(editor: &mut Editor<Buffer>, screen: &mut impl Write, stdin: &mut Stdin) {
    let height = editor.terminal_height;

    let gay_buffer;

    match &mut editor.buffers[0] {
        Buffer::Default(buffer) => {


            write!(screen, "{}", termion::cursor::Goto(1, 1)).unwrap();

            draw_lines(screen, buffer, (height-2).into());
            buffer.set_position(screen, buffer.first_char, 1);
            buffer.shown_line = height-2;
            buffer.shown_first = 1;

            draw_statusline(screen, buffer);

            screen.flush().unwrap();

            gay_buffer = buffer
        },
        Buffer::Anon(buffer) => {

            write!(screen, "{}", termion::cursor::Goto(1, 1)).unwrap();

            draw_lines(screen, buffer, (height-2).into());
            buffer.set_position(screen, buffer.first_char, 1);
            buffer.shown_line = height-2;
            buffer.shown_first = 1;

            draw_statusline(screen, buffer);

            write!(screen, "").unwrap();


            screen.flush().unwrap();

            gay_buffer = buffer;
        }
    }

    get_key(screen, stdin, gay_buffer);
}
