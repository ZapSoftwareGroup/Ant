# `ant`
`ant` is yet another text editor, written in Rust and C. It aims to be small, fast, and efficient, while maintaining important features.

## Why Another Text Editor?
This is mainly a learning project, but there are some good reasons to use `ant`:

- Much of it is written in Rust, ensuring memory, type, and thread safety while using the program.

- Features a load of configuration options that can make it as powerful as vim, or as simplistic as nano.

- Ant can also serve as an introduction to modal editors in the terminal.

- Editing a typical file in Ant can use less than 1 \*megabyte of ram.

\*This value can vary depending on the file.

## Installation
TBC

## Usage
Here are the most common usage examples for `ant`:

### Open a file
`ant file.txt`

### Exit the editor
Enter: `M-x quit`

## Library implementation
The library needs to be designed a certain way in order to fit the frontend. A frontend, GUI or terminal based, can be built around this implementation.

### String from filepath
Functions also exist that take a filepath. `libant_string_from_filepath` will take in a filepath as a string (guarunteed to be there), and return raw file data (string/bytes/whatever this is called in c).

Typically in the frontend, a new type called buffer is created which holds crucial data like filepath, file name. This type will have a method called `.to_string()` which will call 
the `libant_string_from_filepath` function.

All future functions will be performed on this raw data.

### Text manipulation
All text manipulation functions take in raw string. 

Another function is called, `read_next_line_of_file`. This function will return the next line of a string when called. The frontend should read this, and display it on each line.

## License
`ant` is licensed under the GNU/GPLv3
