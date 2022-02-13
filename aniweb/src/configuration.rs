use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize)]
pub struct Setting {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

pub fn get_configuration() -> Result<Setting, config::ConfigError> {
    let mut settings = config::Config::new();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_dir = base_path.join("configuration");

    settings.merge(config::File::from(configuration_dir.join("base")))?;

    let environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());
    settings.merge(config::File::from(configuration_dir.join(&environment)))?;

    settings.try_into()
}
