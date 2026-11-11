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
        let stations = [
            ("Музыка без АП", ""),
            ("Русский рок", ""),
            ("Европа Плюс", "https://europaplus.hostingradio.ru:8030/ep128"),
            ("Radio Cafe", ""),
            ("Дорожное радио", "https://dorognoe.hostingradio.ru/dorognoe"),
            ("Старое Радио", ""),
            ("VIP MIX Radio", ""),
            ("Record Dubstep", ""),
            ("Chillout Radio", ""),
            ("Радио Джаз", ""),
            ("Радио Книга", ""),
            ("Супердискотека 90-х", ""),
            ("Ambient Sleeping Pill", ""),
            ("DrumandBass.FM", ""),
            ("Tecktonik", ""),
            ("Радио Шансон", "https://chanson.hostingradio.ru:8041/chanson128.mp3"),
            ("DFM радио", "https://dfm.hostingradio.ru/dfm128.mp3"),
            ("Радио Рекорд · основной эфир", "https://radiorecord.hostingradio.ru/rr_main96.aacp"),
            ("Наше радио", "https://nashe1.hostingradio.ru/nashe-128.mp3"),
            ("НАШЕ · Панки хой!", "https://nashe1.hostingradio.ru:18000/nashepunks.mp3"),
            ("Радио JAZZ 89.1", "https://nashe1.hostingradio.ru/jazz-128.mp3"),
            ("Lo-Fi Hip-Hop · Chill", "https://stream.zeno.fm/0r0xa792kwzuv"),
            ("Nightwave Plaza", "https://radio.plaza.one/mp3"),
        ];
        Self {
            stations: stations.iter().map(|(name, url)| RadioStation {
                name: (*name).to_string(),
                url: (*url).to_string(),
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
        // Prefer separate fields when they are complete: credentials are URL-encoded here,
        // so passwords containing @, :, /, # or ? do not corrupt URL parsing.
        let host_env = Self::env_value("DB_HOST");
        let database_env = Self::env_value("DB_DATABASE");
        let username_env = Self::env_value("DB_USERNAME");

        if let (Some(host), Some(database), Some(username)) =
            (host_env, database_env, username_env)
        {
            let port = Self::env_value("DB_PORT")
                .and_then(|value| value.parse::<u16>().ok())
                .unwrap_or(self.database.port);
            let password = Self::env_value("DB_PASSWORD")
                .unwrap_or_else(|| self.database.password.clone());
            return Self::build_mysql_url(&host, port, &database, &username, &password);
        }

        // Fall back to a complete URL when individual connection fields are not supplied.
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

        Self::build_mysql_url(&host, port, &database, &username, &password)
    }

    fn build_mysql_url(host: &str, port: u16, database: &str, username: &str, password: &str) -> String {
        let formatted_host = if host.contains(':') && !host.starts_with('[') {
            format!("[{host}]")
        } else {
            host.to_string()
        };

        format!(
            "mysql://{}:{}@{}:{}/{}",
            urlencoding::encode(username),
            urlencoding::encode(password),
            formatted_host,
            port,
            urlencoding::encode(database)
        )
    }

    pub fn validate_database_driver() -> anyhow::Result<()> {
        if let Some(driver) = Self::env_value("DB_CONNECTION") {
            if driver != "mysql" && driver != "mariadb" {
                anyhow::bail!("DB_CONNECTION='{driver}' is not supported; this Rust port currently requires mysql");
            }
        }
        if let Some(port) = Self::env_value("DB_PORT") {
            let parsed: u16 = port.parse()
                .map_err(|_| anyhow::anyhow!("DB_PORT must be an integer between 1 and 65535"))?;
            if parsed == 0 {
                anyhow::bail!("DB_PORT must be between 1 and 65535");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn loads_a_single_configured_radio_station() {
        let config: super::Config = toml::from_str(
            r#"
                [database]
                host = "127.0.0.1"
                port = 3306
                user = "tester"
                password = "secret"
                name = "chat"

                [[radio.stations]]
                name = "My radio"
                url = "https://radio.example.test/live.mp3"
            "#
        ).expect("valid config with one radio station");

        assert_eq!(config.radio.stations.len(), 1);
        assert_eq!(config.radio.stations[0].name, "My radio");
        assert_eq!(config.radio.stations[0].url, "https://radio.example.test/live.mp3");
    }

    #[test]
    fn mysql_url_encodes_credentials_and_database_name() {
        let url = Config::build_mysql_url(
            "db.example.test",
            3307,
            "chat database",
            "user@example.test",
            "p@ss:word /?#",
        );
        let parsed = url::Url::parse(&url).expect("valid MySQL URL");
        assert_eq!(parsed.scheme(), "mysql");
        assert_eq!(parsed.host_str(), Some("db.example.test"));
        assert_eq!(parsed.port(), Some(3307));
        assert_eq!(parsed.username(), "user%40example.test");
        assert_eq!(parsed.password(), Some("p%40ss%3Aword%20%2F%3F%23"));
        assert_eq!(parsed.path(), "/chat%20database");
    }

    #[test]
    fn mysql_url_formats_ipv6_host() {
        let url = Config::build_mysql_url("::1", 3306, "tschat", "user", "pass");
        let parsed = url::Url::parse(&url).expect("valid IPv6 MySQL URL");
        assert_eq!(parsed.host_str(), Some("[::1]"));
        assert_eq!(parsed.port(), Some(3306));
    }
}
