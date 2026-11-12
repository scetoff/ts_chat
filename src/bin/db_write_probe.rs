#[path = "../config.rs"]
mod config;

use anyhow::{Context, Result};
use config::Config;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlConnection, MySqlPoolOptions, MySqlSslMode},
    Connection, Executor, Row,
};
use std::{
    io::{self, Write},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

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
    drop(conn);
    Ok(count == 0)
}

async fn report_row_absence(options: &MySqlConnectOptions, username: &str) {
    match verify_row_absent(options, username).await {
        Ok(true) => println!("Probe row absent after reconnect: true"),
        Ok(false) => println!("WARNING: probe row still exists after reconnect; inspect it in phpMyAdmin"),
        Err(error) => println!("Could not verify probe row after reconnect: {error:#}"),
    }
}

async fn run_autocommit_probe(
    options: &MySqlConnectOptions,
    username: &str,
    email: &str,
) -> Result<bool> {
    println!();
    println!("=== Application-style autocommit INSERT probe ===");
    println!("This matches TSchat registration: one INSERT, no explicit transaction.");
    let _ = io::stdout().flush();

    let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$diagnostic$not-a-real-password-hash";
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

    println!("Opening fresh connection and sending one autocommit INSERT...");
    let _ = io::stdout().flush();
    let mut conn = new_connection(options).await?;
    match conn.execute(statement.as_str()).await {
        Ok(result) => println!("INSERT acknowledgement: OK ({} row(s) affected)", result.rows_affected()),
        Err(error) => println!("INSERT returned error: {error:?}"),
    }
    // Do not send a second command over the connection used for INSERT.
    drop(conn);

    println!("Opening a separate connection to check whether the account persisted...");
    let _ = io::stdout().flush();
    let mut verify = new_connection(options).await?;
    let row = sqlx::query(
        "SELECT COUNT(*) AS row_count FROM tb_users WHERE username = ? AND email = ? AND password = ?"
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .fetch_one(&mut verify)
    .await
    .context("Could not verify autocommit INSERT from a fresh connection")?;
    let count: i64 = row.try_get("row_count")?;
    let persisted = count > 0;
    println!("Probe account persisted after reconnect: {persisted}");

    if persisted {
        println!("Removing only this uniquely marked diagnostic account...");
        match sqlx::query(
            "DELETE FROM tb_users WHERE username = ? AND email = ? AND password = ?"
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .execute(&mut verify)
        .await
        {
            Ok(result) => println!("Diagnostic cleanup: {} row(s) deleted", result.rows_affected()),
            Err(error) => println!("WARNING: automatic cleanup failed: {error:?}; remove username {username} manually in phpMyAdmin"),
        }
    }

    drop(verify);
    if persisted {
        match verify_row_absent(options, username).await {
            Ok(absent) => println!("Probe account absent after cleanup: {absent}"),
            Err(error) => println!("Could not confirm cleanup after reconnect: {error:#}"),
        }
    }
    Ok(persisted)
}

async fn run_pool_registration_probe(
    options: &MySqlConnectOptions,
    username: &str,
    email: &str,
) -> Result<bool> {
    println!();
    println!("=== TSchat connection-pool registration probe ===");
    println!("Uses the same one-connection pool, pre-insert SELECT, and simple INSERT as Database::register.");
    let _ = io::stdout().flush();

    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Some(Duration::from_secs(5)))
        .test_before_acquire(true)
        .connect_with(options.clone())
        .await
        .context("Could not create TSchat-style MySQL connection pool")?;

    println!("Step 1/2: checking username and email through the pool...");
    let conflicts = sqlx::query(
        "SELECT (SELECT COUNT(*) FROM tb_users WHERE username = ?) AS username_count, (SELECT COUNT(*) FROM tb_users WHERE email = ?) AS email_count"
    )
    .bind(username)
    .bind(email)
    .fetch_one(&pool)
    .await
    .context("TSchat-style pre-insert conflict check failed")?;
    let username_count: i64 = conflicts.try_get("username_count")?;
    let email_count: i64 = conflicts.try_get("email_count")?;
    println!("Existing username matches: {username_count}; email matches: {email_count}");
    if username_count != 0 || email_count != 0 {
        pool.close().await;
        anyhow::bail!("Unique pool probe username/email unexpectedly already exist");
    }

    let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$diagnostic$not-a-real-password-hash";
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

    println!("Step 2/2: sending the application INSERT through the pool...");
    let _ = io::stdout().flush();
    let insert_result = (&pool).execute(statement.as_str()).await;
    match insert_result {
        Ok(result) => println!("Pool INSERT acknowledgement: OK ({} row(s) affected)", result.rows_affected()),
        Err(error) => println!("Pool INSERT returned error: {error:?}"),
    }

    pool.close().await;

    println!("Checking persisted state using a new direct connection...");
    let _ = io::stdout().flush();
    let mut verify = new_connection(options).await?;
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM tb_users WHERE username = ? AND email = ? AND password = ?"
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .fetch_one(&mut verify)
    .await
    .context("Could not verify TSchat-style pool INSERT from a fresh connection")?;
    let persisted = count > 0;
    println!("Pool probe account persisted after reconnect: {persisted}");

    if persisted {
        println!("Attempting to remove only this pool-probe account...");
        match sqlx::query(
            "DELETE FROM tb_users WHERE username = ? AND email = ? AND password = ?"
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .execute(&mut verify)
        .await
        {
            Ok(result) => println!("Pool probe cleanup: {} row(s) deleted", result.rows_affected()),
            Err(error) => println!("WARNING: pool probe cleanup failed: {error:?}; remove username {username} in phpMyAdmin"),
        }
    }
    drop(verify);
    if persisted {
        match verify_row_absent(options, username).await {
            Ok(absent) => println!("Pool probe account absent after cleanup: {absent}"),
            Err(error) => println!("Could not confirm pool probe cleanup: {error:#}"),
        }
    }
    Ok(persisted)
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
    let _ = io::stdout().flush();

    println!("Opening a fresh MySQL connection...");
    let _ = io::stdout().flush();
    let mut conn = new_connection(options).await?;
    conn.execute("START TRANSACTION")
        .await
        .context("Could not start the rollback-only transaction")?;
    println!("Transaction started; sending test INSERT...");
    let _ = io::stdout().flush();

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
                    match conn.execute("ROLLBACK").await {
                        Ok(_) => println!("ROLLBACK after SELECT failure: OK"),
                        Err(rollback_error) => println!("ROLLBACK after SELECT failure also failed: {rollback_error:?}"),
                    }
                    drop(conn);
                    report_row_absence(options, username).await;
                    return Ok(false);
                }
            }

            match conn.execute("ROLLBACK").await {
                Ok(_) => println!("ROLLBACK: OK"),
                Err(error) => {
                    println!("ROLLBACK failed: {error:?}");
                    drop(conn);
                    report_row_absence(options, username).await;
                    return Ok(false);
                }
            }
            drop(conn);

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
            drop(conn);

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
async fn main() {
    let result = run_diagnostic().await;
    if let Err(error) = &result {
        eprintln!("\nDIAGNOSTIC ERROR: {error:#}");
    }

    println!("\nДиагностика завершена. Нажмите Enter, чтобы закрыть это окно.");
    let _ = io::stdout().flush();
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);

    if result.is_err() {
        std::process::exit(1);
    }
}

async fn run_diagnostic() -> Result<()> {
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
    let max_connections: u64 = row.try_get("max_connections")?;
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
    let _ = io::stdout().flush();
    // Dropping the direct connection closes its socket without waiting for
    // a graceful COM_QUIT round trip from this hosted MySQL service.
    drop(conn);

    if engine.as_deref() != Some("InnoDB") {
        anyhow::bail!("Expected tb_users to use InnoDB; no write probe was attempted");
    }

    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let prepared_user = format!("__tschat_diag_p_{stamp:x}");
    let simple_user = format!("__tschat_diag_s_{stamp:x}");
    let prepared_email = format!("{prepared_user}@example.invalid");
    let simple_email = format!("{simple_user}@example.invalid");
    let autocommit_user = format!("__tschat_diag_auto_{stamp:x}");
    let autocommit_email = format!("{autocommit_user}@example.invalid");
    let pool_user = format!("__tschat_diag_pool_{stamp:x}");
    let pool_email = format!("{pool_user}@example.invalid");
    println!("Prepared probe username: {prepared_user}");
    println!("Simple-query probe username: {simple_user}");
    println!("Autocommit probe username: {autocommit_user}");
    println!("Pool probe username: {pool_user}");
    let _ = io::stdout().flush();

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

    let autocommit_ok = run_autocommit_probe(
        &options,
        &autocommit_user,
        &autocommit_email,
    )
    .await
    .unwrap_or_else(|error| {
        println!("Autocommit probe could not complete: {error:#}");
        false
    });

    let pool_ok = run_pool_registration_probe(
        &options,
        &pool_user,
        &pool_email,
    )
    .await
    .unwrap_or_else(|error| {
        println!("TSchat-style pool probe could not complete: {error:#}");
        false
    });

    println!();
    println!("=== SUMMARY ===");
    println!("Prepared INSERT + rollback: {}", if prepared_ok { "PASS" } else { "FAIL" });
    println!("Simple INSERT + rollback: {}", if simple_ok { "PASS" } else { "FAIL" });
    println!("Direct autocommit INSERT persisted: {}", if autocommit_ok { "YES (then cleanup attempted)" } else { "NO" });
    println!("TSchat-style pooled INSERT persisted: {}", if pool_ok { "YES (then cleanup attempted)" } else { "NO" });
    Ok(())
}
