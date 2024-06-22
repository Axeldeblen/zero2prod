#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct Configuration {
    pub database_settings: DatabaseSettings,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    return settings.try_deserialize();
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn connection_string_without_db_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
