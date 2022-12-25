use crate::*;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::env;
use config::{Config, ConfigError, File};
use std::str::FromStr;
use anyhow::Result;

const RUN_MODE: &str = "RUN_MODE";
const RDS_SECRET: &str = "RDS_SECRET";

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
    pub async fn instance() -> Result<Box<Settings>> {
        static mut SINGLETON: Option<Box<Settings>> = None;
        unsafe {
            if let Some(setting) = SINGLETON.clone() {
                return Ok(setting);
            }

            let setting: Box<Settings> = Box::new(Settings::new().await?);
            SINGLETON = Some(setting.clone());
            Ok(setting)
        }
    }

    pub async fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        // デフォルト値をセット
        s.merge(File::with_name("config/default"))?;
        // 環境毎の値をセット
        // TODO: EnumによるRUN_MODEのチェック
        let env = env::var(RUN_MODE).unwrap_or_else(|_| "local".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        let mut rds_secret_str: Option<String> = None;
        if let Some(str) = env::var(RDS_SECRET).ok() {
            // Secret Managerから環境変数で上書き
            rds_secret_str = Some(str);
        } 
        
        s.try_into()
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
