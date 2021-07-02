use ant::cli;
use ant::tui;
use ant::editor::Editor;

fn main() {
    let input = cli::cli_matches();

    let blank: bool = if input == "".to_string() { true } else { false };

    if blank {
        tui::render_blank_tui();
    } else {
        let path = cli::find_full_path(input.as_ref());

        let mut editor = Editor {
            buffers: Vec::new()
        };

        editor.new_buffer(path, input);

        tui::render_tui(&mut editor);

    }
}
