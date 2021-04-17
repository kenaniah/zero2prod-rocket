#[derive(serde::Deserialize)]
pub struct Settings {
    pub database_url: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::Environment::default())?;
    settings.try_into()
}
