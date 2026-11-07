use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool, Row};
use crate::{config::Config, models::{ChatStyle, PrivateMessage, User}};

#[derive(Clone)]
pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn connect(config: &Config) -> Result<Self> {
        Config::validate_database_driver()?;
        let pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect(&config.mysql_url())
            .await
            .context("Не удалось подключиться к MySQL")?;
        Ok(Self { pool })
    }


    /// Verify the legacy tables without changing schema or data.
    pub async fn verify_schema(&self) -> Result<()> {
        sqlx::query(
            "SELECT id, username, password, email, name, dp, mobile, chatcolor, chatfont, chatfontsize, chatfontstyle FROM tb_users LIMIT 0"
        ).fetch_all(&self.pool).await.context("Таблица tb_users отсутствует или её структура несовместима")?;

        sqlx::query("SELECT id, chat FROM tb_chat LIMIT 0")
            .fetch_all(&self.pool).await.context("Таблица tb_chat отсутствует или её структура несовместима")?;

        sqlx::query("SELECT id, username, fromuser, subject, message, isread FROM tb_pm LIMIT 0")
            .fetch_all(&self.pool).await.context("Таблица tb_pm отсутствует или её структура несовместима")?;

        // Ensure Argon2id can be stored before upgrading a legacy password hash.
        let password_capacity: Option<u64> = sqlx::query_scalar(
            "SELECT CHARACTER_MAXIMUM_LENGTH FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'tb_users' AND COLUMN_NAME = 'password'"
        ).fetch_one(&self.pool).await.context("Не удалось проверить длину tb_users.password")?;
        if password_capacity.unwrap_or(0) < 100 {
            anyhow::bail!("tb_users.password слишком короткий для Argon2id (нужно минимум 100 символов); сначала проверьте тестовую БД и выполните миграцию столбца");
        }

        let auto_increment_ids: u64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND ((TABLE_NAME = 'tb_users' AND COLUMN_NAME = 'id') OR (TABLE_NAME = 'tb_pm' AND COLUMN_NAME = 'id')) AND EXTRA LIKE '%auto_increment%'"
        ).fetch_one(&self.pool).await.context("Не удалось проверить автоинкремент ID")?;
        if auto_increment_ids != 2 {
            anyhow::bail!("В tb_users.id и tb_pm.id должен быть включён AUTO_INCREMENT; проверьте DDL тестовой БД");
        }

        let chat_row_count: u64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tb_chat WHERE id = 1"
        ).fetch_one(&self.pool).await.context("Не удалось проверить стартовую запись tb_chat")?;
        if chat_row_count == 0 {
            anyhow::bail!("В tb_chat отсутствует строка id=1; заполните её в тестовой БД после проверки резервной копии");
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

    pub async fn register(&self, username: &str, password_hash: &str, email: &str, avatar_base64: &str) -> Result<()> {
        let username_count: u64 = sqlx::query_scalar("SELECT COUNT(*) FROM tb_users WHERE username = ?")
            .bind(username).fetch_one(&self.pool).await?;
        if username_count > 0 {
            anyhow::bail!("Никнейм уже занят");
        }
        let email_count: u64 = sqlx::query_scalar("SELECT COUNT(*) FROM tb_users WHERE email = ?")
            .bind(email).fetch_one(&self.pool).await?;
        if email_count > 0 {
            anyhow::bail!("Эта почта уже используется");
        }
        sqlx::query(
            "INSERT INTO tb_users (username,password,email,name,dp,mobile,chatcolor,chatfont,chatfontsize,chatfontstyle) VALUES (?,?,?,?,?,?,?,?,?,?)"
        )
        .bind(username).bind(password_hash).bind(email).bind(username)
        .bind(avatar_base64).bind("+7(xxx)xxx-xx-xx").bind("FF000000")
        .bind("Candara").bind(16_i32).bind("Regular")
        .execute(&self.pool).await?;
        Ok(())
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
        let username_count: u64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tb_users WHERE username = ? AND username <> ?"
        ).bind(username).bind(old_username).fetch_one(&mut *tx).await?;
        if username_count > 0 { anyhow::bail!("Никнейм уже занят"); }

        let email_count: u64 = sqlx::query_scalar(
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
