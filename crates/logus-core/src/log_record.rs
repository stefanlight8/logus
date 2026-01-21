use crate::log_level::LogLevel;

pub struct LogRecord {
    pub level: LogLevel,
    pub message: String,
}
