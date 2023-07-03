#[derive(Debug, PartialEq)]
pub enum Command {
    IncrementPointer,
    DecrementPointer,
    IncrementCell,
    DecrementCell,
    InputCharacter,
    OutputCharacter
}
