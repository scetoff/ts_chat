use eframe::egui;
use std::{io::Cursor, time::{Duration, Instant}};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use crate::{commands::{parse_command, ChatCommand}, config::{Config, RadioStation}, db::Database, models::User, radio::RadioPlayback};

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
    notes_open: bool,
    note_strokes: Vec<DrawStroke>,
    note_current_stroke: Option<DrawStroke>,
    note_brush_color: egui::Color32,
    note_brush_width: f32,
    note_background: egui::Color32,
    radio_open: bool,
    radio_stations: Vec<RadioStation>,
    selected_radio_station: usize,
    radio_volume: f32,
    radio_status: String,
    radio_player: Option<RadioPlayback>,
    radio_pending: Option<(std::sync::mpsc::Receiver<Result<RadioPlayback, String>>, String)>,
    smiley_textures: Vec<(String, egui::TextureHandle)>,
    command_input: String,
    command_status: String,
    radio_autoplay_pending: Option<usize>,
}


#[derive(Clone)]
struct DrawStroke {
    points: Vec<egui::Vec2>,
    color: egui::Color32,
    width: f32,
}

fn paint_stroke(painter: &egui::Painter, rect: egui::Rect, stroke: &DrawStroke) {
    let to_screen = |p: egui::Vec2| egui::pos2(rect.left() + p.x, rect.top() + p.y);
    match stroke.points.as_slice() {
        [] => {}
        [point] => { painter.circle_filled(to_screen(*point), stroke.width * 0.5, stroke.color); }
        points => {
            for pair in points.windows(2) {
                painter.line_segment(
                    [to_screen(pair[0]), to_screen(pair[1])],
                    egui::Stroke::new(stroke.width, stroke.color),
                );
            }
        }
    }
}

fn stamp_brush(image: &mut image::RgbaImage, cx: i32, cy: i32, radius: i32, color: image::Rgba<u8>) {
    let r2 = radius * radius;
    for y in (cy - radius)..=(cy + radius) {
        for x in (cx - radius)..=(cx + radius) {
            if (x - cx) * (x - cx) + (y - cy) * (y - cy) <= r2
                && x >= 0 && y >= 0
                && (x as u32) < image.width() && (y as u32) < image.height()
            {
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

fn render_strokes_png(
    strokes: &[DrawStroke],
    background: egui::Color32,
    width: u32,
    height: u32,
) -> image::RgbaImage {
    let [br, bg, bb, ba] = background.to_array();
    let mut output = image::RgbaImage::from_pixel(width, height, image::Rgba([br, bg, bb, ba]));
    for stroke in strokes {
        let [r, g, b, a] = stroke.color.to_array();
        let color = image::Rgba([r, g, b, a]);
        let radius = (stroke.width * 0.5).ceil().max(1.0) as i32;
        match stroke.points.as_slice() {
            [] => {}
            [point] => stamp_brush(&mut output, point.x.round() as i32, point.y.round() as i32, radius, color),
            points => {
                for pair in points.windows(2) {
                    let start = pair[0];
                    let end = pair[1];
                    let dx = end.x - start.x;
                    let dy = end.y - start.y;
                    let steps = dx.abs().max(dy.abs()).ceil().max(1.0) as u32;
                    for step in 0..=steps {
                        let t = step as f32 / steps as f32;
                        let x = (start.x + dx * t).round() as i32;
                        let y = (start.y + dy * t).round() as i32;
                        stamp_brush(&mut output, x, y, radius, color);
                    }
                }
            }
        }
    }
    output
}


const SMILEY_CODES: [&str; 16] = [
    ":fuu:", ":beer:", ":sorry:", ":read:", ":girl:", ":lich:", ":sing:", ":haha:",
    ":allbear:", ":say:", ":money:", ":no:", ":king:", ":go:", ":good:", ":nice:",
];

fn show_chat_line(ui: &mut egui::Ui, line: &str, smileys: &[(String, egui::TextureHandle)]) {
    ui.horizontal_wrapped(|ui| {
        let mut rest = line;
        loop {
            let mut next: Option<(usize, &str)> = None;
            for code in SMILEY_CODES {
                if let Some(position) = rest.find(code) {
                    if next.map(|(current, _)| position < current).unwrap_or(true) {
                        next = Some((position, code));
                    }
                }
            }

            match next {
                Some((position, code)) => {
                    if position > 0 {
                        ui.label(&rest[..position]);
                    }
                    if let Some((_, texture)) = smileys.iter().find(|(candidate, _)| candidate == code) {
                        ui.add(egui::Image::new((texture.id(), egui::vec2(28.0, 28.0))));
                    } else {
                        ui.label(code);
                    }
                    rest = &rest[position + code.len()..];
                }
                None => {
                    if !rest.is_empty() { ui.label(rest); }
                    break;
                }
            }
        }
    });
}

impl Default for TsChatApp {
    fn default() -> Self {
        let config = Config::load();
        let radio_stations = config.radio.stations.clone();
        Self {
            config,
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
            notes_open: false, note_strokes: Vec::new(), note_current_stroke: None,
            note_brush_color: egui::Color32::BLACK, note_brush_width: 6.0,
            note_background: egui::Color32::WHITE,
            radio_open: false,
            radio_stations,
            selected_radio_station: 0,
            radio_volume: 0.75,
            radio_status: "Выберите станцию и укажите её URL в настройках.".into(),
            radio_player: None,
            radio_pending: None,
            smiley_textures: Vec::new(),
            command_input: String::new(),
            command_status: String::new(),
            radio_autoplay_pending: None,
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


    fn ensure_smiley_textures(&mut self, ctx: &egui::Context) {
        if self.smiley_textures.len() == SMILEY_CODES.len() { return; }
        let assets: [(&str, &[u8]); 16] = [
            (":fuu:", include_bytes!("../assets/smileys/1.png")),
            (":beer:", include_bytes!("../assets/smileys/2.png")),
            (":sorry:", include_bytes!("../assets/smileys/3.png")),
            (":read:", include_bytes!("../assets/smileys/4.png")),
            (":girl:", include_bytes!("../assets/smileys/5.png")),
            (":lich:", include_bytes!("../assets/smileys/6.png")),
            (":sing:", include_bytes!("../assets/smileys/7.png")),
            (":haha:", include_bytes!("../assets/smileys/8.png")),
            (":allbear:", include_bytes!("../assets/smileys/9.png")),
            (":say:", include_bytes!("../assets/smileys/10.png")),
            (":money:", include_bytes!("../assets/smileys/11.png")),
            (":no:", include_bytes!("../assets/smileys/12.png")),
            (":king:", include_bytes!("../assets/smileys/13.png")),
            (":go:", include_bytes!("../assets/smileys/14.png")),
            (":good:", include_bytes!("../assets/smileys/15.png")),
            (":nice:", include_bytes!("../assets/smileys/16.png")),
        ];

        let mut textures = Vec::with_capacity(assets.len());
        for (code, bytes) in assets {
            let Ok(decoded) = image::load_from_memory(bytes) else { continue; };
            let rgba = decoded.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let color = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
            let name = format!("tschat-smiley-{code}");
            let texture = ctx.load_texture(name, color, egui::TextureOptions::LINEAR);
            textures.push((code.to_string(), texture));
        }
        self.smiley_textures = textures;
    }

    fn add_smiley(&mut self, code: &str) {
        if !self.draft.is_empty() { self.draft.push(' '); }
        self.draft.push_str(code);
    }


    fn save_notes_png(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .set_file_name("tschat-notes.png")
            .add_filter("PNG image", &["png"])
            .save_file() else { return; };
        let image = render_strokes_png(&self.note_strokes, self.note_background, 760, 460);
        match image.save(&path) {
            Ok(()) => self.status = format!("Рисунок сохранён: {}", path.display()),
            Err(error) => self.status = format!("Не удалось сохранить PNG: {error}"),
        }
    }

    fn show_notes_window(&mut self, ctx: &egui::Context) {
        if !self.notes_open { return; }
        let mut clear_requested = false;
        let mut save_requested = false;
        let notes_open = &mut self.notes_open;
        let strokes = &mut self.note_strokes;
        let current = &mut self.note_current_stroke;
        let brush_color = &mut self.note_brush_color;
        let brush_width = &mut self.note_brush_width;
        let background = &mut self.note_background;

        egui::Window::new("Заметки — Paint")
            .open(notes_open)
            .default_size([820.0, 560.0])
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Цвет кисти");
                    ui.color_edit_button_srgba(brush_color);
                    ui.add(egui::Slider::new(brush_width, 1.0..=36.0).text("Размер"));
                    if ui.button("Очистить").clicked() { clear_requested = true; }
                    if ui.button("Сохранить PNG").clicked() { save_requested = true; }
                });
                ui.horizontal(|ui| {
                    ui.label("Фон");
                    if ui.selectable_label(*background == egui::Color32::WHITE, "Белый").clicked() {
                        *background = egui::Color32::WHITE;
                    }
                    if ui.selectable_label(*background == egui::Color32::BLACK, "Чёрный").clicked() {
                        *background = egui::Color32::BLACK;
                    }
                });

                let canvas_size = egui::vec2(760.0, 460.0);
                let (canvas_rect, response) = ui.allocate_exact_size(canvas_size, egui::Sense::drag());
                let painter = ui.painter_at(canvas_rect);
                painter.rect_filled(canvas_rect, 2.0, *background);

                if response.drag_started() {
                    if let Some(pointer) = response.interact_pointer_pos() {
                        *current = Some(DrawStroke {
                            points: vec![pointer - canvas_rect.min],
                            color: *brush_color,
                            width: *brush_width,
                        });
                    }
                }
                if response.dragged() {
                    if let Some(pointer) = response.interact_pointer_pos() {
                        let point = pointer - canvas_rect.min;
                        if current.is_none() {
                            *current = Some(DrawStroke {
                                points: vec![point],
                                color: *brush_color,
                                width: *brush_width,
                            });
                        } else if let Some(stroke) = current.as_mut() {
                            let should_add = stroke.points.last()
                                .map(|last| (*last - point).length() >= 1.0)
                                .unwrap_or(true);
                            if should_add { stroke.points.push(point); }
                        }
                    }
                }
                if response.drag_stopped() {
                    if let Some(stroke) = current.take() {
                        strokes.push(stroke);
                    }
                }

                for stroke in strokes.iter() { paint_stroke(&painter, canvas_rect, stroke); }
                if let Some(stroke) = current.as_ref() { paint_stroke(&painter, canvas_rect, stroke); }
                ui.label("Зажми левую кнопку мыши и рисуй. Рисунок сохраняется в PNG.");
            });

        if clear_requested {
            self.note_strokes.clear();
            self.note_current_stroke = None;
        }
        if save_requested {
            if let Some(stroke) = self.note_current_stroke.take() {
                self.note_strokes.push(stroke);
            }
            self.save_notes_png();
        }
    }



    fn execute_command(&mut self, input: &str) {
        match parse_command(input) {
            ChatCommand::OpenRadio => {
                self.radio_open = true;
                self.command_status = "Открыто радио.".into();
            }
            ChatCommand::CloseRadio => {
                self.radio_open = false;
                self.command_status = "Окно радио закрыто; воспроизведение не остановлено.".into();
            }
            ChatCommand::PlayRadio => {
                self.radio_open = true;
                if let Some(player) = self.radio_player.as_ref() {
                    if player.is_paused() {
                        player.toggle_pause();
                        self.radio_status = "Воспроизведение продолжено.".into();
                    }
                    self.command_status = "Команда воспроизведения выполнена.".into();
                } else {
                    self.command_status = "Выберите станцию в окне радио.".into();
                }
            }
            ChatCommand::PauseRadio => {
                if let Some(player) = self.radio_player.as_ref() {
                    if !player.is_paused() { player.toggle_pause(); }
                    self.command_status = "Радио поставлено на паузу.".into();
                } else {
                    self.command_status = "Радио сейчас не воспроизводится.".into();
                }
            }
            ChatCommand::StopRadio => {
                self.radio_pending = None;
                self.radio_autoplay_pending = None;
                if let Some(player) = self.radio_player.take() { player.stop(); }
                self.radio_status = "Радио остановлено.".into();
                self.command_status = "Радио остановлено.".into();
            }
            ChatCommand::OpenNotes => {
                self.notes_open = true;
                self.command_status = "Открыты заметки.".into();
            }
            ChatCommand::CloseNotes => {
                self.notes_open = false;
                self.command_status = "Окно заметок закрыто.".into();
            }
            ChatCommand::SelectStation(index) => {
                self.selected_radio_station = index;
                self.radio_open = true;
                self.radio_autoplay_pending = Some(index);
                self.command_status = format!("Выбрана станция: {}.", self.radio_stations.get(index).map(|s| s.name.as_str()).unwrap_or("неизвестно"));
            }
            ChatCommand::Unknown => {
                self.command_status = "Команда не распознана или не поддерживается.".into();
            }
        }
    }

    fn save_radio_settings(&mut self) {
        self.config.radio.stations = self.radio_stations.clone();
        match self.config.save() {
            Ok(()) => self.radio_status = "Список станций сохранён в config.toml.".into(),
            Err(error) => self.radio_status = format!("Не удалось сохранить config.toml: {error:#}"),
        }
    }

    fn show_radio_window(&mut self, ctx: &egui::Context) {
        let pending_result = self.radio_pending.as_ref()
            .map(|(receiver, station_name)| (receiver.try_recv(), station_name.clone()));
        if let Some((result, station_name)) = pending_result {
            match result {
                Ok(Ok(player)) => {
                    self.radio_player = Some(player);
                    self.radio_pending = None;
                    self.radio_status = format!("Сейчас играет: {station_name}");
                }
                Ok(Err(error)) => {
                    self.radio_pending = None;
                    self.radio_status = format!("Не удалось запустить радиостанцию: {error}");
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    ctx.request_repaint_after(Duration::from_millis(250));
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.radio_pending = None;
                    self.radio_status = "Подключение к радиостанции прервано.".into();
                }
            }
        }

        if !self.radio_open { return; }

        let mut start_request: Option<(String, String)> = self.radio_autoplay_pending
            .take()
            .and_then(|index| self.radio_stations.get(index).map(|station| (station.url.trim().to_string(), station.name.clone())));
        let mut stop_requested = false;
        let mut pause_requested = false;
        let mut save_requested = false;
        let paused = self.radio_player.as_ref().map(RadioPlayback::is_paused).unwrap_or(true);
        let player_present = self.radio_player.is_some();
        let displayed_radio_status = self.radio_status.clone();
        let connecting = self.radio_pending.is_some();
        let radio_open = &mut self.radio_open;
        let stations = &mut self.radio_stations;
        let selected = &mut self.selected_radio_station;
        let volume = &mut self.radio_volume;

        egui::Window::new("Радио онлайн")
            .open(radio_open)
            .default_size([520.0, 520.0])
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Список станций из оригинальной версии TSchat");
                ui.label("URL потоков не входят в репозиторий — добавьте свои адреса ниже.");
                ui.separator();

                egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                    for (index, station) in stations.iter().enumerate() {
                        if ui.selectable_label(*selected == index, &station.name).clicked() {
                            *selected = index;
                        }
                    }
                });

                ui.separator();
                if let Some(station) = stations.get_mut(*selected) {
                    ui.label(format!("Адрес потока: {}", station.name));
                    ui.add(
                        egui::TextEdit::singleline(&mut station.url)
                            .hint_text("https://example.org/live.mp3")
                            .desired_width(f32::INFINITY),
                    );
                    if ui.button("▶ Воспроизвести выбранную станцию").clicked() {
                        start_request = Some((station.url.trim().to_string(), station.name.clone()));
                    }
                }

                ui.horizontal(|ui| {
                    ui.label("Громкость");
                    ui.add(egui::Slider::new(volume, 0.0..=1.0).show_value(true));
                });
                ui.horizontal(|ui| {
                    if ui.button(if paused { "▶ Продолжить" } else { "Ⅱ Пауза" }).clicked() {
                        pause_requested = true;
                    }
                    if ui.button("■ Остановить").clicked() { stop_requested = true; }
                    if ui.button("Сохранить URL станций").clicked() { save_requested = true; }
                });
                ui.separator();
                ui.label(if player_present { "Плеер запущен." } else if connecting { "Идёт подключение..." } else { "Плеер остановлен." });
                ui.label(&displayed_radio_status);
            });

        if save_requested {
            self.save_radio_settings();
        }
        if let Some(player) = self.radio_player.as_ref() {
            player.set_volume(self.radio_volume);
        }
        if pause_requested {
            if let Some(player) = self.radio_player.as_ref() {
                player.toggle_pause();
                self.radio_status = if player.is_paused() { "Воспроизведение приостановлено.".into() } else { "Воспроизведение продолжено.".into() };
            }
        }
        if stop_requested {
            self.radio_pending = None;
            if let Some(player) = self.radio_player.take() {
                player.stop();
            }
            self.radio_status = "Радио остановлено.".into();
        }
        if let Some((url, station_name)) = start_request {
            if url.is_empty() {
                self.radio_status = "У этой станции нет URL. Заполните адрес потока и сохраните настройки.".into();
            } else {
                self.radio_player = None;
                self.radio_pending = None;
                self.radio_status = format!("Подключение к станции «{station_name}»...");
                let (sender, receiver) = std::sync::mpsc::channel();
                let volume = self.radio_volume;
                let url_for_task = url.clone();
                self.runtime.spawn(async move {
                    let result = RadioPlayback::connect_and_play(&url_for_task, volume)
                        .await
                        .map_err(|error| format!("{error:#}"));
                    let _ = sender.send(result);
                });
                self.radio_pending = Some((receiver, station_name));
                ctx.request_repaint_after(Duration::from_millis(250));
            }
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
        self.ensure_smiley_textures(ctx);
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
                        if ui.button("Заметки / Paint").clicked() { self.notes_open = true; }
                        if ui.button("Радио").clicked() { self.radio_open = true; }
                        if ui.button("Выйти").clicked() { self.current_user = None; self.screen = Screen::Login; }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Команда");
                        ui.add(egui::TextEdit::singleline(&mut self.command_input)
                            .hint_text("Например: Включи Европу")
                            .desired_width(240.0));
                        if ui.button("Выполнить").clicked() {
                            let input = self.command_input.clone();
                            self.execute_command(&input);
                            self.command_input.clear();
                        }
                        ui.label(&self.command_status);
                    });
                    ui.columns(2, |cols| {
                        cols[0].group(|ui| {
                            ui.label("Общий чат");
                            let smiley_items = self.smiley_textures.clone();
                            egui::ScrollArea::vertical().max_height(470.0).show(ui, |ui| {
                                for line in self.chat_text.split('\n') {
                                    show_chat_line(ui, line, &smiley_items);
                                }
                            });
                            ui.add(egui::TextEdit::singleline(&mut self.draft).hint_text("Сообщение...").desired_width(f32::INFINITY));
                            if ui.button("Отправить").clicked() { self.send_chat(); }
                            ui.collapsing("Смайлики", |ui| {
                                egui::Grid::new("smiley-grid").num_columns(4).show(ui, |ui| {
                                    for (index, (code, texture)) in smiley_items.iter().enumerate() {
                                        let image = egui::Image::new((texture.id(), egui::vec2(48.0, 48.0)))
                                            .sense(egui::Sense::click());
                                        if ui.add(image).on_hover_text(code).clicked() {
                                            self.add_smiley(code);
                                        }
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
        self.show_notes_window(ctx);
        self.show_radio_window(ctx);
    }
}
