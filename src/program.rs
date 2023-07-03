use std::io;
use std::io::{Read, Write};

use crate::command::Command;
use crate::memory::Memory;

#[derive(Debug, PartialEq)]
pub struct Program {
    program_counter: usize,
    commands: Vec<Command>,
    memory: Memory
}

impl Program {
    pub fn new(commands: Vec<Command>, memory: Memory) -> Self {
        Program {
            program_counter: 0,
            commands,
            memory
        }
    }

    pub fn execute(&mut self) {
        while self.program_counter < self.commands.len() {
            match self.commands[self.program_counter] {
                Command::IncrementPointer => {
                    self.memory.increment_pointer()
                }
                Command::DecrementPointer => {
                    self.memory.decrement_pointer()
                }
                Command::IncrementCell => {
                    self.memory.increment_cell()
                }
                Command::DecrementCell => {
                    self.memory.decrement_cell()
                }
                Command::InputCharacter => {

                    if let Some(byte) = io::stdin().bytes().next() {
                        let value = byte.expect("Error reading byte from input.");
                        self.memory.set_cell(value);
                    }
                }
                Command::OutputCharacter => {
                    print!("{}", char::from(self.memory.get_cell()))
                }
            }

            self.program_counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_counter_increments() {
        let mut program = Program::new(vec![Command::IncrementCell], Memory::new());

        program.execute();

        assert_eq!(program.program_counter, 1);
    }

    #[test]
    fn test_program_counter_xxx() {
        let mut program = Program::new(vec![Command::InputCharacter], Memory::new());

        program.execute();
    }
}