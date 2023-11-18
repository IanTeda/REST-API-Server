//! src/configuration.rs

///////////////////////////////////////////////////////////////////////////////
// CONFIG-RS
//
// https://github.com/mehcode/config-rs

// Configuration struct
#[derive(serde::Deserialize)]
pub struct Settings {
    pub server_port: u16,
}

// Get configuration from "configuration.yaml"
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml", // Configuration is pulled from this yaml file
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
