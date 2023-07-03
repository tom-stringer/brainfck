const MEMORY_SIZE: usize = 30_000;

#[derive(Debug, PartialEq)]
pub struct Memory {
    pointer: usize,
    data: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            pointer: 0,
            data: [0; MEMORY_SIZE]
        }
    }

    pub fn get_cell(&self) -> u8 {
        self.data[self.pointer]
    }

    pub fn set_cell(&mut self, value: u8) {
        self.data[self.pointer] = value
    }

    pub fn increment_pointer(&mut self) {
        if self.pointer < MEMORY_SIZE - 1 {
            self.pointer += 1;
        } else {
            self.pointer = 0;
        }
    }

    pub fn decrement_pointer(&mut self) {
        if self.pointer > 0 {
            self.pointer -= 1;
        } else {
            self.pointer = MEMORY_SIZE - 1;
        }
    }

    pub fn increment_cell(&mut self) {
        let addition = self.data[self.pointer].overflowing_add(1);
        self.data[self.pointer] = addition.0;
    }

    pub fn decrement_cell(&mut self) {
        let subtraction = self.data[self.pointer].overflowing_sub(1);
        self.data[self.pointer] = subtraction.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_has_correct_size() {
        let memory = Memory::new();

        assert_eq!(MEMORY_SIZE, memory.data.len());
    }

    #[test]
    fn test_memory_initialised_to_zero() {
        let memory = Memory::new();

        for cell in memory.data {
            assert_eq!(0, cell);
        }
    }

    #[test]
    fn test_get_cell() {
        let mut memory = Memory::new();
        memory.data[0] = 1;

        assert_eq!(1, memory.get_cell())
    }

    #[test]
    fn test_set_cell() {
        let mut memory = Memory::new();

        memory.set_cell(1);

        assert_eq!(1, memory.get_cell())
    }

    #[test]
    fn test_increment_pointer() {
        let mut memory = Memory::new();

        memory.increment_pointer();

        assert_eq!(1, memory.pointer)
    }

    #[test]
    fn test_increment_pointer_at_memory_limit() {
        let mut memory = Memory::new();
        memory.pointer = MEMORY_SIZE - 1;

        memory.increment_pointer();

        assert_eq!(0, memory.pointer)
    }

    #[test]
    fn test_decrement_pointer() {
        let mut memory = Memory::new();
        memory.pointer = 1;

        memory.decrement_pointer();

        assert_eq!(0, memory.pointer)
    }

    #[test]
    fn test_decrement_pointer_at_zero() {
        let mut memory = Memory::new();

        memory.decrement_pointer();

        assert_eq!(MEMORY_SIZE - 1, memory.pointer)
    }

    #[test]
    fn test_increment_cell() {
        let mut memory = Memory::new();

        memory.increment_cell();

        assert_eq!(1, memory.data[0])
    }

    #[test]
    fn test_increment_cell_at_u8_max() {
        let mut memory = Memory::new();
        memory.data[0] = u8::MAX;

        memory.increment_cell();

        assert_eq!(0, memory.data[0])
    }

    #[test]
    fn test_decrement_cell() {
        let mut memory = Memory::new();
        memory.data[0] = 1;

        memory.decrement_cell();

        assert_eq!(0, memory.data[0])
    }

    #[test]
    fn test_decrement_cell_at_zero() {
        let mut memory = Memory::new();

        memory.decrement_cell();

        assert_eq!(u8::MAX, memory.data[0])
    }
}
