use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, Row};
use std::env;

/// Read-only verification of the legacy TSchat MySQL schema.
/// This tool never creates, inserts, updates, or deletes any database object or row.
#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("TSCHAT_TEST_DATABASE_URL")
        .context("Set TSCHAT_TEST_DATABASE_URL to the disposable test database URL")?;

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
        println!("tb_chat row id=1: missing (schema is valid, application initialization would create it)");
    }

    println!("TSchat MySQL schema: OK (tb_users, tb_chat, tb_pm)");
    println!("Registered user count: {user_count}");
    println!("No rows or schema objects were modified.");

    pool.close().await;
    Ok(())
}
