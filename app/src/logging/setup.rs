use std::env;

const LOG_LEVEL_ENV_VAR: &str = "RUST_LOG";
const DEFAULT_LOG_LEVEL: &str = "TRACE";

fn configure_log_level() {
    if let Err(_) = env::var(LOG_LEVEL_ENV_VAR) {
        unsafe {
            env::set_var(LOG_LEVEL_ENV_VAR, DEFAULT_LOG_LEVEL);
        }
    }
}

pub fn setup_logging() {
    configure_log_level();
    tracing_subscriber::fmt::init();
}
