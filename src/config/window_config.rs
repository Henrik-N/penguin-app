use penguin_config::*;

#[derive(Debug, Clone, Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}
