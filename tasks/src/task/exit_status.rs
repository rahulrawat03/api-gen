use std::process::ExitStatus;

pub trait StringifiedExitCode {
    fn exit_code(&self) -> String;
}

impl StringifiedExitCode for ExitStatus {
    fn exit_code(&self) -> String {
        self.code()
            .map(|code| code.to_string())
            .unwrap_or("null".to_string())
    }
}
