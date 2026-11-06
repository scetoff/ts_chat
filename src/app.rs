use eframe::egui;
use std::{io::Cursor, time::{Duration, Instant}};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use crate::{config::Config, db::Database, models::User};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Screen { Login, Register, Chat }

pub struct TsChatApp {
    config: Config,
    runtime: tokio::runtime::Runtime,
    db: Option<Database>,
    screen: Screen,
    username: String,
    password: String,
    email: String,
    current_user: Option<User>,
    friends: Vec<String>,
    chat_text: String,
    draft: String,
    selected_friend: Option<String>,
    private_subject: String,
    private_text: String,
    inbox: Vec<crate::models::PrivateMessage>,
    status: String,
    dark_theme: bool,
    connected: bool,
    show_inbox: bool,
    last_refresh: Instant,
    profile_username: String,
    profile_name: String,
    profile_email: String,
    profile_mobile: String,
    avatar_texture: Option<egui::TextureHandle>,
    avatar_cache_key: Option<String>,
    friend_avatar_texture: Option<egui::TextureHandle>,
    friend_avatar_cache_key: Option<String>,
    new_password: String,
    repeat_password: String,
    friend_profile: Option<User>,
    show_friend_profile: bool,
}

impl Default for TsChatApp {
    fn default() -> Self {
        Self {
            config: Config::load(),
            runtime: tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("Не удалось создать async runtime"),
            db: None, screen: Screen::Login,
            username: String::new(), password: String::new(), email: String::new(),
            current_user: None, friends: Vec::new(), chat_text: String::new(),
            draft: String::new(), selected_friend: None, private_subject: String::new(),
            private_text: String::new(), inbox: Vec::new(),
            status: "Введите параметры MySQL в config.toml и нажмите «Подключиться».".into(),
            dark_theme: true, connected: false, show_inbox: false,
            last_refresh: Instant::now(),
            profile_username: String::new(), profile_name: String::new(), profile_email: String::new(), profile_mobile: String::new(),
            avatar_texture: None, avatar_cache_key: None,
            friend_avatar_texture: None, friend_avatar_cache_key: None,
            new_password: String::new(), repeat_password: String::new(),
            friend_profile: None, show_friend_profile: false,
        }
    }
}

impl TsChatApp {
    fn connect(&mut self) {
        let cfg = self.config.clone();
        match self.runtime.block_on(Database::connect(&cfg)) {
            Ok(db) => match self.runtime.block_on(db.initialize()) {
                Ok(()) => { self.db = Some(db); self.connected = true; self.status = "Подключено к MySQL.".into(); }
                Err(e) => self.status = format!("Ошибка инициализации БД: {e:#}"),
            },
            Err(e) => self.status = format!("{e:#}"),
        }
    }

    fn legacy_password_digest(password: &str) -> String {
        // Compatibility with old VB.NET accounts only: MD5(password + ASCII "ZUBI") + "ZUBI", then Base64.
        let mut input = password.as_bytes().to_vec();
        input.extend_from_slice(b"ZUBI");
        let mut digest = md5::compute(input).0.to_vec();
        digest.extend_from_slice(b"ZUBI");
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(digest)
    }

    fn hash_password(password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|error| anyhow::anyhow!("Не удалось создать Argon2id-хеш: {error}"))
    }

    fn verify_password(password: &str, stored_hash: &str) -> bool {
        if stored_hash.starts_with("$argon2") {
            let Ok(parsed) = PasswordHash::new(stored_hash) else { return false; };
            Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok()
        } else {
            stored_hash == Self::legacy_password_digest(password)
        }
    }

    fn run_db<T>(&mut self, f: impl FnOnce(&tokio::runtime::Runtime, &Database) -> anyhow::Result<T>) -> anyhow::Result<T> {
        let db = self.db.clone().ok_or_else(|| anyhow::anyhow!("Сначала подключитесь к базе данных"))?;
        f(&self.runtime, &db)
    }

    fn login(&mut self) {
        if self.username.trim().is_empty() || self.password.is_empty() {
            self.status = "Укажите никнейм и пароль.".into(); return;
        }
        let username = self.username.clone();
        let password = self.password.clone();
        match self.run_db(|rt, db| rt.block_on(db.user_with_password(&username))) {
            Ok(Some((user, stored_hash))) => {
                if !Self::verify_password(&password, &stored_hash) {
                    self.status = "Проверьте никнейм и пароль.".into();
                    return;
                }
                if !stored_hash.starts_with("$argon2") {
                    match Self::hash_password(&password) {
                        Ok(new_hash) => {
                            let account_name = user.username.clone();
                            if let Err(error) = self.run_db(|rt, db| rt.block_on(db.update_password(&account_name, &new_hash))) {
                                self.status = format!("Вход выполнен, но не удалось обновить защиту пароля: {error:#}");
                            }
                        }
                        Err(error) => self.status = format!("Вход выполнен, но обновление хеша не удалось: {error:#}"),
                    }
                }
                let name = user.username.clone();
                let friends = self.run_db(|rt, db| rt.block_on(db.users_except(&name))).unwrap_or_default();
                let chat = self.run_db(|rt, db| rt.block_on(db.read_chat())).unwrap_or_default();
                self.profile_username = user.username.clone();
                self.profile_name = user.display_name.clone();
                self.profile_email = user.email.clone();
                self.profile_mobile = user.mobile.clone();
                self.avatar_cache_key = None;
                self.avatar_texture = None;
                self.current_user = Some(user); self.friends = friends;
                self.chat_text = chat; self.screen = Screen::Chat; self.status.clear();
            }
            Ok(None) => self.status = "Проверьте никнейм и пароль.".into(),
            Err(e) => self.status = format!("Ошибка авторизации: {e:#}"),
        }
    }

    fn register(&mut self) {
        if self.username.trim().is_empty() || self.password.is_empty() || self.email.trim().is_empty() {
            self.status = "Заполните никнейм, пароль и почту.".into(); return;
        }
        let username = self.username.clone(); let email = self.email.clone();
        let hash = match Self::hash_password(&self.password) {
            Ok(hash) => hash,
            Err(error) => { self.status = format!("Не удалось защитить пароль: {error:#}"); return; }
        };
        // Empty avatar is allowed by the legacy schema; the user can upload one from profile settings.
        match self.run_db(|rt, db| rt.block_on(db.register(&username, &hash, &email, ""))) {
            Ok(()) => { self.status = "Аккаунт создан. Теперь выполните вход.".into(); self.screen = Screen::Login; }
            Err(e) => self.status = format!("Не удалось создать аккаунт: {e:#}"),
        }
    }


    fn save_profile(&mut self) {
        let Some(user) = self.current_user.as_ref() else { return; };
        let old_username = user.username.clone();
        let new_username = self.profile_username.trim().to_string();
        let email = self.profile_email.trim().to_string();
        let name = self.profile_name.trim().to_string();
        let mobile = self.profile_mobile.trim().to_string();
        if new_username.is_empty() || email.is_empty() || name.is_empty() {
            self.status = "Имя, никнейм и почта не должны быть пустыми.".into();
            return;
        }
        let password = self.new_password.clone();
        let repeat = self.repeat_password.clone();
        if !password.is_empty() && password != repeat {
            self.status = "Новые пароли не совпадают.".into();
            return;
        }
        match self.run_db(|rt, db| rt.block_on(db.update_profile(&old_username, &new_username, &email, &name, &mobile))) {
            Ok(()) => {
                if let Some(user) = self.current_user.as_mut() {
                    user.username = new_username.clone();
                    user.email = email.clone();
                    user.display_name = name.clone();
                    user.mobile = mobile.clone();
                }
                if !password.is_empty() {
                    let hash = match Self::hash_password(&password) {
                        Ok(hash) => hash,
                        Err(error) => { self.status = format!("Профиль сохранён, но пароль захэшировать не удалось: {error:#}"); return; }
                    };
                    match self.run_db(|rt, db| rt.block_on(db.update_password(&new_username, &hash))) {
                        Ok(()) => self.status = "Профиль и пароль сохранены.".into(),
                        Err(e) => self.status = format!("Профиль сохранён, но пароль обновить не удалось: {e:#}"),
                    }
                } else {
                    self.status = "Профиль сохранён.".into();
                }
                self.new_password.clear();
                self.repeat_password.clear();
                if let Ok(friends) = self.run_db(|rt, db| rt.block_on(db.users_except(&new_username))) {
                    self.friends = friends;
                }
            }
            Err(e) => self.status = format!("Не удалось сохранить профиль: {e:#}"),
        }
    }

    fn choose_avatar(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Изображения", &["png", "jpg", "jpeg", "gif"])
            .pick_file() else { return; };
        let result = (|| -> anyhow::Result<String> {
            let bytes = std::fs::read(&path)?;
            let image = image::load_from_memory(&bytes)?;
            let resized = image.thumbnail(640, 480);
            let mut png = Cursor::new(Vec::new());
            resized.write_to(&mut png, image::ImageFormat::Png)?;
            use base64::Engine;
            Ok(base64::engine::general_purpose::STANDARD.encode(png.into_inner()))
        })();
        match result {
            Ok(encoded) => {
                let Some(username) = self.current_user.as_ref().map(|u| u.username.clone()) else { return; };
                match self.run_db(|rt, db| rt.block_on(db.update_avatar(&username, &encoded))) {
                    Ok(()) => {
                        if let Some(user) = self.current_user.as_mut() {
                            user.avatar_base64 = encoded;
                        }
                        self.avatar_cache_key = None;
                        self.status = "Аватар обновлён.".into();
                    }
                    Err(e) => self.status = format!("Не удалось сохранить аватар: {e:#}"),
                }
            }
            Err(e) => self.status = format!("Не удалось прочитать изображение: {e:#}"),
        }
    }

    fn ensure_avatar_texture(&mut self, ctx: &egui::Context) {
        let encoded = self.current_user.as_ref().map(|u| u.avatar_base64.clone()).unwrap_or_default();
        if encoded.is_empty() || self.avatar_cache_key.as_deref() == Some(encoded.as_str()) {
            return;
        }
        use base64::Engine;
        let texture = base64::engine::general_purpose::STANDARD.decode(&encoded)
            .ok()
            .and_then(|bytes| image::load_from_memory(&bytes).ok())
            .map(|img| {
                let rgba = img.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let color = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
                ctx.load_texture("tschat-avatar", color, egui::TextureOptions::LINEAR)
            });
        self.avatar_texture = texture;
        self.avatar_cache_key = Some(encoded);
    }


    fn ensure_friend_avatar_texture(&mut self, ctx: &egui::Context) {
        let encoded = self.friend_profile.as_ref()
            .map(|u| u.avatar_base64.clone()).unwrap_or_default();
        if encoded.is_empty() || self.friend_avatar_cache_key.as_deref() == Some(encoded.as_str()) {
            return;
        }
        use base64::Engine;
        let texture = base64::engine::general_purpose::STANDARD.decode(&encoded)
            .ok()
            .and_then(|bytes| image::load_from_memory(&bytes).ok())
            .map(|img| {
                let rgba = img.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let color = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
                ctx.load_texture("tschat-friend-avatar", color, egui::TextureOptions::LINEAR)
            });
        self.friend_avatar_texture = texture;
        self.friend_avatar_cache_key = Some(encoded);
    }

    fn add_smiley(&mut self, code: &str) {
        if !self.draft.is_empty() { self.draft.push(' '); }
        self.draft.push_str(code);
    }

    fn refresh_chat(&mut self) {
        if let Ok(chat) = self.run_db(|rt, db| rt.block_on(db.read_chat())) { self.chat_text = chat; }
    }

    fn send_chat(&mut self) {
        let Some(user) = self.current_user.as_ref() else { return; };
        if self.draft.trim().is_empty() { return; }
        let next = format!("{}\n\n{}: {}", self.chat_text, user.username, self.draft.trim());
        match self.run_db(|rt, db| rt.block_on(db.write_chat(&next))) {
            Ok(()) => { self.chat_text = next; self.draft.clear(); }
            Err(e) => self.status = format!("Ошибка отправки: {e:#}"),
        }
    }

    fn send_private(&mut self) {
        let Some(user) = self.current_user.as_ref() else { return; };
        let Some(to) = self.selected_friend.clone() else { self.status = "Выберите собеседника.".into(); return; };
        let from = user.username.clone(); let subject = self.private_subject.clone(); let body = self.private_text.clone();
        if subject.trim().is_empty() || body.trim().is_empty() { self.status = "Тема и текст сообщения обязательны.".into(); return; }
        match self.run_db(|rt, db| rt.block_on(db.send_private_message(&to, &from, &subject, &body))) {
            Ok(()) => { self.private_subject.clear(); self.private_text.clear(); self.status = format!("Сообщение отправлено пользователю {to}."); }
            Err(e) => self.status = format!("Ошибка отправки личного сообщения: {e:#}"),
        }
    }

    fn load_selected_profile(&mut self) {
        let Some(name) = self.selected_friend.clone() else {
            self.status = "Сначала выберите пользователя.".into();
            return;
        };
        match self.run_db(|rt, db| rt.block_on(db.user_by_name(&name))) {
            Ok(Some(user)) => {
                self.friend_profile = Some(user);
                self.friend_avatar_texture = None;
                self.friend_avatar_cache_key = None;
                self.show_friend_profile = true;
            }
            Ok(None) => self.status = "Пользователь не найден.".into(),
            Err(e) => self.status = format!("Не удалось загрузить профиль: {e:#}"),
        }
    }

    fn mark_message_read(&mut self, message_id: i64) {
        let Some(username) = self.current_user.as_ref().map(|u| u.username.clone()) else { return; };
        match self.run_db(|rt, db| rt.block_on(db.mark_message_read(message_id, &username))) {
            Ok(()) => {
                if let Some(message) = self.inbox.iter_mut().find(|m| m.id == message_id) {
                    message.is_read = true;
                }
            }
            Err(e) => self.status = format!("Не удалось отметить сообщение прочитанным: {e:#}"),
        }
    }

    fn load_inbox(&mut self) {
        let Some(user) = self.current_user.as_ref() else { return; };
        let name = user.username.clone();
        match self.run_db(|rt, db| rt.block_on(db.inbox(&name))) {
            Ok(items) => { self.inbox = items; self.show_inbox = true; }
            Err(e) => self.status = format!("Ошибка загрузки входящих: {e:#}"),
        }
    }
}

impl eframe::App for TsChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.screen == Screen::Chat {
            ctx.request_repaint_after(Duration::from_secs(2));
            if self.last_refresh.elapsed() >= Duration::from_secs(2) {
                self.refresh_chat();
                self.last_refresh = Instant::now();
            }
        }

        self.ensure_avatar_texture(ctx);
        if self.show_friend_profile { self.ensure_friend_avatar_texture(ctx); }
        if self.dark_theme { ctx.set_visuals(egui::Visuals::dark()); } else { ctx.set_visuals(egui::Visuals::light()); }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("TSchat");
                ui.separator();
                if ui.button(if self.dark_theme { "☀ Светлая тема" } else { "☾ Тёмная тема" }).clicked() { self.dark_theme = !self.dark_theme; }
                ui.separator();
                ui.label(if self.connected { "● MySQL подключена" } else { "○ Нет подключения" });
                if ui.button("Подключиться").clicked() && !self.connected { self.connect(); }
                if self.screen == Screen::Chat && ui.button("Обновить чат").clicked() { self.refresh_chat(); }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.screen {
                Screen::Login | Screen::Register => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(55.0);
                        ui.heading(if self.screen == Screen::Login { "Авторизация" } else { "Создание аккаунта" });
                        ui.add_space(16.0);
                        ui.label("Никнейм");
                        ui.add(egui::TextEdit::singleline(&mut self.username).desired_width(280.0));
                        ui.label("Пароль");
                        ui.add(egui::TextEdit::singleline(&mut self.password).password(true).desired_width(280.0));
                        if self.screen == Screen::Register {
                            ui.label("Почта");
                            ui.add(egui::TextEdit::singleline(&mut self.email).desired_width(280.0));
                        }
                        ui.add_space(10.0);
                        if self.screen == Screen::Login {
                            if ui.button("Войти").clicked() { self.login(); }
                            if ui.button("Создать аккаунт").clicked() { self.screen = Screen::Register; self.status.clear(); }
                        } else {
                            if ui.button("Зарегистрироваться").clicked() { self.register(); }
                            if ui.button("Назад к авторизации").clicked() { self.screen = Screen::Login; self.status.clear(); }
                        }
                    });
                }
                Screen::Chat => {
                    ui.horizontal(|ui| {
                        ui.heading(format!("Чат — {}", self.current_user.as_ref().map(|u| u.username.as_str()).unwrap_or("")));
                        if ui.button("Входящие").clicked() { self.load_inbox(); }
                        if ui.button("Выйти").clicked() { self.current_user = None; self.screen = Screen::Login; }
                    });
                    ui.columns(2, |cols| {
                        cols[0].group(|ui| {
                            ui.label("Общий чат");
                            let mut readable_chat = self.chat_text.clone();
                            for (code, glyph) in [
                                (":fuu:", "😠"), (":beer:", "🍺"), (":sorry:", "🙏"), (":read:", "📖"),
                                (":girl:", "👧"), (":lich:", "🧙"), (":sing:", "🎤"), (":haha:", "😂"),
                                (":allbear:", "🐻"), (":say:", "💬"), (":money:", "💰"), (":no:", "⛔"),
                                (":king:", "👑"), (":go:", "🚀"), (":good:", "👍"), (":nice:", "😎"),
                            ] {
                                readable_chat = readable_chat.replace(code, glyph);
                            }
                            ui.add(egui::TextEdit::multiline(&mut readable_chat).desired_rows(23).desired_width(f32::INFINITY).interactive(false));
                            ui.add(egui::TextEdit::singleline(&mut self.draft).hint_text("Сообщение...").desired_width(f32::INFINITY));
                            if ui.button("Отправить").clicked() { self.send_chat(); }
                            ui.collapsing("Смайлики", |ui| {
                                let smileys = [":fuu:", ":beer:", ":sorry:", ":read:", ":girl:", ":lich:", ":sing:", ":haha:",
                                    ":allbear:", ":say:", ":money:", ":no:", ":king:", ":go:", ":good:", ":nice:"];
                                egui::Grid::new("smiley-grid").num_columns(4).show(ui, |ui| {
                                    for (index, code) in smileys.iter().enumerate() {
                                        if ui.button(*code).clicked() { self.add_smiley(code); }
                                        if (index + 1) % 4 == 0 { ui.end_row(); }
                                    }
                                });
                            });
                        });
                        cols[1].group(|ui| {
                            ui.horizontal(|ui| {
                                if let Some(texture) = &self.avatar_texture {
                                    ui.add(egui::Image::new((texture.id(), egui::vec2(88.0, 88.0))));
                                } else {
                                    ui.label("Аватар не задан");
                                }
                                if ui.button("Изменить аватар").clicked() { self.choose_avatar(); }
                            });
                            ui.collapsing("Редактировать профиль", |ui| {
                                ui.label("Имя");
                                ui.text_edit_singleline(&mut self.profile_name);
                                ui.label("Никнейм");
                                ui.text_edit_singleline(&mut self.profile_username);
                                ui.label("Почта");
                                ui.text_edit_singleline(&mut self.profile_email);
                                ui.label("Телефон");
                                ui.text_edit_singleline(&mut self.profile_mobile);
                                ui.label("Новый пароль (необязательно)");
                                ui.add(egui::TextEdit::singleline(&mut self.new_password).password(true));
                                ui.label("Повторите пароль");
                                ui.add(egui::TextEdit::singleline(&mut self.repeat_password).password(true));
                                if ui.button("Сохранить профиль").clicked() { self.save_profile(); }
                            });
                            ui.separator();
                            ui.label("Пользователи");
                            egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                                for name in self.friends.clone() {
                                    if ui.selectable_label(self.selected_friend.as_deref() == Some(name.as_str()), &name).clicked() {
                                        self.selected_friend = Some(name);
                                    }
                                }
                            });
                            if ui.button("Показать профиль выбранного пользователя").clicked() {
                                self.load_selected_profile();
                            }
                            ui.separator();
                            ui.label("Личное сообщение");
                            ui.add(egui::TextEdit::singleline(&mut self.private_subject).hint_text("Тема"));
                            ui.add(egui::TextEdit::multiline(&mut self.private_text).desired_rows(5).hint_text("Текст сообщения"));
                            if ui.button("Отправить личное").clicked() { self.send_private(); }
                        });
                    });
                    if self.show_friend_profile {
                        let mut close_profile = false;
                        egui::Window::new("Профиль пользователя").show(ctx, |ui| {
                            if let Some(texture) = &self.friend_avatar_texture {
                                ui.add(egui::Image::new((texture.id(), egui::vec2(120.0, 120.0))));
                            }
                            if let Some(profile) = &self.friend_profile {
                                ui.heading(&profile.display_name);
                                ui.label(format!("Никнейм: {}", profile.username));
                                ui.label(format!("Почта: {}", profile.email));
                                ui.label(format!("Телефон: {}", profile.mobile));
                            }
                            if ui.button("Закрыть").clicked() { close_profile = true; }
                        });
                        if close_profile { self.show_friend_profile = false; }
                    }
                    if self.show_inbox {
                        let inbox_items = self.inbox.clone();
                        let mut close_inbox = false;
                        egui::Window::new("Входящие сообщения").show(ctx, |ui| {
                            if ui.button("Закрыть").clicked() { close_inbox = true; }
                            for m in inbox_items {
                                ui.collapsing(
                                    format!("{} — {}{}", m.from_user, m.subject, if m.is_read { "" } else { " • новое" }),
                                    |ui| {
                                        ui.label(&m.message);
                                        if !m.is_read && ui.button("Отметить прочитанным").clicked() {
                                            self.mark_message_read(m.id);
                                        }
                                    },
                                );
                            }
                        });
                        if close_inbox { self.show_inbox = false; }
                    }
                }
            }
            if !self.status.is_empty() { ui.separator(); ui.label(&self.status); }
        });
    }
}
