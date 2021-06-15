use ant::cli;
// use termion::clear;

fn main() {
    let input = cli::cli_matches();

    let blank: bool = if input == "".to_string() { true } else { false };

    if blank {
        println!("No input provided, entering splash screen!");
    } else {
        let path = cli::find_full_path(input.as_ref());
        println!("File provided: {}", path.display());
    }
}
