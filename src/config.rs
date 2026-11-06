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
