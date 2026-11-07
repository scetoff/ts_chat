use anyhow::{Context, Result};
use sqlx::{mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlSslMode}, Row};
use std::{env, time::Duration};

/// Read-only verification of the legacy TSchat MySQL schema.
/// This tool never creates, inserts, updates, or deletes any database object or row.
#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let url = database_url_from_env()
        .context("Set DATABASE_URL or DB_HOST/DB_PORT/DB_DATABASE/DB_USERNAME for the disposable test database")?;

    // FreeDB supports TLS on port 3306. Require encrypted transport and
    // keep the pool small for shared hosting.
    let options = url.parse::<MySqlConnectOptions>()?
        .ssl_mode(MySqlSslMode::Required);
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(30))
        .test_before_acquire(true)
        .connect_with(options)
        .await
        .context("Could not connect to the TSchat test database")?;

    // Hold one server session throughout smoke checks. Free/shared MySQL accounts
    // often have max_user_connections set to 1.
    let mut conn = pool.acquire().await.context("Could not acquire the single MySQL session")?;

    // Verify the exact columns expected by the Rust client without reading user data.
    sqlx::query(
        "SELECT id, username, password, email, name, dp, mobile, chatcolor, chatfont, chatfontsize, chatfontstyle FROM tb_users LIMIT 0"
    )
    .fetch_all(&mut *conn)
    .await
    .context("tb_users is missing or does not match the expected TSchat columns. Check DB_DATABASE; for a new disposable database, run database/schema.mysql.sql manually before retrying")?;

    sqlx::query("SELECT id, chat FROM tb_chat LIMIT 0")
        .fetch_all(&mut *conn)
        .await
        .context("tb_chat is missing or does not match the expected TSchat columns. Check DB_DATABASE or manually prepare a disposable database with database/schema.mysql.sql")?;

    sqlx::query("SELECT id, username, fromuser, subject, message, isread FROM tb_pm LIMIT 0")
        .fetch_all(&mut *conn)
        .await
        .context("tb_pm is missing or does not match the expected TSchat columns. Check DB_DATABASE or manually prepare a disposable database with database/schema.mysql.sql")?;

    let password_capacity: Option<i64> = sqlx::query_scalar(
        "SELECT CHARACTER_MAXIMUM_LENGTH FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'tb_users' AND COLUMN_NAME = 'password'"
    ).fetch_one(&mut *conn).await.context("Could not inspect tb_users.password column length")?;
    match password_capacity {
        Some(length) if length >= 100 => println!("tb_users.password capacity: {length} characters (Argon2id-compatible)"),
        Some(length) => anyhow::bail!("tb_users.password is only {length} characters; Argon2id upgrade requires at least 100"),
        None => anyhow::bail!("Could not determine tb_users.password capacity"),
    }

    let auto_increment_ids: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND ((TABLE_NAME = 'tb_users' AND COLUMN_NAME = 'id') OR (TABLE_NAME = 'tb_pm' AND COLUMN_NAME = 'id')) AND EXTRA LIKE '%auto_increment%'"
    ).fetch_one(&mut *conn).await.context("Could not inspect auto-increment IDs")?;
    if auto_increment_ids != 2 {
        anyhow::bail!("tb_users.id and tb_pm.id must both be AUTO_INCREMENT");
    }

    // Avoid collation metadata probes: shared MySQL hosts may expose these
    // as binary values or restrict information_schema while table access works.

    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tb_users")
        .fetch_one(&mut *conn)
        .await
        .context("Could not read tb_users count")?;

    // Diagnose whether the connected database account has write access without
    // attempting any INSERT/UPDATE or changing user data.
    match sqlx::query("SHOW GRANTS").fetch_all(&mut *conn).await {
        Ok(rows) => {
            let grants: Vec<String> = rows
                .iter()
                .filter_map(|row| row.try_get::<String, _>(0).ok())
                .map(|grant| grant.to_ascii_uppercase())
                .collect();
            let has_insert = grants.iter().any(|grant| {
                let Some(rest) = grant.strip_prefix("GRANT ") else { return false; };
                let Some((privileges, _target)) = rest.split_once(" ON ") else { return false; };
                privileges.contains("ALL PRIVILEGES")
                    || privileges.split(',').any(|item| item.trim() == "INSERT")
            });
            let has_update = grants.iter().any(|grant| {
                let Some(rest) = grant.strip_prefix("GRANT ") else { return false; };
                let Some((privileges, _target)) = rest.split_once(" ON ") else { return false; };
                privileges.contains("ALL PRIVILEGES")
                    || privileges.split(',').any(|item| item.trim() == "UPDATE")
            });
            println!("Current account grants include INSERT: {has_insert}");
            println!("Current account grants include UPDATE: {has_update}");
        }
        Err(_) => println!("Current account grants: unavailable (SHOW GRANTS was not permitted)"),
    }

    let schema_size_mb: Option<f64> = sqlx::query_scalar(
        "SELECT ROUND(COALESCE(SUM(data_length + index_length), 0) / 1024 / 1024, 2) FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE()"
    )
    .fetch_one(&mut *conn)
    .await
    .context("Could not estimate current schema storage")?;
    println!(
        "Current schema storage: {:.2} MB (host quota may cover multiple databases)",
        schema_size_mb.unwrap_or(0.0)
    );

    let chat_row = sqlx::query("SELECT id, chat FROM tb_chat WHERE id = 1")
        .fetch_optional(&mut *conn)
        .await
        .context("Could not read shared chat row")?;

    if let Some(row) = chat_row {
        let chat_id: i32 = row.try_get("id")?;
        let chat_text: String = row.try_get("chat")?;
        println!("tb_chat row id=1: present ({} characters)", chat_text.chars().count());
        if chat_id != 1 {
            anyhow::bail!("Unexpected shared chat row id");
        }
    } else {
        anyhow::bail!("tb_chat row id=1 is missing; the application expects the original welcome row");
    }

    println!("TSchat MySQL schema: OK (tb_users, tb_chat, tb_pm)");
    println!("Registered user count: {user_count}");
    println!("No rows or schema objects were modified.");

    drop(conn);
    pool.close().await;
    Ok(())
}

fn non_empty_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

fn database_url_from_env() -> Result<String> {
    let driver = non_empty_env("DB_CONNECTION").unwrap_or_else(|| "mysql".into());
    if driver != "mysql" && driver != "mariadb" {
        anyhow::bail!("DB_CONNECTION='{driver}' is not supported; expected mysql");
    }

    // Prefer separate fields over DATABASE_URL when they are all supplied. This avoids
    // URL parsing problems when a database password contains reserved characters.
    if let (Some(host), Some(database), Some(username)) = (
        non_empty_env("DB_HOST"),
        non_empty_env("DB_DATABASE"),
        non_empty_env("DB_USERNAME"),
    ) {
        let port_text = non_empty_env("DB_PORT").unwrap_or_else(|| "3306".into());
        let port: u16 = port_text.parse().context("DB_PORT must be a valid TCP port")?;
        if port == 0 {
            anyhow::bail!("DB_PORT must be between 1 and 65535");
        }
        let password = non_empty_env("DB_PASSWORD").unwrap_or_default();
        return Ok(build_mysql_url(&host, port, &database, &username, &password));
    }

    if let Some(url) = non_empty_env("TSCHAT_TEST_DATABASE_URL")
        .or_else(|| non_empty_env("DATABASE_URL"))
    {
        validate_mysql_url(&url)?;
        return Ok(url);
    }

    anyhow::bail!(
        "Set DB_HOST, DB_DATABASE and DB_USERNAME (plus optional DB_PORT/DB_PASSWORD),          or provide a valid DATABASE_URL for the disposable test database"
    )
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

fn validate_mysql_url(url: &str) -> Result<()> {
    let parsed = url::Url::parse(url).map_err(|error| {
        anyhow::anyhow!(
            "DATABASE_URL is malformed ({error}). Encode reserved characters in the username/password              or configure DB_HOST, DB_PORT, DB_DATABASE, DB_USERNAME and DB_PASSWORD separately"
        )
    })?;
    if parsed.scheme() != "mysql" {
        anyhow::bail!("DATABASE_URL must use the mysql:// scheme");
    }
    if parsed.host_str().is_none() {
        anyhow::bail!("DATABASE_URL must include a database host");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{build_mysql_url, validate_mysql_url};

    #[test]
    fn mysql_url_encodes_credentials_and_database_name() {
        let url = build_mysql_url(
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
        let url = build_mysql_url("::1", 3306, "tschat", "user", "pass");
        let parsed = url::Url::parse(&url).expect("valid IPv6 MySQL URL");
        assert_eq!(parsed.host_str(), Some("[::1]"));
        assert_eq!(parsed.port(), Some(3306));
    }

    #[test]
    fn malformed_mysql_url_is_rejected_without_echoing_credentials() {
        let result = validate_mysql_url("mysql://user:secret@db.example.test:not-a-port/chat");
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(error.contains("DATABASE_URL is malformed"));
        assert!(!error.contains("secret"));
    }
}
