use penguin_config::*;

#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub output_path: String,
    pub debug_message_severity: log::LevelFilter,
}
impl From<LoggerConfigDeserializable> for LoggerConfig {
    fn from(x: LoggerConfigDeserializable) -> Self {
        Self {
            output_path: x.output_path,
            debug_message_severity: x.debug_message_severity.into(),
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct LoggerConfigDeserializable {
    pub output_path: String,
    pub debug_message_severity: LogLevelFilter,
}



/// can't derive the necessary trait for the log::LevelFilter enum to serialize it since it's in
/// another crate, so this is an ugly fix
#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevelFilter {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
impl From<LogLevelFilter> for log::LevelFilter {
    fn from(level: LogLevelFilter) -> Self {
        match level {
            LogLevelFilter::Off => Self::Off,
            LogLevelFilter::Error => Self::Error,
            LogLevelFilter::Warn => Self::Warn,
            LogLevelFilter::Info => Self::Info,
            LogLevelFilter::Debug => Self::Debug,
            LogLevelFilter::Trace => Self::Trace,
        }
    }
}
