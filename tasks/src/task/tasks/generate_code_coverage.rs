use std::{
    env,
    fs::{create_dir_all, remove_dir_all},
    process::Command,
};

use crate::{
    arguments::Arguments,
    task::{
        Task, executable::Executable, exit_status::StringifiedExitCode,
        util::get_project_root,
    },
};

pub struct GenerateCodeCoverageTask {
    dependencies: Vec<Task>,
}

impl Executable for GenerateCodeCoverageTask {
    fn execute(&self, arguments: &Arguments) {
        self.clean_existing_artificats();
        self.execute_dependencies(arguments);
        self.create_target_lcov_directory();

        if arguments
            .flags
            .contains(GenerateCodeCoverageTask::USE_LCOV_FLAG)
        {
            self.generate_lcov_coverage_report();
            self.generate_html_from_lcov_report();
        } else {
            self.generate_html_coverage_report();
        }
    }
}

impl GenerateCodeCoverageTask {
    pub const TASK_NAME: &'static str = "code-coverage";

    const DEFAULT_TARGET_DIR: &'static str = "target";
    const DEFAULT_TARGET_COVERAGE_DIR: &'static str = "target/coverage";

    const USE_LCOV_FLAG: &'static str = "use-lcov";

    pub fn new(dependencies: Vec<Task>) -> Self {
        Self { dependencies }
    }

    fn execute_dependencies(&self, arguments: &Arguments) {
        for dependency in &self.dependencies {
            dependency.execute(arguments);
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
            GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR,
        );

        create_dir_all(GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR)
            .expect(&format!(
                "[TASK: {}]: Failed to create {} directory.",
                GenerateCodeCoverageTask::TASK_NAME,
                GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR,
            ));

        println!(
            "[TASK: {}]: Successfully created directory {}.",
            GenerateCodeCoverageTask::TASK_NAME,
            GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR,
        );
    }

    fn generate_lcov_coverage_report(&self) {
        self.generate_code_coverage_report(
            "LCOV",
            &[
                "llvm-cov",
                "--lcov",
                "--workspace",
                "--exclude",
                "tasks",
                "--output-path",
                &format!(
                    "{}/lcov.info",
                    GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR
                ),
            ],
        );
    }

    fn generate_html_coverage_report(&self) {
        self.generate_code_coverage_report(
            "HTML",
            &[
                "llvm-cov",
                "--html",
                "--workspace",
                "--exclude",
                "tasks",
                "--output-dir",
                &format!(
                    "{}",
                    GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR
                ),
            ],
        );
    }

    fn generate_code_coverage_report(&self, format: &str, cli_args: &[&str]) {
        println!(
            "[TASK: {}]: Generating coverage report ({})...",
            GenerateCodeCoverageTask::TASK_NAME,
            format,
        );

        let cargo = env::var("CARGO").unwrap_or("cargo".to_string());
        let project_root = get_project_root();

        let result = Command::new(cargo)
            .current_dir(project_root)
            .args(cli_args)
            .status()
            .expect(&format!(
                "[TASK: {}]: Something went wrong while generating code coverage ({}).",
                GenerateCodeCoverageTask::TASK_NAME,
                format,
            ));

        if !result.success() {
            panic!(
                "[TASK: {}]: Code coverage generation ({}) failed with exit code {}.",
                GenerateCodeCoverageTask::TASK_NAME,
                format,
                result.exit_code(),
            );
        }

        println!(
            "[TASK: {}]: Successfully generated code coverage report ({}).",
            GenerateCodeCoverageTask::TASK_NAME,
            format,
        );
    }

    fn generate_html_from_lcov_report(&self) {
        println!(
            "[TASK: {}]: Generating HTML report from LCOV...",
            GenerateCodeCoverageTask::TASK_NAME,
        );

        let project_root = get_project_root();
        let result = Command::new("genhtml")
            .current_dir(project_root)
            .args(&[
                &format!("{}/lcov.info", GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR),
                "-o",
                GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR,
            ])
            .status()
            .expect(&format!(
                "[TASK: {}]: Something went wrong while generating HTML report from LCOV report.",
                GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR,
            ));

        if !result.success() {
            panic!(
                "[TASK: {}]: LCOV to HTML report conversion failed with exit code {}.",
                GenerateCodeCoverageTask::DEFAULT_TARGET_COVERAGE_DIR,
                result.exit_code(),
            );
        }

        println!(
            "[TASK: {}]: Successfully generated HTML report from LCOV report.",
            GenerateCodeCoverageTask::TASK_NAME
        );
    }
}
