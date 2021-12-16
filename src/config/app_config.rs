use penguin_config::*;
use crate::config::logger_config::{LoggerConfig, LoggerConfigDeserializable};
use crate::config::window_config::WindowConfig;


#[derive(Debug, Clone)]
pub struct AppConfig {
    pub logger_config: LoggerConfig,
    pub window_config: WindowConfig,
}
impl From<AppConfigDeserializable> for AppConfig {
    fn from(x: AppConfigDeserializable) -> Self {
        AppConfig {
            logger_config: x.logger_config.into(),
            window_config: x.window_config,
        }
    }
}
impl PenguinConfig for AppConfig {
    fn read_config() -> Self {
        let config = AppConfigDeserializable::read_config();
        let config: AppConfig = config.into();
        config
    }
}


#[derive(Debug, Deserialize, PenguinConfigFile)]
#[penguin_config_file_path(path = "app-config.json")]
pub struct AppConfigDeserializable {
    pub logger_config: LoggerConfigDeserializable,
    pub window_config: WindowConfig,
}
