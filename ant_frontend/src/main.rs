use ant::cli;
use ant::tui;
use ant::editor::Editor;
use ant::buffer::Buffer;

fn main() {
    let input = cli::cli_matches();

    let blank: bool = if input == "".to_string() { true } else { false };

    if blank {
        tui::render_blank_tui();
    } else {
        let path = cli::find_full_path(input.as_ref());

        let buffer = Buffer {
            name: Some(input),
            file_path: path,
        };

        let buf_vec = vec![buffer];

        let editor = Editor {
            buffers: buf_vec
        };

        tui::render_tui(editor);

    }
}
