use ant::cli;
use ant::editor::Editor;
use std::process;



fn main() {
    let input = cli::cli_matches();

    let blank: bool = if input == "".to_string() { true } else { false };

    if blank {
        let mut editor = Editor::new();
        editor.run();
    } else {
        let path = match cli::find_full_path(input.as_ref()) {
            Ok(p) => p,
            Err(_v) => {
                println!("Dude, you're not root. Put sudo (or doas) in front, and then we'll talk. Until then, cya!");
                process::exit(1)
            }
        };
        let name = cli::find_name(&path);
        let name = match name {
            Some(val) => val,
            None => input
        };

        let mut editor = Editor::from(name, path);

        editor.run();

    }
}
