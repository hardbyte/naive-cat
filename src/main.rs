use std::{io};
use std::fs::File;
use std::io::{BufReader, Read};
use clap::{App, Arg};
use std::io::BufRead;

const USAGE: &'static str = "Usage: cat [option] [FILE]";


fn main() {
    let app = App::new("cat")
        .version("0.1.0")
        .about(USAGE)
        .arg(
            Arg::with_name("input")
                .help("Input file")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("enumerate")
                .short("-n")
                .help("number output lines")

        );
    let args = app.get_matches();
    let input_filename = match args.value_of("input") {
        Some(filename) => filename,
        None => "-"
    };

    let include_line_numbers = args.is_present("enumerate");

    println!("{}", input_filename);
    print_lines_from_filename(input_filename, include_line_numbers);

}


fn print_lines_from_filename(input_filename: &str, include_line_numbers: bool) {
    match input_filename {
        "-" => {
            let stdin = io::stdin();
            let handle = stdin.lock();
            print_from_file_like_object(handle, include_line_numbers);
        }
        path => {
            // Try to open the file at the given path
            let file = File::open(path).expect("Unable to open file");
            if file.metadata().expect("couldn't read metadata").is_dir() {
                panic!("'{}' is a directory.", path);
            }
            print_from_file_like_object(file, include_line_numbers)
        }
    };
}

// This is too dynamic :-[
// fn print_lines_from_file(input_filename: &str) {
//     let filelike = match input_filename {
//         "-" => {
//             let stdin = io::stdin();
//             stdin.lock()
//         }
//         path => {
//             // Try to open the file at the given path
//             File::open(path).expect("Unable to open file")
//         }
//     };
//     print_from_file_like_object(filelike);
// }

fn print_from_file_like_object<T: std::io::Read>(filelike: T, include_line_numbers: bool) {
    let buffered_reader = BufReader::new(filelike);

    // Assume text data that can be split on lines.
    for (i, line) in buffered_reader.lines().enumerate() {
        if include_line_numbers {
            print!("{}", i);
        }
        println!("{}", line.expect("Invalid line"));
    }
}