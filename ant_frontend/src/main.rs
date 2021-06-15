use ant::cli;
use ant::tui;


fn main() {
    let input = cli::cli_matches();

    let blank: bool = if input == "".to_string() { true } else { false };

    if blank {
        tui::render_tui(None);
    } else {
        let _path = cli::find_full_path(input.as_ref());
        tui::render_tui(Some(input.as_ref()));

    }
}
