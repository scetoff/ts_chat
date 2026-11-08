#[path = "../config.rs"]
mod config;

use anyhow::{Context, Result};
use config::Config;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlConnection, MySqlSslMode},
    Connection, Executor, Row,
};
use std::time::{SystemTime, UNIX_EPOCH};

fn hex_literal(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    format!("CONVERT(X'{encoded}' USING utf8mb4)")
}

async fn new_connection(options: &MySqlConnectOptions) -> Result<MySqlConnection> {
    MySqlConnection::connect_with(options)
        .await
        .context("TCP/TLS/MySQL connect failed")
}

async fn verify_row_absent(options: &MySqlConnectOptions, username: &str) -> Result<bool> {
    let mut conn = new_connection(options).await?;
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM tb_users WHERE username = ?"
    )
    .bind(username)
    .fetch_one(&mut conn)
    .await
    .context("Could not verify that the probe row was rolled back")?;
    conn.close().await.ok();
    Ok(count == 0)
}

async fn run_probe(
    options: &MySqlConnectOptions,
    label: &str,
    username: &str,
    email: &str,
    prepared: bool,
) -> Result<bool> {
    println!();
    println!("=== {label} INSERT probe ===");

    let mut conn = new_connection(options).await?;
    conn.execute("START TRANSACTION")
        .await
        .context("Could not start the rollback-only transaction")?;

    // Same field order and representative values as TSchat registration.
    let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$diagnostic$not-a-real-password-hash";
    let insert_result = if prepared {
        sqlx::query(
            "INSERT INTO tb_users (username,password,email,name,dp,mobile,chatcolor,chatfont,chatfontsize,chatfontstyle) VALUES (?,?,?,?,?,?,?,?,?,?)"
        )
        .bind(username)
        .bind(password_hash)
        .bind(email)
        .bind(username)
        .bind("")
        .bind("+7(xxx)xxx-xx-xx")
        .bind("FF000000")
        .bind("Candara")
        .bind(16_i32)
        .bind("Regular")
        .execute(&mut conn)
        .await
        .map(|_| ())
    } else {
        let statement = format!(
            "INSERT INTO tb_users (username,password,email,name,dp,mobile,chatcolor,chatfont,chatfontsize,chatfontstyle) VALUES ({},{},{},{},{},{},{},{},16,'Regular')",
            hex_literal(username),
            hex_literal(password_hash),
            hex_literal(email),
            hex_literal(username),
            hex_literal(""),
            hex_literal("+7(xxx)xxx-xx-xx"),
            hex_literal("FF000000"),
            hex_literal("Candara"),
        );
        conn.execute(statement.as_str()).await.map(|_| ())
    };

    match insert_result {
        Ok(()) => {
            println!("INSERT: OK");
            let count_result: std::result::Result<i64, sqlx::Error> = sqlx::query_scalar(
                "SELECT COUNT(*) FROM tb_users WHERE username = ?"
            )
            .bind(username)
            .fetch_one(&mut conn)
            .await;
            match count_result {
                Ok(count) => println!("Row visible inside transaction: {count}"),
                Err(error) => {
                    println!("SELECT inside transaction failed: {error:?}");
                    let _ = conn.execute("ROLLBACK").await;
                    conn.close().await.ok();
                    return Ok(false);
                }
            }

            match conn.execute("ROLLBACK").await {
                Ok(_) => println!("ROLLBACK: OK"),
                Err(error) => {
                    println!("ROLLBACK failed: {error:?}");
                    conn.close().await.ok();
                    return Ok(false);
                }
            }
            conn.close().await.ok();

            let absent = verify_row_absent(options, username).await?;
            println!("Probe row absent after reconnect: {absent}");
            Ok(absent)
        }
        Err(error) => {
            println!("INSERT failed: {error:?}");
            // If the connection is still alive, explicitly roll back. If the
            // server reset it, MySQL rolls back the open InnoDB transaction.
            match conn.execute("ROLLBACK").await {
                Ok(_) => println!("ROLLBACK after failed INSERT: OK"),
                Err(rollback_error) => println!("ROLLBACK after failed INSERT also failed: {rollback_error:?}"),
            }
            conn.close().await.ok();

            match verify_row_absent(options, username).await {
                Ok(true) => println!("Probe row absent after reconnect: true"),
                Ok(false) => println!("WARNING: probe row exists after failed INSERT; inspect the database before proceeding"),
                Err(verify_error) => println!("Could not verify probe row after reconnect: {verify_error:#}"),
            }
            Ok(false)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("TSchat MySQL write diagnostic");
    println!("Uses config.toml/.env; credentials are not printed.");
    println!("Each test INSERT is run inside an InnoDB transaction and rolled back.");

    let _ = dotenvy::dotenv();
    Config::validate_database_driver()?;
    let config = Config::load();
    let options = config
        .mysql_url()
        .parse::<MySqlConnectOptions>()
        .context("Could not parse MySQL connection settings")?
        .ssl_mode(MySqlSslMode::Required);

    let mut conn = new_connection(&options).await?;
    let row = sqlx::query(
        "SELECT DATABASE() AS db_name, CURRENT_USER() AS account_name, @@version AS server_version, @@session.max_user_connections AS max_connections"
    )
    .fetch_one(&mut conn)
    .await
    .context("Connected but could not read MySQL session information")?;

    let db_name: String = row.try_get("db_name")?;
    let account_name: String = row.try_get("account_name")?;
    let server_version: String = row.try_get("server_version")?;
    let max_connections: i64 = row.try_get("max_connections")?;
    println!("Database: {db_name}");
    println!("MySQL account: {account_name}");
    println!("Server: {server_version}");
    println!("max_user_connections: {max_connections}");

    let table = sqlx::query(
        "SELECT ENGINE FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'tb_users'"
    )
    .fetch_optional(&mut conn)
    .await
    .context("Could not inspect tb_users engine")?;
    let engine: Option<String> = table.map(|row| row.try_get("ENGINE")).transpose()?;
    println!("tb_users engine: {}", engine.as_deref().unwrap_or("NOT FOUND"));
    conn.close().await.ok();

    if engine.as_deref() != Some("InnoDB") {
        anyhow::bail!("Expected tb_users to use InnoDB; no write probe was attempted");
    }

    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let prepared_user = format!("__tschat_diag_p_{stamp:x}");
    let simple_user = format!("__tschat_diag_s_{stamp:x}");
    let prepared_email = format!("{prepared_user}@example.invalid");
    let simple_email = format!("{simple_user}@example.invalid");

    let prepared_ok = run_probe(
        &options,
        "Prepared (SQLx bind parameters)",
        &prepared_user,
        &prepared_email,
        true,
    )
    .await
    .unwrap_or_else(|error| {
        println!("Prepared probe could not complete: {error:#}");
        false
    });

    let simple_ok = run_probe(
        &options,
        "Simple COM_QUERY (hex-encoded values)",
        &simple_user,
        &simple_email,
        false,
    )
    .await
    .unwrap_or_else(|error| {
        println!("Simple-query probe could not complete: {error:#}");
        false
    });

    println!();
    println!("=== SUMMARY ===");
    println!("Prepared INSERT + rollback: {}", if prepared_ok { "PASS" } else { "FAIL" });
    println!("Simple INSERT + rollback: {}", if simple_ok { "PASS" } else { "FAIL" });
    println!("No test account should remain in tb_users.");
    Ok(())
}
