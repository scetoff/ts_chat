#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_base64: String,
    pub mobile: String,
    pub chat_color: String,
    pub chat_font: String,
    pub chat_font_size: i32,
    pub chat_font_style: String,
}

#[derive(Debug, Clone)]
pub struct PrivateMessage {
    pub id: i64,
    pub from_user: String,
    pub subject: String,
    pub message: String,
    pub is_read: bool,
}
