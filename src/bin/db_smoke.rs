use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, Row};
use std::env;

/// Read-only verification of the legacy TSchat MySQL schema.
/// This tool never creates, inserts, updates, or deletes any database object or row.
#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let url = database_url_from_env()
        .context("Set DATABASE_URL or DB_HOST/DB_PORT/DB_DATABASE/DB_USERNAME for the disposable test database")?;

    let pool = MySqlPoolOptions::new()
        .max_connections(2)
        .connect(&url)
        .await
        .context("Could not connect to the TSchat test database")?;

    // Verify the exact columns expected by the Rust client without reading user data.
    sqlx::query(
        "SELECT id, username, password, email, name, dp, mobile, chatcolor, chatfont, chatfontsize, chatfontstyle FROM tb_users LIMIT 0"
    )
    .fetch_all(&pool)
    .await
    .context("tb_users is missing or does not match the expected TSchat columns")?;

    sqlx::query("SELECT id, chat FROM tb_chat LIMIT 0")
        .fetch_all(&pool)
        .await
        .context("tb_chat is missing or does not match the expected TSchat columns")?;

    sqlx::query("SELECT id, username, fromuser, subject, message, isread FROM tb_pm LIMIT 0")
        .fetch_all(&pool)
        .await
        .context("tb_pm is missing or does not match the expected TSchat columns")?;

    let password_capacity: Option<u64> = sqlx::query_scalar(
        "SELECT CHARACTER_MAXIMUM_LENGTH FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'tb_users' AND COLUMN_NAME = 'password'"
    ).fetch_one(&pool).await.context("Could not inspect tb_users.password column length")?;
    match password_capacity {
        Some(length) if length >= 100 => println!("tb_users.password capacity: {length} characters (Argon2id-compatible)"),
        Some(length) => anyhow::bail!("tb_users.password is only {length} characters; Argon2id upgrade requires at least 100"),
        None => anyhow::bail!("Could not determine tb_users.password capacity"),
    }

    let auto_increment_ids: u64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND ((TABLE_NAME = 'tb_users' AND COLUMN_NAME = 'id') OR (TABLE_NAME = 'tb_pm' AND COLUMN_NAME = 'id')) AND EXTRA LIKE '%auto_increment%'"
    ).fetch_one(&pool).await.context("Could not inspect auto-increment IDs")?;
    if auto_increment_ids != 2 {
        anyhow::bail!("tb_users.id and tb_pm.id must both be AUTO_INCREMENT");
    }

    let charsets = sqlx::query(
        "SELECT TABLE_NAME, TABLE_COLLATION FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME IN ('tb_users','tb_chat','tb_pm') ORDER BY TABLE_NAME"
    ).fetch_all(&pool).await.context("Could not inspect table collations")?;
    for row in charsets {
        let table: String = row.try_get("TABLE_NAME")?;
        let collation: Option<String> = row.try_get("TABLE_COLLATION")?;
        println!("{table} collation: {}", collation.unwrap_or_else(|| "unknown".into()));
    }

    let user_count: u64 = sqlx::query_scalar("SELECT COUNT(*) FROM tb_users")
        .fetch_one(&pool)
        .await
        .context("Could not read tb_users count")?;

    let chat_row = sqlx::query("SELECT id, chat FROM tb_chat WHERE id = 1")
        .fetch_optional(&pool)
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

    pool.close().await;
    Ok(())
}

fn non_empty_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

fn database_url_from_env() -> Result<String> {
    if let Some(url) = non_empty_env("TSCHAT_TEST_DATABASE_URL")
        .or_else(|| non_empty_env("DATABASE_URL"))
    {
        return Ok(url);
    }

    let driver = non_empty_env("DB_CONNECTION").unwrap_or_else(|| "mysql".into());
    if driver != "mysql" && driver != "mariadb" {
        anyhow::bail!("DB_CONNECTION='{driver}' is not supported; expected mysql");
    }

    let host = non_empty_env("DB_HOST").context("DB_HOST is required")?;
    let port = non_empty_env("DB_PORT").unwrap_or_else(|| "3306".into());
    let database = non_empty_env("DB_DATABASE").context("DB_DATABASE is required")?;
    let username = non_empty_env("DB_USERNAME").context("DB_USERNAME is required")?;
    let password = non_empty_env("DB_PASSWORD").unwrap_or_default();
    let port: u16 = port.parse().context("DB_PORT must be a valid TCP port")?;

    Ok(format!(
        "mysql://{}:{}@{}:{}/{}",
        urlencoding::encode(&username),
        urlencoding::encode(&password),
        host,
        port,
        urlencoding::encode(&database)
    ))
}
