# Ant text editor

`ant` is yet another text editor, written in Rust and C. It aims to be small, fast, and efficient, while maintaining important features. Note that it only supports OSX and Linux.

Current version: 0.1.0 - otherwise known as Pre-Pre-Alpha!

## Why Another Text Editor?

This is mainly a learning project, but there are some good reasons to use `ant`:

- Written in Rust, ensuring memory, type, and thread safety while using the program.

- Features a load of configuration options that can make it as powerful as vim, or as simplistic as nano. (Not ready yet)

- Ant can also serve as an introduction to modal editors in the terminal. (Not ready yet)

- Find a text editor *faster than this. I'll wait.

\*This value can vary depending on the file.

## Features

List of features that are complete, and not so complete. Not in exact order:

- [x] Open and create a file

- [x] View contents of file

- [x] Accurately move through file

- [x] Insert characters in file

- [x] Allow saving file to disk

- [x] Allow anonymous buffers

- [ ] Allow longer lines

- [x] Use tabs instead of spaces

- [x] Pressing return mid-line should move rest of text to next line

- [x] Allow lines >1000

- [x] Require sudo to open or create a protected file

- [x] Display messages on save and such

- [ ] Allow multiple buffers

- [ ] Allow syntax highlighting

- [ ] Allow search

- [ ] Allow find and replace

- [ ] Allow easy suckless configuration

- [ ] Allow modal editing and Vim keybindings

- [ ] Allow editor configuration

- [ ] Allow macros

- [ ] Allow custom commands

- [ ] Allow moving to line by doing `:{linenumber}`

- [ ] Codebase refactor

- [ ] Undo and Redo

## Installation

Presently, the only way to install Ant is to build it from source.

1. Clone the repo, and `cd` into it

2. `cd` into the `ant_frontend` directory

3. Run `cargo build --release`

4. Now, move the binary from `./target/release/ant` to some directory on your system path

## Usage

Here are the most common usage examples for `ant`:

### Open a file

`ant file.txt`

### Exit the editor

Enter: `C-q`

## License

`ant` is licensed under the GNU/GPLv3
