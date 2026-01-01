use std::{env, process::exit};

use crate::{
    arguments::Arguments,
    task::{Task, executable::Executable},
};

mod arguments;
mod task;

const HELP_MESSAGE: &'static str = "
Available Tasks:
    code-coverage: Generates the test coverage report (Use --use-lcov flag if LCOV report is needed)
    install-code-coverage-utility: Installs the utilities required to generate code coverage
";

fn main() {
    let (task, arguments) = parse_task();
    task.execute(&arguments);
}

fn parse_task() -> (Task, Arguments) {
    let args = env::args().collect::<Vec<_>>();

    let task_name = args.get(1).map(|t| t.to_string());
    let task: Task = match task_name {
        Some(task) => match task.try_into() {
            Ok(task) => task,
            Err(err) => help(&err),
        },
        None => help("No task provided!"),
    };

    let arguments = Arguments::parse(&args[2..]);

    (task, arguments)
}

fn help(error_message: &str) -> ! {
    eprintln!("{error_message}\n{HELP_MESSAGE}");
    exit(0);
}
