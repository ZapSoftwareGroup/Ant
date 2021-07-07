use ant::cli;
// use ant::tui;
use ant::editor::Editor;

fn main() {
    let input = cli::cli_matches();

    let blank: bool = if input == "".to_string() { true } else { false };

    if blank {
        let mut editor = Editor::new();
        // tui::render_blank_tui();
        editor.run();
    } else {
        let path = cli::find_full_path(input.as_ref());
        let name = cli::find_name(&path);
        let name = match name {
            Some(val) => val,
            None => input
        };

        let mut editor = Editor::from(name, path);

        editor.run();

    }
}
