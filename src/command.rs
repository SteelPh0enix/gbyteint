use std::fmt;
use processor::Processor;

pub struct Command<T> {
    cmd_description: String,
    pub cmd_func: Box<FnMut(&mut Processor<T>) -> Option<T>>,
}

impl<T> fmt::Debug for Command<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command ({})", self.cmd_description)
    }
}

impl<T> Command<T> {
    pub fn new(func: Box<FnMut(&mut Processor<T>) -> Option<T>>, description: &str) -> Self {
        Command {
            cmd_description: String::from(description),
            cmd_func: func,
        }
    }

    pub fn description(&self) -> &String {
        &self.cmd_description
    }

    pub fn execute(&mut self, processor: &mut Processor<T>) -> Option<T> {
        (self.cmd_func)(processor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_command() -> Command<u8> {
        Command::new(
            Box::new(|p: &mut Processor<u8>| {
                p.stack.push(0);
                p.stack_ptr += 1;
                None
            }),
            "Push 0 on stack and increment stack pointer",
        )
    }

    #[test]
    fn create_command() {
        let cmd = get_test_command();

        assert_eq!(
            cmd.description(),
            "Push 0 on stack and increment stack pointer"
        );
    }
}
