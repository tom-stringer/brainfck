use crate::command::Command;
use crate::memory::Memory;
use crate::program::Program;

pub fn parse_program(program: &str) -> Program {
    let mut commands: Vec<Command> = vec![];

    for char in program.chars() {
        let command = match char {
            '>' => Some(Command::IncrementPointer),
            '<' => Some(Command::DecrementPointer),
            '+' => Some(Command::IncrementCell),
            '-' => Some(Command::DecrementCell),
            ',' => Some(Command::InputCharacter),
            '.' => Some(Command::OutputCharacter),
            _ => None
        };

        if let Some(command) = command {
            commands.push(command);
        }
    }

    Program::new(commands, Memory::new())
}

#[cfg(test)]
mod tests {
    use crate::command::Command;
    use crate::memory::Memory;
    use crate::parser::parse_program;
    use crate::program::Program;

    #[test]
    fn test_parse_program() {
        let program = parse_program("><+-,.");

        let expected = Program::new(
            vec![
                Command::IncrementPointer,
                Command::DecrementPointer,
                Command::IncrementCell,
                Command::DecrementCell,
                Command::InputCharacter,
                Command::OutputCharacter
            ],
            Memory::new()
        );
        assert_eq!(expected, program);
    }
}