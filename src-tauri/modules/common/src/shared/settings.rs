use crate::*;
use anyhow::Result;
use config_file::{ConfigFileError, FromConfigFile};
use derive_getters::Getters;
use derive_new::new;
use serde::Deserialize;

#[derive(Debug, Clone, new, Getters, Deserialize)]
pub struct Environment {
    environment: String,
}

#[derive(Debug, Getters, Clone, new, Deserialize)]
pub struct Aws {
    region: String,
    account_id: String,
}

#[derive(Debug, Getters, Clone, new, Deserialize)]
pub struct Commondb {
    host: String,
    port: i32,
    user: String,
    database: String,
    password: String,
}

impl Commondb {
    pub fn database_url(&self) -> String {
        Settings::create_database_url(
            &self.user,
            &self.password,
            &self.host,
            &self.port,
            &self.database,
        )
    }
}

#[derive(Debug, Getters, Clone, Deserialize)]
pub struct Settings {
    environment: Environment,
    commondb: Commondb,
    aws: Aws,
}

impl Settings {
    pub fn instance() -> Result<Box<Settings>> {
        static mut SINGLETON: Option<Box<Settings>> = None;
        unsafe {
            if let Some(setting) = SINGLETON.clone() {
                return Ok(setting);
            }

            let setting: Box<Settings> = Box::new(Settings::new()?);
            SINGLETON = Some(setting.clone());
            Ok(setting)
        }
    }

    pub fn new() -> Result<Self, ConfigFileError> {
        let config = Settings::from_config_file(CONFIG_FILE)?;
        Ok(config)
    }

    pub fn create_database_url(
        user: &str,
        password: &str,
        host: &str,
        port: &i32,
        database: &str,
    ) -> String {
        format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}?sslmode=disable",
            user = user,
            password = password,
            host = host,
            port = port,
            database = database
        )
    }
}
