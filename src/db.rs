use anyhow::{Context, Result};
use sqlx::{mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlSslMode}, Executor, MySqlPool, Row};
use std::time::Duration;
use crate::{config::Config, models::{ChatStyle, PrivateMessage, User}};


/// Detect failures caused by a broken network/session rather than a schema mismatch.
fn is_transient_sqlx_error(error: &sqlx::Error) -> bool {
    match error {
        sqlx::Error::Io(_)
        | sqlx::Error::Tls(_)
        | sqlx::Error::Protocol(_)
        | sqlx::Error::PoolTimedOut
        | sqlx::Error::WorkerCrashed => true,
        sqlx::Error::Database(database_error) => {
            matches!(database_error.code().as_deref(), Some("2006" | "2013" | "2055"))
        }
        _ => false,
    }
}

fn is_transient_connection_error(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        cause
            .downcast_ref::<sqlx::Error>()
            .map(is_transient_sqlx_error)
            .unwrap_or(false)
    })
}

fn is_duplicate_key_error(error: &sqlx::Error) -> bool {
    matches!(
        error,
        sqlx::Error::Database(database_error)
            if database_error.code().as_deref() == Some("1062")
    )
}

/// Build a MySQL string expression from UTF-8 bytes without interpolating
/// user-controlled characters into SQL syntax. This is used only by the
/// simple-query protocol fallback for hosts that reset prepared INSERTs.
fn mysql_utf8_literal(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    format!("CONVERT(X'{encoded}' USING utf8mb4)")
}

#[derive(Clone)]
pub struct Database {
    pool: MySqlPool,
    connect_options: MySqlConnectOptions,
}

impl Database {
    pub async fn connect(config: &Config) -> Result<Self> {
        Config::validate_database_driver()?;
        Self::connect_url(&config.mysql_url()).await
    }

    /// Create the one-connection pool used by the desktop client and registration probes.
    async fn create_pool(options: &MySqlConnectOptions) -> Result<MySqlPool> {
        MySqlPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(30))
            // FreeDB may cap simultaneous sessions per account. Release an
            // idle pooled socket quickly so phpMyAdmin/other clients can connect.
            .idle_timeout(Some(Duration::from_secs(5)))
            .test_before_acquire(true)
            .connect_with(options.clone())
            .await
            .context("Не удалось подключиться к MySQL")
    }

    /// Connect using an explicit URL for the guarded disposable-database integration test.
    async fn connect_url(url: &str) -> Result<Self> {
        // FreeDB documents TLS support on port 3306. Require encryption and keep
        // a small pool for shared/free database hosting; validate idle sockets before reuse.
        let connect_options = url.parse::<MySqlConnectOptions>()?
            .ssl_mode(MySqlSslMode::Required);
        let pool = Self::create_pool(&connect_options).await?;
        Ok(Self { pool, connect_options })
    }


    /// Verify the legacy tables without changing schema or data.
    ///
    /// Run each metadata probe through the pool instead of pinning one physical
    /// session for the whole sequence. Some shared MySQL hosts reset idle/metadata
    /// probe connections; SQLx can validate or replace the pooled connection
    /// between probes (test_before_acquire is enabled in connect_url).
    pub async fn verify_schema(&self) -> Result<()> {
        match self.verify_schema_once().await {
            Ok(()) => Ok(()),
            Err(first_error) if is_transient_connection_error(&first_error) => {
                // A transport reset is not evidence of a missing table. Give the
                // host a moment and retry the read-only checks on a fresh pooled session.
                tokio::time::sleep(Duration::from_millis(350)).await;
                self.verify_schema_once().await.map_err(|retry_error| {
                    anyhow::anyhow!(
                        "Проверка схемы MySQL не удалась после переподключения.                          Первая ошибка: {first_error:#}. Повторная ошибка: {retry_error:#}"
                    )
                })
            }
            Err(error) => Err(error),
        }
    }

    async fn verify_schema_once(&self) -> Result<()> {
        // Validate all required columns and the shared-chat seed in one round-trip.
        // A few shared MySQL hosts reset TCP sessions between otherwise valid
        // metadata probes; keeping this read-only check to one statement avoids
        // depending on a connection surviving several sequential requests.
        let row = sqlx::query(
            "SELECT
                (SELECT COUNT(*) FROM tb_users
                    WHERE 1 = 0
                      AND id IS NOT NULL
                      AND username IS NOT NULL
                      AND password IS NOT NULL
                      AND email IS NOT NULL
                      AND name IS NOT NULL
                      AND dp IS NOT NULL
                      AND mobile IS NOT NULL
                      AND chatcolor IS NOT NULL
                      AND chatfont IS NOT NULL
                      AND chatfontsize IS NOT NULL
                      AND chatfontstyle IS NOT NULL) AS users_schema,
                (SELECT COUNT(*) FROM tb_chat
                    WHERE 1 = 0
                      AND id IS NOT NULL
                      AND chat IS NOT NULL) AS chat_schema,
                (SELECT COUNT(*) FROM tb_pm
                    WHERE 1 = 0
                      AND id IS NOT NULL
                      AND username IS NOT NULL
                      AND fromuser IS NOT NULL
                      AND subject IS NOT NULL
                      AND message IS NOT NULL
                      AND isread IS NOT NULL) AS pm_schema,
                (SELECT COUNT(*) FROM tb_chat WHERE id = 1) AS chat_seed"
        )
        .fetch_one(&self.pool)
        .await
        .context("Не удалось проверить структуру tb_users, tb_chat и tb_pm одним запросом; проверьте соединение и таблицы")?;

        let chat_seed_count: i64 = row
            .try_get("chat_seed")
            .context("Не удалось прочитать результат проверки tb_chat.id=1")?;
        if chat_seed_count == 0 {
            anyhow::bail!(
                "В tb_chat отсутствует строка id=1; выполните ручную инициализацию только для пустой тестовой БД"
            );
        }

        Ok(())
    }

    /// Optional bootstrap for an empty database. Never called during normal login.
    pub async fn initialize(&self) -> Result<()> {
        let statements = [
            "CREATE TABLE IF NOT EXISTS tb_chat (id INT NOT NULL PRIMARY KEY, chat LONGTEXT NOT NULL) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
            "CREATE TABLE IF NOT EXISTS tb_pm (id INT NOT NULL AUTO_INCREMENT PRIMARY KEY, username VARCHAR(200) NOT NULL, fromuser VARCHAR(200) NOT NULL, subject VARCHAR(300) NOT NULL, message TEXT NOT NULL, isread INT NOT NULL DEFAULT 0) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
            "CREATE TABLE IF NOT EXISTS tb_users (id INT NOT NULL AUTO_INCREMENT PRIMARY KEY, username VARCHAR(200) NOT NULL UNIQUE, password VARCHAR(200) NOT NULL, email VARCHAR(200) NOT NULL UNIQUE, name VARCHAR(200) NOT NULL, dp LONGTEXT NOT NULL, mobile VARCHAR(50) NOT NULL, chatcolor VARCHAR(100) NOT NULL, chatfont VARCHAR(100) NOT NULL, chatfontsize INT NOT NULL, chatfontstyle VARCHAR(40) NOT NULL) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
        ];
        for statement in statements {
            sqlx::query(statement).execute(&self.pool).await?;
        }
        sqlx::query("INSERT IGNORE INTO tb_chat (id, chat) VALUES (1, 'Добро пожаловать в Таверну Странника')")
            .execute(&self.pool).await?;
        Ok(())
    }


    pub async fn user_with_password(&self, username: &str) -> Result<Option<(User, String)>> {
        let row = sqlx::query(
            "SELECT id, username, password, email, name, dp, mobile, chatcolor, chatfont, chatfontsize, chatfontstyle FROM tb_users WHERE username = ?"
        ).bind(username).fetch_optional(&self.pool).await?;
        Ok(row.map(|r| {
            let user = User {
                id: r.get("id"),
                username: r.get("username"),
                email: r.get("email"),
                display_name: r.get("name"),
                avatar_base64: r.get("dp"),
                mobile: r.get("mobile"),
                chat_color: r.get("chatcolor"),
                chat_font: r.get("chatfont"),
                chat_font_size: r.get("chatfontsize"),
                chat_font_style: r.get("chatfontstyle"),
            };
            let password_hash: String = r.get("password");
            (user, password_hash)
        }))
    }

    pub async fn authenticate(&self, username: &str, password_hash: &str) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, username, email, name, dp, mobile, chatcolor, chatfont, chatfontsize, chatfontstyle FROM tb_users WHERE username = ? AND password = ?"
        )
        .bind(username).bind(password_hash)
        .fetch_optional(&self.pool).await?;
        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            display_name: r.get("name"),
            avatar_base64: r.get("dp"),
            mobile: r.get("mobile"),
            chat_color: r.get("chatcolor"),
            chat_font: r.get("chatfont"),
            chat_font_size: r.get("chatfontsize"),
            chat_font_style: r.get("chatfontstyle"),
        }))
    }

    async fn registration_conflicts(
        &self,
        username: &str,
        email: &str,
    ) -> Result<(i64, i64)> {
        // The legacy tb_users schema does not require UNIQUE indexes, so retain
        // the original username/email checks but combine them into one round-trip.
        let row = sqlx::query(
            "SELECT
                (SELECT COUNT(*) FROM tb_users WHERE username = ?) AS username_count,
                (SELECT COUNT(*) FROM tb_users WHERE email = ?) AS email_count"
        )
        .bind(username)
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .context("Не удалось проверить занятость никнейма и почты")?;

        let username_count: i64 = row.try_get("username_count")?;
        let email_count: i64 = row.try_get("email_count")?;
        Ok((username_count, email_count))
    }

    async fn registration_conflicts_with_retry(
        &self,
        username: &str,
        email: &str,
    ) -> Result<(i64, i64)> {
        match self.registration_conflicts(username, email).await {
            Ok(counts) => Ok(counts),
            Err(first_error) if is_transient_connection_error(&first_error) => {
                tokio::time::sleep(Duration::from_millis(500)).await;
                self.registration_conflicts(username, email)
                    .await
                    .context(format!(
                        "Проверка никнейма и почты не удалась после переподключения. Первая ошибка: {first_error:#}"
                    ))
            }
            Err(error) => Err(error),
        }
    }

    async fn insert_registration(
        &self,
        username: &str,
        password_hash: &str,
        email: &str,
        avatar_base64: &str,
    ) -> std::result::Result<(), sqlx::Error> {
        // Use MySQL's simple COM_QUERY protocol for this INSERT. Some shared
        // database gateways reset the TCP session on the prepared-statement
        // INSERT path even though reads and phpMyAdmin's ordinary INSERT work.
        // Every dynamic value is represented as a UTF-8 hex literal, so quotes,
        // backslashes and SQL metacharacters remain data rather than SQL syntax.
        let statement = format!(
            "INSERT INTO tb_users              (username,password,email,name,dp,mobile,chatcolor,chatfont,chatfontsize,chatfontstyle)              VALUES ({},{},{},{},{},{},{},{},16,'Regular')",
            mysql_utf8_literal(username),
            mysql_utf8_literal(password_hash),
            mysql_utf8_literal(email),
            mysql_utf8_literal(username),
            mysql_utf8_literal(avatar_base64),
            mysql_utf8_literal("+7(xxx)xxx-xx-xx"),
            mysql_utf8_literal("FF000000"),
            mysql_utf8_literal("Candara"),
        );
        (&self.pool).execute(statement.as_str()).await?;
        Ok(())
    }

    /// Confirm an INSERT whose acknowledgement may have been lost when MySQL
    /// reset the TCP connection. The fresh Argon2 hash makes this attempt
    /// distinguishable from an older account with the same username/email.
    async fn registration_was_persisted(
        &self,
        username: &str,
        password_hash: &str,
        email: &str,
    ) -> Result<bool> {
        let row = sqlx::query(
            "SELECT username, email, password FROM tb_users WHERE username = ? OR email = ? LIMIT 1"
        )
        .bind(username)
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .context("Не удалось проверить результат регистрации после сброса соединения")?;

        let Some(row) = row else {
            return Ok(false);
        };
        let saved_username: String = row.try_get("username")?;
        let saved_email: String = row.try_get("email")?;
        let saved_hash: String = row.try_get("password")?;
        Ok(saved_username == username && saved_email == email && saved_hash == password_hash)
    }

    async fn registration_was_persisted_with_retry(
        &self,
        username: &str,
        password_hash: &str,
        email: &str,
    ) -> Result<bool> {
        match self.registration_was_persisted(username, password_hash, email).await {
            Ok(saved) => Ok(saved),
            Err(first_error) if is_transient_connection_error(&first_error) => {
                tokio::time::sleep(Duration::from_millis(500)).await;
                self.registration_was_persisted(username, password_hash, email)
                    .await
                    .context(format!(
                        "Не удалось проверить результат регистрации после повторного подключения. Первая ошибка: {first_error:#}"
                    ))
            }
            Err(error) => Err(error),
        }
    }

    pub async fn register(
        &self,
        username: &str,
        password_hash: &str,
        email: &str,
        avatar_base64: &str,
    ) -> Result<()> {
        // Registration uses a fresh one-connection pool instead of reusing the
        // long-lived app pool that may have been through schema checks or sat idle
        // while the user filled the form. FreeDB/shared gateways can reset those
        // older sessions without SQLx having received a clean server shutdown.
        let registration_pool = Self::create_pool(&self.connect_options)
            .await
            .context("Не удалось открыть новое соединение MySQL для регистрации")?;
        let registration_db = Self {
            pool: registration_pool,
            connect_options: self.connect_options.clone(),
        };

        let result = registration_db
            .register_on_fresh_pool(username, password_hash, email, avatar_base64)
            .await;
        registration_db.pool.close().await;
        result
    }

    async fn register_on_fresh_pool(
        &self,
        username: &str,
        password_hash: &str,
        email: &str,
        avatar_base64: &str,
    ) -> Result<()> {
        let (username_count, email_count) = self
            .registration_conflicts_with_retry(username, email)
            .await?;
        if username_count > 0 {
            anyhow::bail!("Никнейм уже занят");
        }
        if email_count > 0 {
            anyhow::bail!("Эта почта уже используется");
        }

        match self.insert_registration(username, password_hash, email, avatar_base64).await {
            Ok(()) => Ok(()),
            Err(error) if is_duplicate_key_error(&error) => {
                if self.registration_was_persisted_with_retry(username, password_hash, email).await? {
                    Ok(())
                } else {
                    anyhow::bail!("Никнейм или почта уже используются");
                }
            }
            Err(first_error) if is_transient_sqlx_error(&first_error) => {
                // First reconcile the outcome: the server may have committed the
                // INSERT and only lost the acknowledgement. Never blindly repeat it.
                tokio::time::sleep(Duration::from_millis(500)).await;
                if self.registration_was_persisted_with_retry(username, password_hash, email)
                    .await
                    .context(format!("Первичная ошибка регистрации: {first_error}"))?
                {
                    return Ok(());
                }

                match self.insert_registration(username, password_hash, email, avatar_base64).await {
                    Ok(()) => Ok(()),
                    Err(retry_error) if is_duplicate_key_error(&retry_error) => {
                        if self.registration_was_persisted_with_retry(username, password_hash, email).await? {
                            Ok(())
                        } else {
                            anyhow::bail!("Никнейм или почта уже используются");
                        }
                    }
                    Err(retry_error) if is_transient_sqlx_error(&retry_error) => {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        if self.registration_was_persisted_with_retry(username, password_hash, email)
                            .await
                            .context(format!(
                                "Первичная ошибка: {first_error}. Повторная ошибка: {retry_error}"
                            ))?
                        {
                            Ok(())
                        } else {
                            Err(anyhow::anyhow!(
                                "MySQL дважды разорвал соединение во время регистрации; результат INSERT не подтверждён. Первая ошибка: {first_error}. Повторная ошибка: {retry_error}"
                            ))
                        }
                    }
                    Err(retry_error) => Err(anyhow::Error::new(retry_error)
                        .context(format!("Повторная попытка регистрации после ошибки соединения: {first_error}"))),
                }
            }
            Err(error) => Err(anyhow::Error::new(error).context("Не удалось вставить запись нового пользователя в tb_users")),
        }
    }

    pub async fn users_except(&self, current: &str) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT username FROM tb_users WHERE username <> ? ORDER BY username")
            .bind(current).fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| r.get("username")).collect())
    }

    pub async fn user_by_name(&self, username: &str) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, username, email, name, dp, mobile, chatcolor, chatfont, chatfontsize, chatfontstyle FROM tb_users WHERE username = ?"
        ).bind(username).fetch_optional(&self.pool).await?;
        Ok(row.map(|r| User {
            id: r.get("id"), username: r.get("username"), email: r.get("email"),
            display_name: r.get("name"), avatar_base64: r.get("dp"), mobile: r.get("mobile"),
            chat_color: r.get("chatcolor"), chat_font: r.get("chatfont"),
            chat_font_size: r.get("chatfontsize"), chat_font_style: r.get("chatfontstyle")
        }))
    }

    pub async fn read_chat(&self) -> Result<String> {
        let row = sqlx::query("SELECT chat FROM tb_chat WHERE id = 1")
            .fetch_one(&self.pool).await?;
        Ok(row.get("chat"))
    }

    pub async fn write_chat(&self, chat: &str) -> Result<()> {
        sqlx::query("UPDATE tb_chat SET chat = ? WHERE id = 1")
            .bind(chat).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn send_private_message(&self, to: &str, from: &str, subject: &str, message: &str) -> Result<()> {
        sqlx::query("INSERT INTO tb_pm (username,fromuser,subject,message,isread) VALUES (?,?,?,?,0)")
            .bind(to).bind(from).bind(subject).bind(message)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn mark_message_read(&self, message_id: i64, username: &str) -> Result<()> {
        sqlx::query("UPDATE tb_pm SET isread = 1 WHERE id = ? AND username = ?")
            .bind(message_id).bind(username)
            .execute(&self.pool).await?;
        Ok(())
    }

    /// Delete exactly one message from the authenticated user's own inbox.
    pub async fn delete_private_message(&self, message_id: i64, username: &str) -> Result<u64> {
        let result = sqlx::query("DELETE FROM tb_pm WHERE id = ? AND username = ?")
            .bind(message_id)
            .bind(username)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn inbox(&self, username: &str) -> Result<Vec<PrivateMessage>> {
        let rows = sqlx::query("SELECT id,fromuser,subject,message,isread FROM tb_pm WHERE username = ? ORDER BY id DESC")
            .bind(username).fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| PrivateMessage {
            id: r.get("id"), from_user: r.get("fromuser"), subject: r.get("subject"),
            message: r.get("message"), is_read: r.get::<i32,_>("isread") != 0
        }).collect())
    }

    pub async fn update_profile_with_messages(
        &self,
        old_username: &str,
        username: &str,
        email: &str,
        display_name: &str,
        mobile: &str,
        password_hash: Option<&str>,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let username_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tb_users WHERE username = ? AND username <> ?"
        ).bind(username).bind(old_username).fetch_one(&mut *tx).await?;
        if username_count > 0 { anyhow::bail!("Никнейм уже занят"); }

        let email_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tb_users WHERE email = ? AND username <> ?"
        ).bind(email).bind(old_username).fetch_one(&mut *tx).await?;
        if email_count > 0 { anyhow::bail!("Эта почта уже используется"); }

        sqlx::query("UPDATE tb_users SET username = ?, email = ?, name = ?, mobile = ? WHERE username = ?")
            .bind(username).bind(email).bind(display_name).bind(mobile).bind(old_username)
            .execute(&mut *tx).await?;

        if old_username != username {
            sqlx::query("UPDATE tb_pm SET username = ? WHERE username = ?")
                .bind(username).bind(old_username).execute(&mut *tx).await?;
            sqlx::query("UPDATE tb_pm SET fromuser = ? WHERE fromuser = ?")
                .bind(username).bind(old_username).execute(&mut *tx).await?;
        }
        if let Some(hash) = password_hash {
            sqlx::query("UPDATE tb_users SET password = ? WHERE username = ?")
                .bind(hash).bind(username).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn chat_styles(&self) -> Result<std::collections::HashMap<String, ChatStyle>> {
        let rows = sqlx::query(
            "SELECT username, chatcolor, chatfont, chatfontsize, chatfontstyle FROM tb_users"
        ).fetch_all(&self.pool).await?;
        let mut styles = std::collections::HashMap::with_capacity(rows.len());
        for row in rows {
            let username: String = row.get("username");
            let style = ChatStyle {
                color: row.get("chatcolor"),
                font: row.get("chatfont"),
                font_size: row.get("chatfontsize"),
                font_style: row.get("chatfontstyle"),
            };
            styles.insert(username, style);
        }
        Ok(styles)
    }

    pub async fn update_chat_style(
        &self,
        username: &str,
        color: &str,
        font: &str,
        font_size: i32,
        font_style: &str,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE tb_users SET chatcolor = ?, chatfont = ?, chatfontsize = ?, chatfontstyle = ? WHERE username = ?"
        )
        .bind(color)
        .bind(font)
        .bind(font_size)
        .bind(font_style)
        .bind(username)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_avatar(&self, username: &str, avatar_base64: &str) -> Result<()> {
        sqlx::query("UPDATE tb_users SET dp = ? WHERE username = ?")
            .bind(avatar_base64).bind(username)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_password(&self, username: &str, password_hash: &str) -> Result<()> {
        sqlx::query("UPDATE tb_users SET password = ? WHERE username = ?")
            .bind(password_hash).bind(username)
            .execute(&self.pool).await?;
        Ok(())
    }
}


#[cfg(test)]
mod sql_literal_tests {
    use super::mysql_utf8_literal;

    #[test]
    fn user_text_is_encoded_as_hex_and_cannot_break_out_of_literal() {
        assert_eq!(
            mysql_utf8_literal("a'b\\c"),
            "CONVERT(X'6127625C63' USING utf8mb4)"
        );
        assert_eq!(mysql_utf8_literal(""), "CONVERT(X'' USING utf8mb4)");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::Database;
    use anyhow::{Context, Result};
    use std::{
        env,
        time::{SystemTime, UNIX_EPOCH},
    };

    /// Covers registration, authentication, PM read state, both username-reference
    /// updates, password updates, and recipient-scoped message deletion.
    ///
    /// This test is deliberately opt-in. It accepts only a dedicated MySQL URL whose
    /// database name ends in _test (or is exactly test).
    #[tokio::test]
    async fn mutating_mysql_flows_use_isolated_disposable_database() -> Result<()> {
        if env::var("TSCHAT_ALLOW_MUTATING_TESTS").as_deref() != Ok("YES") {
            eprintln!(
                "Mutating MySQL integration test skipped; set TSCHAT_ALLOW_MUTATING_TESTS=YES \
                 and TSCHAT_MUTATING_TEST_DATABASE_URL to an isolated *_test database."
            );
            return Ok(());
        }

        let url = env::var("TSCHAT_MUTATING_TEST_DATABASE_URL")
            .context("TSCHAT_MUTATING_TEST_DATABASE_URL is required when mutating tests are enabled")?;
        let parsed = url::Url::parse(&url).context("Invalid mutating-test MySQL URL")?;
        anyhow::ensure!(parsed.scheme() == "mysql", "Mutating-test URL must use mysql://");
        anyhow::ensure!(parsed.host_str().is_some(), "Mutating-test URL must include a host");
        let database_name = urlencoding::decode(parsed.path().trim_start_matches('/'))
            .context("Invalid database name in mutating-test URL")?;
        let normalized_name = database_name.to_ascii_lowercase();
        anyhow::ensure!(
            normalized_name == "test" || normalized_name.ends_with("_test"),
            "Refusing to run mutating integration tests: database name must be exactly 'test' or end in '_test'"
        );

        let db = Database::connect_url(&url).await?;
        db.verify_schema().await?;

        let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let sender = format!("tschat_it_sender_{unique:x}");
        let recipient = format!("tschat_it_recipient_{unique:x}");
        let renamed_sender = format!("{sender}_new");
        let renamed_recipient = format!("{recipient}_new");
        let sender_email = format!("{sender}@example.invalid");
        let recipient_email = format!("{recipient}@example.invalid");
        let renamed_sender_email = format!("{renamed_sender}@example.invalid");
        let renamed_recipient_email = format!("{renamed_recipient}@example.invalid");
        let original_hash = "integration-test-hash-v1";
        let updated_hash = "integration-test-hash-v2";

        let test_result: Result<()> = async {
            db.register(&sender, original_hash, &sender_email, "").await?;
            db.register(&recipient, original_hash, &recipient_email, "").await?;

            let authenticated = db.authenticate(&sender, original_hash).await?
                .context("Registered sender could not authenticate")?;
            anyhow::ensure!(authenticated.username == sender, "Authentication returned the wrong user");

            let (stored_user, stored_hash) = db.user_with_password(&sender).await?
                .context("Registered sender disappeared from tb_users")?;
            anyhow::ensure!(stored_user.email == sender_email, "Registration stored the wrong email");
            anyhow::ensure!(stored_hash == original_hash, "Registration stored the wrong password hash");

            db.send_private_message(
                &recipient,
                &sender,
                "Integration test",
                "Private-message body",
            ).await?;

            let unread = db.inbox(&recipient).await?;
            anyhow::ensure!(unread.len() == 1, "Recipient should see exactly one test message");
            anyhow::ensure!(!unread[0].is_read, "New private message should be unread");
            let message_id = unread[0].id;

            db.mark_message_read(message_id, &recipient).await?;
            let read = db.inbox(&recipient).await?;
            anyhow::ensure!(read.len() == 1 && read[0].is_read, "Read-state update was not persisted");

            db.update_profile_with_messages(
                &sender,
                &renamed_sender,
                &renamed_sender_email,
                "TSchat integration sender",
                "+10000000000",
                Some(updated_hash),
            ).await?;
            let renamed_sender_user = db.user_by_name(&renamed_sender).await?
                .context("Sender profile rename was not persisted")?;
            anyhow::ensure!(
                renamed_sender_user.email == renamed_sender_email,
                "Sender profile email was not updated"
            );
            anyhow::ensure!(
                db.user_by_name(&sender).await?.is_none(),
                "Old sender username still exists after rename"
            );
            let (_, migrated_hash) = db.user_with_password(&renamed_sender).await?
                .context("Renamed sender password could not be read")?;
            anyhow::ensure!(migrated_hash == updated_hash, "Password update was not persisted");

            let sender_renamed_inbox = db.inbox(&recipient).await?;
            anyhow::ensure!(
                sender_renamed_inbox.len() == 1
                    && sender_renamed_inbox[0].from_user == renamed_sender,
                "Sender rename did not update tb_pm.fromuser"
            );

            db.update_profile_with_messages(
                &recipient,
                &renamed_recipient,
                &renamed_recipient_email,
                "TSchat integration recipient",
                "+10000000001",
                None,
            ).await?;
            anyhow::ensure!(
                db.user_by_name(&recipient).await?.is_none(),
                "Old recipient username still exists after rename"
            );

            let moved_inbox = db.inbox(&renamed_recipient).await?;
            anyhow::ensure!(
                moved_inbox.len() == 1
                    && moved_inbox[0].id == message_id
                    && moved_inbox[0].from_user == renamed_sender
                    && moved_inbox[0].is_read,
                "Recipient rename did not preserve the private-message history"
            );

            anyhow::ensure!(
                db.delete_private_message(message_id, &renamed_sender).await? == 0,
                "Sender must not be able to delete a message from the recipient's inbox"
            );
            anyhow::ensure!(
                db.delete_private_message(message_id, &renamed_recipient).await? == 1,
                "Recipient could not delete their selected message"
            );
            anyhow::ensure!(
                db.delete_private_message(message_id, &renamed_recipient).await? == 0,
                "Deleting the same message twice should affect no rows"
            );
            anyhow::ensure!(
                db.inbox(&renamed_recipient).await?.is_empty(),
                "Deleted message is still present in the recipient inbox"
            );

            Ok(())
        }.await;

        // Always remove only rows with this test's unpredictable marker, even if an
        // assertion or an intermediate SQL operation fails.
        let cleanup_result: Result<()> = async {
            sqlx::query(
                "DELETE FROM tb_pm WHERE username IN (?, ?) OR fromuser IN (?, ?)"
            )
            .bind(&recipient)
            .bind(&renamed_recipient)
            .bind(&sender)
            .bind(&renamed_sender)
            .execute(&db.pool)
            .await?;

            sqlx::query("DELETE FROM tb_users WHERE username IN (?, ?)")
                .bind(&sender)
                .bind(&renamed_sender)
                .execute(&db.pool)
                .await?;

            sqlx::query("DELETE FROM tb_users WHERE username IN (?, ?)")
                .bind(&recipient)
                .bind(&renamed_recipient)
                .execute(&db.pool)
                .await?;

            Ok(())
        }.await;

        cleanup_result.context("Failed to clean up uniquely marked MySQL integration-test rows")?;
        test_result?;
        Ok(())
    }
}
