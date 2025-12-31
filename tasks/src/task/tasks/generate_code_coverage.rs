use std::{
    env,
    fs::{create_dir_all, remove_dir_all},
    process::Command,
};

use crate::task::{
    Task, executable::Executable, exit_status::StringifiedExitCode,
    util::get_project_root,
};

pub struct GenerateCodeCoverageTask {
    dependencies: Vec<Task>,
}

impl Executable for GenerateCodeCoverageTask {
    fn execute(&self) {
        self.clean_existing_artificats();
        self.execute_dependencies();
        self.create_target_lcov_directory();
        self.generate_lcov_report();
        self.generate_html_report();
    }
}

impl GenerateCodeCoverageTask {
    pub const TASK_NAME: &'static str = "code-coverage";
    const DEFAULT_TARGET_DIR: &'static str = "target";

    const DEFAULT_TARGET_LCOV_DIR: &'static str = "target/lcov";

    pub fn new(dependencies: Vec<Task>) -> Self {
        Self { dependencies }
    }

    fn execute_dependencies(&self) {
        for dependency in &self.dependencies {
            dependency.execute();
        }
    }

    fn clean_existing_artificats(&self) {
        println!(
            "[TASK: {}]: Cleaning existing artifacts...",
            GenerateCodeCoverageTask::TASK_NAME,
        );

        match remove_dir_all(GenerateCodeCoverageTask::DEFAULT_TARGET_DIR) {
            Ok(_) => {}
            Err(err) => {
                println!(
                    "[TASK: {}]: Failed to delete {} directory, {}",
                    GenerateCodeCoverageTask::TASK_NAME,
                    GenerateCodeCoverageTask::DEFAULT_TARGET_DIR,
                    err,
                );
            }
        }
    }

    fn create_target_lcov_directory(&self) {
        println!(
            "[TASK: {}]: Creating directory {}...",
            GenerateCodeCoverageTask::TASK_NAME,
            GenerateCodeCoverageTask::DEFAULT_TARGET_LCOV_DIR,
        );

        create_dir_all(GenerateCodeCoverageTask::DEFAULT_TARGET_LCOV_DIR)
            .expect(&format!(
                "[TASK: {}]: Failed to create {} directory.",
                GenerateCodeCoverageTask::TASK_NAME,
                GenerateCodeCoverageTask::DEFAULT_TARGET_LCOV_DIR,
            ));

        println!(
            "[TASK: {}]: Successfully created directory {}.",
            GenerateCodeCoverageTask::TASK_NAME,
            GenerateCodeCoverageTask::DEFAULT_TARGET_LCOV_DIR,
        );
    }

    fn generate_lcov_report(&self) {
        println!(
            "[TASK: {}]: Generating LCOV report...",
            GenerateCodeCoverageTask::TASK_NAME,
        );

        let cargo = env::var("CARGO").unwrap_or("cargo".to_string());
        let project_root = get_project_root();

        let result = Command::new(cargo)
            .current_dir(project_root)
            .args(&[
                "llvm-cov",
                "--workspace",
                "--lcov",
                "--exclude",
                "tasks",
                "--output-path",
                &format!("{}/lcov.info", GenerateCodeCoverageTask::DEFAULT_TARGET_LCOV_DIR),
            ])
            .status()
            .expect(&format!(
                "[TASK: {}]: Something went wrong while generating code coverage.",
                GenerateCodeCoverageTask::TASK_NAME,
            ));

        if !result.success() {
            panic!(
                "[TASK: {}]: Code coverage generation failed with exit code {}.",
                GenerateCodeCoverageTask::TASK_NAME,
                result.exit_code(),
            );
        }

        println!(
            "[TASK: {}]: Successfully generated LCOV report.",
            GenerateCodeCoverageTask::TASK_NAME
        );
    }

    fn generate_html_report(&self) {
        println!(
            "[TASK: {}]: Generating HTML report...",
            GenerateCodeCoverageTask::TASK_NAME,
        );

        let project_root = get_project_root();
        let result = Command::new("genhtml")
            .current_dir(project_root)
            .args(&[
                &format!("{}/lcov.info", GenerateCodeCoverageTask::DEFAULT_TARGET_LCOV_DIR),
                "-o",
                GenerateCodeCoverageTask::DEFAULT_TARGET_LCOV_DIR,
            ])
            .status()
            .expect(&format!(
                "[TASK: {}]: Something went wrong while generating HTML report from LCOV report.",
                GenerateCodeCoverageTask::DEFAULT_TARGET_LCOV_DIR,
            ));

        if !result.success() {
            panic!(
                "[TASK: {}]: LCOV to HTML report conversion failed with exit code {}.",
                GenerateCodeCoverageTask::DEFAULT_TARGET_LCOV_DIR,
                result.exit_code(),
            );
        }

        println!(
            "[TASK: {}]: Successfully generated HTML report.",
            GenerateCodeCoverageTask::TASK_NAME
        );
    }
}
