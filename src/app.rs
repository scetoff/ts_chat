use eframe::egui;
use crate::{config::Config, db::Database, models::User};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Screen { Login, Register, Chat }

pub struct TsChatApp {
    config: Config,
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
}

impl Default for TsChatApp {
    fn default() -> Self {
        Self {
            config: Config::load(), db: None, screen: Screen::Login,
            username: String::new(), password: String::new(), email: String::new(),
            current_user: None, friends: Vec::new(), chat_text: String::new(),
            draft: String::new(), selected_friend: None, private_subject: String::new(),
            private_text: String::new(), inbox: Vec::new(),
            status: "Введите параметры MySQL в config.toml и нажмите «Подключиться».".into(),
            dark_theme: true, connected: false, show_inbox: false,
        }
    }
}

impl TsChatApp {
    fn connect(&mut self) {
        let cfg = self.config.clone();
        match tokio::runtime::Builder::new_current_thread().enable_all().build() {
            Ok(rt) => match rt.block_on(Database::connect(&cfg)) {
                Ok(db) => match rt.block_on(db.initialize()) {
                    Ok(()) => { self.db = Some(db); self.connected = true; self.status = "Подключено к MySQL.".into(); }
                    Err(e) => self.status = format!("Ошибка инициализации БД: {e:#}"),
                },
                Err(e) => self.status = format!("{e:#}"),
            },
            Err(e) => self.status = format!("Ошибка запуска async runtime: {e}"),
        }
    }

    fn password_digest(password: &str) -> String {
        // Legacy compatibility only: original VB.NET added ASCII "ZUBI" before and after MD5 bytes.
        let mut input = password.as_bytes().to_vec();
        input.extend_from_slice(b"ZUBI");
        let mut digest = md5::compute(input).0.to_vec();
        digest.extend_from_slice(b"ZUBI");
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, digest)
    }

    fn run_db<T>(&mut self, f: impl FnOnce(&tokio::runtime::Runtime, &Database) -> anyhow::Result<T>) -> anyhow::Result<T> {
        let db = self.db.clone().ok_or_else(|| anyhow::anyhow!("Сначала подключитесь к базе данных"))?;
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        f(&rt, &db)
    }

    fn login(&mut self) {
        if self.username.trim().is_empty() || self.password.is_empty() {
            self.status = "Укажите никнейм и пароль.".into(); return;
        }
        let username = self.username.clone();
        let hash = Self::password_digest(&self.password);
        match self.run_db(|rt, db| rt.block_on(db.authenticate(&username, &hash))) {
            Ok(Some(user)) => {
                let name = user.username.clone();
                let friends = self.run_db(|rt, db| rt.block_on(db.users_except(&name))).unwrap_or_default();
                let chat = self.run_db(|rt, db| rt.block_on(db.read_chat())).unwrap_or_default();
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
        let hash = Self::password_digest(&self.password);
        // The old application shipped a users.png default avatar; empty data preserves DB schema but should be replaced.
        match self.run_db(|rt, db| rt.block_on(db.register(&username, &hash, &email, ""))) {
            Ok(()) => { self.status = "Аккаунт создан. Теперь выполните вход.".into(); self.screen = Screen::Login; }
            Err(e) => self.status = format!("Не удалось создать аккаунт: {e:#}"),
        }
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
                            ui.add(egui::TextEdit::multiline(&mut self.chat_text).desired_rows(23).desired_width(f32::INFINITY).interactive(false));
                            ui.add(egui::TextEdit::singleline(&mut self.draft).hint_text("Сообщение...").desired_width(f32::INFINITY));
                            if ui.button("Отправить").clicked() { self.send_chat(); }
                        });
                        cols[1].group(|ui| {
                            ui.label("Пользователи");
                            egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                                for name in self.friends.clone() {
                                    if ui.selectable_label(self.selected_friend.as_deref() == Some(name.as_str()), &name).clicked() {
                                        self.selected_friend = Some(name);
                                    }
                                }
                            });
                            ui.separator();
                            ui.label("Личное сообщение");
                            ui.add(egui::TextEdit::singleline(&mut self.private_subject).hint_text("Тема"));
                            ui.add(egui::TextEdit::multiline(&mut self.private_text).desired_rows(5).hint_text("Текст сообщения"));
                            if ui.button("Отправить личное").clicked() { self.send_private(); }
                        });
                    });
                    if self.show_inbox {
                        egui::Window::new("Входящие сообщения").open(&mut self.show_inbox).show(ctx, |ui| {
                            for m in &self.inbox {
                                ui.collapsing(format!("{} — {}", m.from_user, m.subject), |ui| { ui.label(&m.message); });
                            }
                        });
                    }
                }
            }
            if !self.status.is_empty() { ui.separator(); ui.label(&self.status); }
        });
    }
}
