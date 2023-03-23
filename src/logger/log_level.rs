pub(crate) enum LogLevel {
    Debug,
    Verbose,
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    pub(crate) fn from_str(level: &str) -> Result<LogLevel, String> {
        match level.to_lowercase().as_str() {
            "debug" => Ok(LogLevel::Debug),
            "verbose" => Ok(LogLevel::Verbose),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "fatal" => Ok(LogLevel::Fatal),
            _ => Err(format!("Invalid log level: {}", level)),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match *self {
            LogLevel::Debug => "Debug".to_string(),
            LogLevel::Verbose => "Verbose".to_string(),
            LogLevel::Info => "Info".to_string(),
            LogLevel::Warn => "Warn".to_string(),
            LogLevel::Error => "Error".to_string(),
            LogLevel::Fatal => "Fatal".to_string(),
        }
    }
}

// implement display
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Verbose => write!(f, "verbose"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Fatal => write!(f, "fatal"),
        }
    }
}

impl PartialEq<Self> for LogLevel {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LogLevel::Debug, LogLevel::Debug) => true,
            (LogLevel::Verbose, LogLevel::Verbose) => true,
            (LogLevel::Info, LogLevel::Info) => true,
            (LogLevel::Warn, LogLevel::Warn) => true,
            (LogLevel::Error, LogLevel::Error) => true,
            _ => false,
        }
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &LogLevel) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (LogLevel::Debug, LogLevel::Debug) => Some(std::cmp::Ordering::Equal),
            (LogLevel::Debug, LogLevel::Verbose) => Some(std::cmp::Ordering::Less),
            (LogLevel::Debug, LogLevel::Info) => Some(std::cmp::Ordering::Less),
            (LogLevel::Debug, LogLevel::Warn) => Some(std::cmp::Ordering::Less),
            (LogLevel::Debug, LogLevel::Error) => Some(std::cmp::Ordering::Less),
            (LogLevel::Verbose, LogLevel::Debug) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Verbose, LogLevel::Verbose) => Some(std::cmp::Ordering::Equal),
            (LogLevel::Verbose, LogLevel::Info) => Some(std::cmp::Ordering::Less),
            (LogLevel::Verbose, LogLevel::Warn) => Some(std::cmp::Ordering::Less),
            (LogLevel::Verbose, LogLevel::Error) => Some(std::cmp::Ordering::Less),
            (LogLevel::Info, LogLevel::Debug) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Info, LogLevel::Verbose) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Info, LogLevel::Info) => Some(std::cmp::Ordering::Equal),
            (LogLevel::Info, LogLevel::Warn) => Some(std::cmp::Ordering::Less),
            (LogLevel::Info, LogLevel::Error) => Some(std::cmp::Ordering::Less),
            (LogLevel::Warn, LogLevel::Debug) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Warn, LogLevel::Verbose) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Warn, LogLevel::Info) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Warn, LogLevel::Warn) => Some(std::cmp::Ordering::Equal),
            (LogLevel::Warn, LogLevel::Error) => Some(std::cmp::Ordering::Less),
            (LogLevel::Error, LogLevel::Debug) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Error, LogLevel::Verbose) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Error, LogLevel::Info) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Error, LogLevel::Warn) => Some(std::cmp::Ordering::Greater),
            (LogLevel::Error, LogLevel::Error) => Some(std::cmp::Ordering::Equal),
            (LogLevel::Fatal, LogLevel::Fatal) => Some(std::cmp::Ordering::Equal),
            (LogLevel::Fatal, _) => Some(std::cmp::Ordering::Greater),
            (_, LogLevel::Fatal) => Some(std::cmp::Ordering::Less),
        }
    }
}