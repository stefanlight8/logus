#[derive(Clone, Eq, PartialEq)]
pub enum LogLevel {
    Warning,
    Info,
    Debug,
    Error,
    Trace,
    Custom(String),
}
