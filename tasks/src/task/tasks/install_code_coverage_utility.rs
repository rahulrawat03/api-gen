use std::{env::consts, process::Command};

use crate::task::{
    executable::Executable, exit_status::StringifiedExitCode,
    util::get_project_root,
};

pub enum InstallCodeCoverageUtilityTask {
    MacOs(MacOs),
    // Add more if needed
    Other(Other),
}

impl Executable for InstallCodeCoverageUtilityTask {
    fn execute(&self) {
        match self {
            Self::MacOs(mac_os) => mac_os.execute(),
            Self::Other(other) => other.execute(),
        }
    }
}

impl InstallCodeCoverageUtilityTask {
    pub const TASK_NAME: &'static str = "install-code-coverage-utility";

    pub fn new() -> Self {
        let os = consts::OS;

        match os {
            "macos" => Self::MacOs(MacOs::new()),
            _ => Self::Other(Other::new(os.to_string())),
        }
    }
}

pub struct MacOs;

impl Executable for MacOs {
    fn execute(&self) {
        MacOs::install_lcov();
        MacOs::install_cargo_llvm_lcov();
    }
}

impl MacOs {
    fn new() -> Self {
        Self
    }

    fn install_lcov() {
        MacOs::install_package("lcov");
    }

    fn install_cargo_llvm_lcov() {
        MacOs::install_package("cargo-llvm-cov");
    }

    fn install_package(package_name: &str) {
        if MacOs::is_package_installed(package_name) {
            println!(
                "[TASK: {}]: `{}` is already installed.",
                InstallCodeCoverageUtilityTask::TASK_NAME,
                package_name
            );
            return;
        }

        println!(
            "[TASK: {}]: Installing `{}`...",
            InstallCodeCoverageUtilityTask::TASK_NAME,
            package_name
        );

        let project_root = get_project_root();
        let result = Command::new("brew")
            .current_dir(project_root)
            .args(&["install", package_name])
            .status()
            .expect(&format!(
                "[TASK: {}]: Something went wrong while installing `{}`.",
                InstallCodeCoverageUtilityTask::TASK_NAME,
                package_name,
            ));

        if !result.success() {
            panic!(
                "[TASK: {}]: `{}` installation failed with exit code {}.",
                InstallCodeCoverageUtilityTask::TASK_NAME,
                package_name,
                result.exit_code(),
            );
        }

        println!(
            "[TASK: {}]: Successfully installed `{}`.",
            InstallCodeCoverageUtilityTask::TASK_NAME,
            package_name
        );
    }

    fn is_package_installed(package_name: &str) -> bool {
        let project_root = get_project_root();
        let result = Command::new("brew")
            .current_dir(project_root)
            .args(&["ls", "--versions", package_name])
            .output();

        match result {
            Ok(output) => {
                output.status.success()
                    && String::from_utf8(output.stdout)
                        .map(|str| str.contains(package_name))
                        .unwrap_or(false)
            }
            Err(_) => false,
        }
    }
}

pub struct Other {
    os_name: String,
}

impl Executable for Other {
    fn execute(&self) {
        println!(
            "[TASK: {}]: Installation task not implemented for {}.",
            InstallCodeCoverageUtilityTask::TASK_NAME,
            &self.os_name
        );
    }
}

impl Other {
    fn new(os_name: String) -> Self {
        Self { os_name }
    }
}
