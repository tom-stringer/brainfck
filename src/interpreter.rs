use crate::parser;

pub fn interpret(program_string: &str) {
    let mut program = parser::parse_program(&program_string);

    program.execute();
}