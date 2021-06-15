use ant::cli;
use ant::tui;


fn main() {
    let input = cli::cli_matches();

    let blank: bool = if input == "".to_string() { true } else { false };

    if blank {
        println!("No input provided, entering splash screen!");
        tui::render_blank_tui();
    } else {
        let path = cli::find_full_path(input.as_ref());

    }
}
