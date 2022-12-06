use config::{builder::DefaultState, Config, ConfigBuilder, ConfigError, File};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub host_name: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let dirs = directories::ProjectDirs::from("test", "bly", "rust").expect("unreachable");
        let config_path = dirs.config_dir().join("config.toml");
        if config_path.exists() {
            if let Some(path) = config_path.to_str() {
                Self::from_file(path)
            } else {
                Err(ConfigError::Message(
                    "config path must be a valid UTF-8 string".to_string(),
                ))
            }
        } else {
            default_config_builder().build()?.try_deserialize()
        }
    }

    pub fn from_file(cfg_path: &str) -> Result<Self, ConfigError> {
        let s = default_config_builder()
            .add_source(File::with_name(cfg_path))
            .build()?;

        s.try_deserialize()
    }
}

fn default_config_builder() -> ConfigBuilder<DefaultState> {
    // let dirs = directories::ProjectDirs::from("test", "bly", "rust").expect("unreachable");
    // let config_dir = dirs.config_dir();
    let host_name = "bly";

    Config::builder()
        .set_default("host", host_name.to_string())
        .expect("default host name")
}
