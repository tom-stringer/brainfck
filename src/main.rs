extern crate brainfuck;

use std::{env, fs, process};
use std::path::Path;

fn main() {
    let program_path = env::args().nth(1)
        .unwrap_or_else(|| {
            println!("Provide the path to a Brainfuck program as the first argument.");
            process::exit(1);
        });

    let program_string = fs::read_to_string(Path::new(&program_path))
        .unwrap_or_else(|error| {
            println!("Error reading program at path {}.", &program_path);
            println!("{}", error);
            process::exit(1);
        });

    brainfuck::interpret(&program_string);
}
