use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub database: DatabaseConfig,
    #[serde(default)]
    pub radio: RadioConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct RadioConfig {
    pub stations: Vec<RadioStation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RadioStation {
    pub name: String,
    pub url: String,
}

impl Default for RadioConfig {
    fn default() -> Self {
        let names = [
            "Музыка без АП",
            "Русский рок",
            "Европа Плюс",
            "Radio Cafe",
            "Дорожное радио",
            "Старое Радио",
            "VIP MIX Radio",
            "Record Dubstep",
            "Chillout Radio",
            "Радио Джаз",
            "Радио Книга",
            "Супердискотека 90-х",
            "Ambient Sleeping Pill",
            "DrumandBass.FM",
            "Tecktonik",
        ];
        Self {
            stations: names.iter().map(|name| RadioStation {
                name: (*name).to_string(),
                url: String::new(),
            }).collect(),
        }
    }
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
            radio: RadioConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        // Optional local environment file. Real process variables take precedence.
        let _ = dotenvy::dotenv();
        let path = Path::new("config.toml");
        if let Ok(contents) = fs::read_to_string(path) {
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let contents = toml::to_string_pretty(self)?;
        fs::write("config.toml", contents)?;
        Ok(())
    }

    fn env_value(name: &str) -> Option<String> {
        std::env::var(name).ok().filter(|value| !value.trim().is_empty())
    }

    pub fn mysql_url(&self) -> String {
        // Prefer a full URL. This supports DATABASE_URL used by common deployment tooling.
        if let Some(url) = Self::env_value("DATABASE_URL") {
            return url;
        }

        let host = Self::env_value("DB_HOST").unwrap_or_else(|| self.database.host.clone());
        let port = Self::env_value("DB_PORT")
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(self.database.port);
        let database = Self::env_value("DB_DATABASE").unwrap_or_else(|| self.database.name.clone());
        let username = Self::env_value("DB_USERNAME").unwrap_or_else(|| self.database.user.clone());
        let password = Self::env_value("DB_PASSWORD").unwrap_or_else(|| self.database.password.clone());

        format!(
            "mysql://{}:{}@{}:{}/{}",
            urlencoding::encode(&username),
            urlencoding::encode(&password),
            host,
            port,
            urlencoding::encode(&database)
        )
    }

    pub fn validate_database_driver() -> anyhow::Result<()> {
        if let Some(driver) = Self::env_value("DB_CONNECTION") {
            if driver != "mysql" && driver != "mariadb" {
                anyhow::bail!("DB_CONNECTION='{driver}' is not supported; this Rust port currently requires mysql");
            }
        }
        Ok(())
    }
}
