use std::{env, process::exit};

use crate::task::{Task, executable::Executable};

mod task;

const HELP_MESSAGE: &'static str = "
Available Tasks:
    code-coverage: Generates the test coverage report
    install-code-coverage-utility: Installs the utilities required to generate code coverage
";

fn main() {
    let task = parse_task();
    task.execute();
}

fn parse_task() -> Task {
    match env::args().nth(1) {
        Some(task) => match task.try_into() {
            Ok(task) => task,
            Err(err) => help(&err),
        },
        None => help("No task provided!"),
    }
}

fn help(error_message: &str) -> ! {
    eprintln!("{error_message}\n{HELP_MESSAGE}");
    exit(0);
}
