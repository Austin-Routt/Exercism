/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// The best way to convert an enum to a string is to implement the std::fmt::Display trait
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "[INFO]"),
            LogLevel::Warning => write!(f, "[WARNING]"),
            LogLevel::Error => write!(f, "[ERROR]")
        }
    }
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    return format!("{}: {}", level.to_string(),message);

}
pub fn info(message: &str) -> String {
    return format!("{}: {}", LogLevel::Info.to_string(),message);
}
pub fn warn(message: &str) -> String {
    return format!("{}: {}", LogLevel::Warning.to_string(),message);
}
pub fn error(message: &str) -> String {
    return format!("{}: {}", LogLevel::Error.to_string(),message);
}