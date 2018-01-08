#[derive(Debug)]
pub struct Processor<T> {
    pub stack: Vec<T>,
    pub stack_ptr: usize,
}

impl<T> Processor<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Processor {
            stack: vec![Default::default()],
            stack_ptr: 0usize,
        }
    }

    pub fn stack_size(&self) -> usize {
        self.stack.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_processor() {
        let processor: Processor<u8> = Processor::new();

        assert_eq!(processor.stack_size(), 0usize);
        assert_eq!(processor.stack_ptr, 0usize);
    }

    #[test]
    pub fn push_on_stack() {
        let mut processor: Processor<u8> = Processor::new();
        processor.stack.push(1u8);

        assert_eq!(processor.stack_size(), 1usize);
        assert_eq!(processor.stack_ptr, 0usize);
        assert_eq!(processor.stack[processor.stack_ptr], 0);
    }

    #[test]
    pub fn push_and_increment_ptr() {
        let mut processor: Processor<u8> = Processor::new();
        processor.stack.push(1u8);
        processor.stack_ptr += 1;

        assert_eq!(processor.stack_size(), 1usize);
        assert_eq!(processor.stack_ptr, 1usize);
        assert_eq!(processor.stack[processor.stack_ptr], 1);
    }
}
