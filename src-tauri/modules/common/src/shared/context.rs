use sea_orm::{Database, DatabaseConnection, DbErr};
use serde::de::Error;
use anyhow::{bail, Result};

use crate::*;

use super::Settings;

#[derive(Debug, Clone)]
pub struct Context {
    settings: Settings,
    connection: DatabaseConnection,
}

impl Context {
    pub async fn common_db_pool(settings: &Settings, connection: &DatabaseConnection)  -> Result<Context> {
        Ok(Context { settings: settings.clone(), connection: connection.clone() })
    }

    pub async fn connect(
        settings: Settings,
    ) -> Result<Context> {
        let db_url = Settings::create_database_url(
            settings.commondb().user(), 
            settings.commondb().password(), 
            settings.commondb().host(), 
            settings.commondb().port(), 
            settings.commondb().database()
        );
        
        let connection = connect(db_url).await?;
        Ok(Context{
            settings,
            connection
        })
    }
}

pub async fn connect(db_url: String) -> Result<DatabaseConnection> {
    let result = Database::connect(&db_url).await;
    match result {
        Ok(pg_pool) => Ok(pg_pool),
        Err(_) => bail!("Cannot connect database"),
    }
}
