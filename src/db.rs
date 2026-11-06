use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool, Row};
use crate::{config::Config, models::{PrivateMessage, User}};

#[derive(Clone)]
pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn connect(config: &Config) -> Result<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect(&config.mysql_url())
            .await
            .context("Не удалось подключиться к MySQL")?;
        Ok(Self { pool })
    }

    /// Creates tables compatible with the original TSchat data layout.
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

    pub async fn inbox(&self, username: &str) -> Result<Vec<PrivateMessage>> {
        let rows = sqlx::query("SELECT id,fromuser,subject,message,isread FROM tb_pm WHERE username = ? ORDER BY id DESC")
            .bind(username).fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| PrivateMessage {
            id: r.get("id"), from_user: r.get("fromuser"), subject: r.get("subject"),
            message: r.get("message"), is_read: r.get::<i32,_>("isread") != 0
        }).collect())
    }

    pub async fn update_profile(
        &self,
        old_username: &str,
        username: &str,
        email: &str,
        display_name: &str,
        mobile: &str,
    ) -> Result<()> {
        sqlx::query("UPDATE tb_users SET username = ?, email = ?, name = ?, mobile = ? WHERE username = ?")
            .bind(username).bind(email).bind(display_name).bind(mobile).bind(old_username)
            .execute(&self.pool).await?;
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
