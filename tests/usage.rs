extern crate gbyteint;
use gbyteint::interpreter::Interpreter;
use gbyteint::command::Command;
use gbyteint::processor::Processor;

#[test]
fn create_interpreter() {
    let inp: Interpreter<u8> = Interpreter::new();
}

#[test]
fn create_interpreter_with_command() {
    let mut inp: Interpreter<u8> = Interpreter::new();

    inp.add_command(
        "p0",
        Command::new(
            Box::new(|p: &mut Processor<u8>| -> Option<u8> {
                p.stack.push(0);
                None
            }),
            "Push 0 on stack",
        ),
    );
}

#[test]
fn push_add_show_pop() {
    let mut inp: Interpreter<u8> = Interpreter::new();

    inp.add_command(
        "push0",
        Command::new(
            Box::new(|p: &mut Processor<u8>| -> Option<u8> {
                p.stack.push(0);
                p.stack_ptr += 1;
                None
            }),
            "Push 0 on stack",
        ),
    );
    inp.add_command(
        "pop",
        Command::new(
            Box::new(|p: &mut Processor<u8>| -> Option<u8> {
                p.stack.pop();
                p.stack_ptr -= 1;
                None
            }),
            "Remove last element from stack",
        ),
    );
    inp.add_command(
        "get",
        Command::new(
            Box::new(|p: &mut Processor<u8>| -> Option<u8> { Some(p.stack[p.stack_ptr]) }),
            "Return element pointed by stack pointer",
        ),
    );
    inp.add_command(
        "add",
        Command::new(
            Box::new(|p: &mut Processor<u8>| -> Option<u8> {
                p.stack[p.stack_ptr] += 1;
                None
            }),
            "Add 1 to value pointed by stack pointer",
        ),
    );

    inp.execute("push0").unwrap();
    inp.execute("add").unwrap();
    inp.execute("add").unwrap();
    inp.execute("add").unwrap();
    let result = inp.execute("get").unwrap();
    inp.execute("pop").unwrap();

    assert_eq!(result.unwrap(), 3u8);
}
