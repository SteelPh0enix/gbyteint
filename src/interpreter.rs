use command::Command;
use processor::Processor;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Interpreter<T> {
    processor: Processor<T>,
    commands: BTreeMap<String, Command<T>>,
}

#[derive(Debug)]
pub enum InterpreterError {
    CommandNotFound,
}

impl<T> Interpreter<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Interpreter {
            processor: Processor::new(),
            commands: BTreeMap::new(),
        }
    }

    pub fn commands(&self) -> &BTreeMap<String, Command<T>> {
        &self.commands
    }

    pub fn add_command(&mut self, opcode: &str, cmd: Command<T>) {
        self.commands.insert(String::from(opcode), cmd);
    }

    pub fn execute(&mut self, opcode: &str) -> Result<Option<T>, InterpreterError> {
        //(self.commands.get_mut(opcode).cmd_func)(&mut self.processor);

        if let Some(command) = self.commands.get_mut(opcode) {
            Ok((command.cmd_func)(&mut self.processor))
        } else {
            Err(InterpreterError::CommandNotFound)
        }
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

    fn get_test_interpreter() -> Interpreter<u8> {
        Interpreter::new()
    }

    #[test]
    fn create_command() {
        let cmd = get_test_command();

        assert_eq!(
            cmd.description(),
            "Push 0 on stack and increment stack pointer"
        );
    }

    #[test]
    fn create_interpreter() {
        let interpreter = get_test_interpreter();
        assert_eq!(interpreter.commands().len(), 0usize);
    }

    #[test]
    fn add_command_to_interpreter() {
        let mut interpreter = get_test_interpreter();
        let cmd = get_test_command();

        interpreter.add_command("t", cmd);

        assert_eq!(interpreter.commands().len(), 1usize);
        assert_eq!(
            interpreter.commands()["t"].description(),
            "Push 0 on stack and increment stack pointer"
        );
    }

    #[test]
    fn execute_command() {
        let mut interpreter = get_test_interpreter();
        let cmd = get_test_command();

        interpreter.add_command("t", cmd);

        //assert_eq!(interpreter.execute("t"), None);
        let exec = interpreter.execute("t");
        assert!(exec.is_ok());
        assert_eq!(exec.unwrap(), None);
        assert_eq!(interpreter.processor.stack_size(), 1usize);
        assert_eq!(interpreter.processor.stack[0], 0);
        assert_eq!(interpreter.processor.stack_ptr, 1usize);
    }

    #[test]
    fn execute_returning_command() {
        let mut interpreter = get_test_interpreter();
        let cmd = get_test_command();
        let cmd_ret = Command::new(
            Box::new(|p: &mut Processor<u8>| Some(p.stack[p.stack_ptr])),
            "Return value pointed by stack pointer",
        );

        interpreter.add_command("a", cmd);
        interpreter.add_command("r", cmd_ret);
        assert!(interpreter.execute("a").is_ok());

        let exec_result = interpreter.execute("r");
        assert!(exec_result.is_ok());
        assert_eq!(exec_result.unwrap().unwrap(), 0u8);
    }
}
