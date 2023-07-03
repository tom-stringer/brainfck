extern crate brainfuck;

use std::fs;
use std::path::Path;

fn read_program(filename: &str) -> String {
    let path = format!("tests/programs/{}", filename);

    fs::read_to_string(Path::new(&path))
        .expect(&format!("Error reading file {}", filename))
}

#[test]
fn test_print_zero() {
    let program = read_program("print-zero.bf");

    brainfuck::interpret(&program);
}
