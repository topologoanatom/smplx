use std::{cell::Cell, sync::OnceLock};

use simplicityhl::tracker::TrackerLogLevel;

pub const DEFAULT_LOG_LEVEL: TrackerLogLevel = TrackerLogLevel::Debug;

/// Global config log level, initialized in `test::context::TestContext::new()`.
static CONFIG_LOG_LEVEL: OnceLock<TrackerLogLevel> = OnceLock::new();

pub fn set_config_log_level(level: TrackerLogLevel) {
    let _ = CONFIG_LOG_LEVEL.set(level);
}

pub fn get_config_log_level() -> Option<TrackerLogLevel> {
    CONFIG_LOG_LEVEL.get().copied()
}

thread_local! {
    /// Thread specific log level holder.
    static THREAD_LOG_LEVEL: Cell<TrackerLogLevel> = Cell::new(
        CONFIG_LOG_LEVEL.get().copied().unwrap_or(DEFAULT_LOG_LEVEL)
    );
}

pub fn set_tracker_log_level(level: TrackerLogLevel) {
    THREAD_LOG_LEVEL.with(|cell| cell.set(level));
}

pub fn get_tracker_log_level() -> TrackerLogLevel {
    THREAD_LOG_LEVEL.get()
}

/// Returns the log level and resets to config's level or `DEFAULT_LOG_LEVEL` if config is not initialized.
pub fn take_tracker_log_level() -> TrackerLogLevel {
    THREAD_LOG_LEVEL.with(|cell| {
        let taked_level = cell.get();
        let config_level = CONFIG_LOG_LEVEL.get().copied().unwrap_or(DEFAULT_LOG_LEVEL);
        cell.set(config_level);
        taked_level
    })
}

#[derive(Clone, Copy, Debug)]
pub struct LogLevel(pub TrackerLogLevel);

#[derive(thiserror::Error, Debug)]
pub enum LogLevelParseError {
    #[error("Log level should either be `none`, `debug`, `warning` or `trace`, got: {0}")]
    Invalid(String),
}

impl std::str::FromStr for LogLevel {
    type Err = LogLevelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(LogLevel(TrackerLogLevel::None)),
            "debug" => Ok(LogLevel(TrackerLogLevel::Debug)),
            "warning" => Ok(LogLevel(TrackerLogLevel::Warning)),
            "trace" => Ok(LogLevel(TrackerLogLevel::Trace)),
            _ => Err(LogLevelParseError::Invalid(s.to_string())),
        }
    }
}
