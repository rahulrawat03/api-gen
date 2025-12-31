use crate::task::{
    executable::Executable,
    tasks::{
        generate_code_coverage::GenerateCodeCoverageTask,
        install_code_coverage_utility::InstallCodeCoverageUtilityTask,
    },
};

pub mod executable;
pub mod exit_status;
pub mod tasks;
pub mod util;

pub enum Task {
    GenerateCodeCoverage(GenerateCodeCoverageTask),
    InstallCodeCoverageUtility(InstallCodeCoverageUtilityTask),
}

impl Executable for Task {
    fn execute(&self) {
        match self {
            Self::GenerateCodeCoverage(task) => task.execute(),
            Self::InstallCodeCoverageUtility(task) => task.execute(),
        }
    }
}

impl TryFrom<String> for Task {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == GenerateCodeCoverageTask::TASK_NAME {
            let dependencies = vec![Task::InstallCodeCoverageUtility(
                InstallCodeCoverageUtilityTask::new(),
            )];
            return Ok(Self::GenerateCodeCoverage(
                GenerateCodeCoverageTask::new(dependencies),
            ));
        } else if value == InstallCodeCoverageUtilityTask::TASK_NAME {
            return Ok(Self::InstallCodeCoverageUtility(
                InstallCodeCoverageUtilityTask::new(),
            ));
        }

        Err("Invalid Task!".to_string())
    }
}
