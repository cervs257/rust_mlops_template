//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use fundamentals_week4::config::Logging;
//! let config = Logging::new();
//! ```
//!
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use fundamentals_week4::config::Logging;
/// let config = Logging::new();
/// ```
///
///
/// Create a new instance of the Logging struct using custom values:
/// ```
/// use fundamentals_week4::config::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Debug,
///     destination: LogOutput::File("log.txt".to_string())
/// };
/// ```
///

pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput, // i might not want to expose this field -> make it private
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}
