use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                host: "127.0.0.1".into(),
                port: 3306,
                user: "tschat".into(),
                password: String::new(),
                name: "db_TSchat".into(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = Path::new("config.toml");
        if let Ok(contents) = fs::read_to_string(path) {
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn mysql_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            urlencoding::encode(&self.database.user),
            urlencoding::encode(&self.database.password),
            self.database.host,
            self.database.port,
            urlencoding::encode(&self.database.name)
        )
    }
}
