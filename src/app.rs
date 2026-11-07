use eframe::egui;
use std::{io::Cursor, time::{Duration, Instant}};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use crate::{commands::{parse_command, ChatCommand}, config::{Config, RadioStation}, db::Database, models::{ChatStyle, User}, radio::RadioPlayback};
#[cfg(feature = "voice")]
use crate::voice::{self, MicrophoneRecorder, WhisperModel};

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
    private_message_open: bool,
    confirm_delete_message: Option<i64>,
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
    show_clear_chat_confirmation: bool,
    radio_autoplay_pending: Option<usize>,
    chat_styles: std::collections::HashMap<String, ChatStyle>,
    chat_style_color: egui::Color32,
    chat_style_font: String,
    chat_style_size: i32,
    chat_style_effect: String,
    #[cfg(feature = "voice")]
    voice_open: bool,
    #[cfg(feature = "voice")]
    voice_model_path: String,
    #[cfg(feature = "voice")]
    voice_model: Option<WhisperModel>,
    #[cfg(feature = "voice")]
    voice_model_pending: Option<std::sync::mpsc::Receiver<Result<WhisperModel, String>>>,
    #[cfg(feature = "voice")]
    voice_recorder: Option<MicrophoneRecorder>,
    #[cfg(feature = "voice")]
    voice_recording_started: Option<Instant>,
    #[cfg(feature = "voice")]
    voice_transcription_pending: Option<std::sync::mpsc::Receiver<Result<String, String>>>,
    #[cfg(feature = "voice")]
    voice_status: String,
    show_smileys: bool,
    edit_profile_open: bool,
    chat_style_open: bool,
    last_viewport_screen: Screen,
    ui_textures: std::collections::HashMap<String, egui::TextureHandle>,
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



fn parse_legacy_argb(value: &str) -> Option<egui::Color32> {
    let hex = value.trim().trim_start_matches('#');
    let parsed = u32::from_str_radix(hex, 16).ok()?;
    match hex.len() {
        8 => Some(egui::Color32::from_rgba_unmultiplied(
            ((parsed >> 16) & 0xff) as u8,
            ((parsed >> 8) & 0xff) as u8,
            (parsed & 0xff) as u8,
            ((parsed >> 24) & 0xff) as u8,
        )),
        6 => Some(egui::Color32::from_rgb(
            ((parsed >> 16) & 0xff) as u8,
            ((parsed >> 8) & 0xff) as u8,
            (parsed & 0xff) as u8,
        )),
        _ => None,
    }
}

fn legacy_color_hex(color: egui::Color32) -> String {
    let [r, g, b, a] = color.to_array();
    format!("{a:02X}{r:02X}{g:02X}{b:02X}")
}

fn styled_rich_text<'a>(text: &'a str, style: Option<&ChatStyle>) -> egui::RichText {
    let Some(style) = style else { return egui::RichText::new(text); };
    let mut rich = egui::RichText::new(text)
        .size(style.font_size.clamp(8, 48) as f32);
    if let Some(color) = parse_legacy_argb(&style.color) { rich = rich.color(color); }
    match style.font_style.to_lowercase().as_str() {
        "bold" => { rich = rich.strong(); }
        "italic" => { rich = rich.italics(); }
        "underline" => { rich = rich.underline(); }
        "strikeout" => { rich = rich.strikethrough(); }
        _ => {}
    }
    if matches!(style.font.to_lowercase().as_str(), "consolas" | "courier new" | "monospace") {
        rich = rich.family(egui::FontFamily::Monospace);
    }
    rich
}

const SMILEY_CODES: [&str; 16] = [
    ":fuu:", ":beer:", ":sorry:", ":read:", ":girl:", ":lich:", ":sing:", ":haha:",
    ":allbear:", ":say:", ":money:", ":no:", ":king:", ":go:", ":good:", ":nice:",
];

fn show_chat_line(
    ui: &mut egui::Ui,
    line: &str,
    smileys: &[(String, egui::TextureHandle)],
    styles: &std::collections::HashMap<String, ChatStyle>,
) {
    ui.horizontal_wrapped(|ui| {
        let author_style = line.split_once(": ")
            .and_then(|(author, _)| styles.get(author));
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
                        ui.label(styled_rich_text(&rest[..position], author_style));
                    }
                    if let Some((_, texture)) = smileys.iter().find(|(candidate, _)| candidate == code) {
                        ui.add(egui::Image::new((texture.id(), egui::vec2(28.0, 28.0))));
                    } else {
                        ui.label(code);
                    }
                    rest = &rest[position + code.len()..];
                }
                None => {
                    if !rest.is_empty() { ui.label(styled_rich_text(rest, author_style)); }
                    break;
                }
            }
        }
    });
}



fn legacy_rect(origin: egui::Pos2, x: f32, y: f32, width: f32, height: f32) -> egui::Rect {
    egui::Rect::from_min_size(origin + egui::vec2(x, y), egui::vec2(width, height))
}

fn legacy_text(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    text: impl Into<String>,
    color: egui::Color32,
    size: f32,
) -> egui::Response {
    ui.put(
        rect,
        egui::Label::new(egui::RichText::new(text).color(color).size(size)),
    )
}

fn legacy_button(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    text: &str,
    fill: egui::Color32,
    foreground: egui::Color32,
    size: f32,
) -> egui::Response {
    ui.put(
        rect,
        egui::Button::new(egui::RichText::new(text).color(foreground).size(size))
            .fill(fill)
            .stroke(egui::Stroke::new(1.0, foreground)),
    )
}

fn legacy_text_edit(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    value: &mut String,
    hint: &str,
    password: bool,
    foreground: egui::Color32,
) -> egui::Response {
    let mut edit = egui::TextEdit::singleline(value)
        .desired_width(rect.width())
        .hint_text(hint)
        .text_color(foreground)
        .margin(egui::vec2(6.0, 3.0));
    if password {
        edit = edit.password(true);
    }
    ui.put(rect, edit)
}

fn legacy_group(
    ui: &egui::Ui,
    rect: egui::Rect,
    fill: egui::Color32,
    border: egui::Color32,
) {
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, fill);
    let stroke = egui::Stroke::new(1.0, border);
    painter.line_segment([rect.left_top(), rect.right_top()], stroke);
    painter.line_segment([rect.left_bottom(), rect.right_bottom()], stroke);
    painter.line_segment([rect.left_top(), rect.left_bottom()], stroke);
    painter.line_segment([rect.right_top(), rect.right_bottom()], stroke);
}

fn legacy_static_image(
    ui: &egui::Ui,
    textures: &std::collections::HashMap<String, egui::TextureHandle>,
    name: &str,
    rect: egui::Rect,
) {
    if let Some(texture) = textures.get(name) {
        ui.painter().image(
            texture.id(),
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

fn legacy_image_button(
    ui: &mut egui::Ui,
    textures: &std::collections::HashMap<String, egui::TextureHandle>,
    name: &str,
    rect: egui::Rect,
) -> Option<egui::Response> {
    let texture = textures.get(name)?;
    Some(ui.put(
        rect,
        egui::Image::new((texture.id(), rect.size())).sense(egui::Sense::click()),
    ))
}

fn legacy_title_bar(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    textures: &std::collections::HashMap<String, egui::TextureHandle>,
    origin: egui::Pos2,
    width: f32,
    icon_x: f32,
    minimize_x: f32,
    close_x: f32,
) {
    let bar = legacy_rect(origin, 0.0, 0.0, width, 35.0);
    ui.painter().rect_filled(bar, 0.0, egui::Color32::from_rgb(0, 0, 128));

    legacy_static_image(
        ui,
        textures,
        "chat_icon",
        legacy_rect(origin, icon_x, 0.0, 35.0, 35.0),
    );

    if let Some(response) = legacy_image_button(
        ui,
        textures,
        "minimize",
        legacy_rect(origin, minimize_x, 0.0, 35.0, 35.0),
    ) {
        if response.clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
    }

    if let Some(response) = legacy_image_button(
        ui,
        textures,
        "close",
        legacy_rect(origin, close_x, 0.0, 35.0, 35.0),
    ) {
        if response.clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    let drag_rect = legacy_rect(origin, 35.0, 0.0, (width - 110.0).max(40.0), 35.0);
    let drag = ui.interact(drag_rect, egui::Id::new("tschat-legacy-title-drag"), egui::Sense::click_and_drag());
    if drag.drag_started() {
        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
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
            dark_theme: true, connected: false, show_inbox: false, private_message_open: false,
            confirm_delete_message: None,
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
            show_clear_chat_confirmation: false,
            radio_autoplay_pending: None,
            chat_styles: std::collections::HashMap::new(),
            chat_style_color: egui::Color32::BLACK,
            chat_style_font: "Candara".into(),
            chat_style_size: 16,
            chat_style_effect: "Regular".into(),
            #[cfg(feature = "voice")]
            voice_open: false,
            #[cfg(feature = "voice")]
            voice_model_path: String::new(),
            #[cfg(feature = "voice")]
            voice_model: None,
            #[cfg(feature = "voice")]
            voice_model_pending: None,
            #[cfg(feature = "voice")]
            voice_recorder: None,
            #[cfg(feature = "voice")]
            voice_recording_started: None,
            #[cfg(feature = "voice")]
            voice_transcription_pending: None,
            #[cfg(feature = "voice")]
            voice_status: "Выберите мультиязычную модель Whisper (.bin).".into(),
            show_smileys: false,
            edit_profile_open: false,
            chat_style_open: false,
            last_viewport_screen: Screen::Login,
            ui_textures: std::collections::HashMap::new(),
        }
    }
}

impl TsChatApp {
    fn connect(&mut self) {
        let cfg = self.config.clone();
        match self.runtime.block_on(Database::connect(&cfg)) {
            Ok(db) => match self.runtime.block_on(db.verify_schema()) {
                Ok(()) => {
                    self.db = Some(db);
                    self.connected = true;
                    self.status = "Подключено к существующей БД TSchat. Структура проверена; данные не изменялись.".into();
                }
                Err(e) => self.status = format!("Структура тестовой БД не прошла проверку: {e:#}"),
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
        if self.db.is_none() {
            self.connect();
            if self.db.is_none() {
                return;
            }
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
                self.chat_styles = self.run_db(|rt, db| rt.block_on(db.chat_styles())).unwrap_or_default();
                self.chat_style_color = parse_legacy_argb(&user.chat_color).unwrap_or(egui::Color32::BLACK);
                self.chat_style_font = if user.chat_font.is_empty() { "Candara".into() } else { user.chat_font.clone() };
                self.chat_style_size = user.chat_font_size.clamp(8, 48);
                self.chat_style_effect = if user.chat_font_style.is_empty() { "Regular".into() } else { user.chat_font_style.clone() };
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
        if self.db.is_none() {
            self.connect();
            if self.db.is_none() {
                return;
            }
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



    fn save_chat_style(&mut self) {
        let Some(username) = self.current_user.as_ref().map(|u| u.username.clone()) else { return; };
        let color = legacy_color_hex(self.chat_style_color);
        let font = self.chat_style_font.clone();
        let size = self.chat_style_size.clamp(8, 48);
        let effect = self.chat_style_effect.clone();
        match self.run_db(|rt, db| rt.block_on(db.update_chat_style(&username, &color, &font, size, &effect))) {
            Ok(()) => {
                let style = ChatStyle {
                    color: color.clone(), font: font.clone(), font_size: size, font_style: effect.clone(),
                };
                self.chat_styles.insert(username.clone(), style);
                if let Some(user) = self.current_user.as_mut() {
                    user.chat_color = color;
                    user.chat_font = font;
                    user.chat_font_size = size;
                    user.chat_font_style = effect;
                }
                self.status = "Оформление сообщений сохранено.".into();
            }
            Err(error) => self.status = format!("Не удалось сохранить оформление: {error:#}"),
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
        if !password.is_empty() && password != self.repeat_password {
            self.status = "Новые пароли не совпадают.".into();
            return;
        }

        let password_hash = if password.is_empty() {
            None
        } else {
            match Self::hash_password(&password) {
                Ok(hash) => Some(hash),
                Err(error) => {
                    self.status = format!("Не удалось защитить новый пароль: {error:#}");
                    return;
                }
            }
        };

        match self.run_db(|rt, db| {
            rt.block_on(db.update_profile_with_messages(
                &old_username,
                &new_username,
                &email,
                &name,
                &mobile,
                password_hash.as_deref(),
            ))
        }) {
            Ok(()) => {
                if let Some(user) = self.current_user.as_mut() {
                    user.username = new_username.clone();
                    user.email = email.clone();
                    user.display_name = name.clone();
                    user.mobile = mobile.clone();
                }
                // Preserve the old username's style for historical chat lines and copy it to the new name.
                if let Some(style) = self.chat_styles.get(&old_username).cloned() {
                    self.chat_styles.insert(new_username.clone(), style);
                }
                self.status = if password_hash.is_some() {
                    "Профиль, никнейм, личные сообщения и пароль сохранены.".into()
                } else if old_username != new_username {
                    "Профиль обновлён; старые личные сообщения перенесены на новый никнейм.".into()
                } else {
                    "Профиль сохранён.".into()
                };

                self.new_password.clear();
                self.repeat_password.clear();
                if let Ok(friends) = self.run_db(|rt, db| rt.block_on(db.users_except(&new_username))) {
                    self.friends = friends;
                }
            }
            Err(error) => {
                self.status = format!("Не удалось сохранить профиль: {error:#}");
            }
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
            ChatCommand::RequestClearChat => {
                self.show_clear_chat_confirmation = true;
                self.command_status = "Подтвердите очистку общего чата.".into();
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



    #[cfg(feature = "voice")]
    fn finish_voice_recording(&mut self) {
        self.voice_recording_started = None;
        let Some(recorder) = self.voice_recorder.take() else { return; };
        let Some(model) = self.voice_model.clone() else {
            self.voice_status = "Сначала загрузите модель Whisper.".into();
            return;
        };

        match recorder.finish() {
            Ok(audio) => {
                let (sender, receiver) = std::sync::mpsc::channel();
                self.runtime.spawn_blocking(move || {
                    let result = voice::transcribe(model, &audio)
                        .map_err(|error| format!("{error:#}"));
                    let _ = sender.send(result);
                });
                self.voice_transcription_pending = Some(receiver);
                self.voice_status = "Распознаю русскую команду…".into();
            }
            Err(error) => self.voice_status = format!("Не удалось завершить запись: {error:#}"),
        }
    }

    #[cfg(feature = "voice")]
    fn show_voice_window(&mut self, ctx: &egui::Context) {
        // Always stop the capture device automatically after the 12-second limit.
        if self.voice_recording_started
            .map(|started| started.elapsed() >= Duration::from_secs(12))
            .unwrap_or(false)
        {
            self.finish_voice_recording();
        }

        let model_result = self.voice_model_pending.as_ref().map(|receiver| receiver.try_recv());
        if let Some(result) = model_result {
            match result {
                Ok(Ok(model)) => {
                    self.voice_model = Some(model);
                    self.voice_model_pending = None;
                    self.voice_status = "Whisper-модель загружена. Микрофон включается только кнопкой записи.".into();
                }
                Ok(Err(error)) => {
                    self.voice_model_pending = None;
                    self.voice_model = None;
                    self.voice_status = format!("Не удалось загрузить модель: {error}");
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    ctx.request_repaint_after(Duration::from_millis(250));
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.voice_model_pending = None;
                    self.voice_status = "Загрузка модели прервана.".into();
                }
            }
        }

        let transcription_result = self.voice_transcription_pending.as_ref().map(|receiver| receiver.try_recv());
        if let Some(result) = transcription_result {
            match result {
                Ok(Ok(text)) => {
                    self.voice_transcription_pending = None;
                    self.command_input = text.clone();
                    self.execute_command(&text);
                    self.voice_status = format!("Распознано: {text}");
                }
                Ok(Err(error)) => {
                    self.voice_transcription_pending = None;
                    self.voice_status = format!("Ошибка распознавания: {error}");
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    ctx.request_repaint_after(Duration::from_millis(250));
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.voice_transcription_pending = None;
                    self.voice_status = "Распознавание прервано.".into();
                }
            }
        }

        if !self.voice_open { return; }

        let model_loaded = self.voice_model.is_some();
        let loading = self.voice_model_pending.is_some();
        let transcribing = self.voice_transcription_pending.is_some();
        let recording = self.voice_recorder.is_some();
        let model_path = self.voice_model_path.clone();
        let status = self.voice_status.clone();
        let mut choose_model = false;
        let mut start_recording = false;
        let mut stop_recording = false;

        if loading || transcribing || recording {
            ctx.request_repaint_after(Duration::from_millis(200));
        }

        egui::Window::new("Голосовое управление")
            .open(&mut self.voice_open)
            .default_size([500.0, 270.0])
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Локальное распознавание речи на русском языке (Whisper).");
                ui.label("Запись запускается вручную; микрофон не прослушивается в фоне.");
                ui.separator();
                if model_path.is_empty() {
                    ui.label("Модель не выбрана.");
                } else {
                    ui.label(format!("Модель: {model_path}"));
                }
                if ui.add_enabled(!loading && !recording && !transcribing,
                    egui::Button::new("Выбрать модель Whisper (.bin)")).clicked() {
                    choose_model = true;
                }
                ui.horizontal(|ui| {
                    if ui.add_enabled(model_loaded && !recording && !transcribing,
                        egui::Button::new("● Начать запись")).clicked() {
                        start_recording = true;
                    }
                    if ui.add_enabled(recording,
                        egui::Button::new("■ Остановить и распознать")).clicked() {
                        stop_recording = true;
                    }
                });
                if loading { ui.label("Загрузка модели…"); }
                if transcribing { ui.label("Распознавание речи…"); }
                if recording { ui.label("Идёт запись (максимум 12 секунд)…"); }
                ui.separator();
                ui.label(&status);
                ui.label("Подойдёт мультиязычная ggml-модель, например ggml-base.bin.");
            });

        if choose_model {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Whisper model", &["bin"])
                .pick_file()
            {
                let path_string = path.display().to_string();
                self.voice_model_path = path_string.clone();
                self.voice_model = None;
                let (sender, receiver) = std::sync::mpsc::channel();
                self.runtime.spawn_blocking(move || {
                    let result = voice::load_model(&path_string)
                        .map_err(|error| format!("{error:#}"));
                    let _ = sender.send(result);
                });
                self.voice_model_pending = Some(receiver);
                self.voice_status = "Загружаю модель Whisper…".into();
            }
        }

        if start_recording {
            match MicrophoneRecorder::start() {
                Ok(recorder) => {
                    self.voice_recorder = Some(recorder);
                    self.voice_recording_started = Some(Instant::now());
                    self.voice_status = "Запись началась. Скажите одну команду; через 12 секунд она остановится автоматически.".into();
                }
                Err(error) => self.voice_status = format!("Не удалось включить микрофон: {error:#}"),
            }
        }

        if stop_recording {
            self.finish_voice_recording();
        }

        // Closing the voice window cancels an active recording and releases the microphone.
        if !self.voice_open && self.voice_recorder.is_some() {
            self.voice_recorder = None;
            self.voice_recording_started = None;
            self.voice_status = "Запись отменена; микрофон выключен.".into();
        }
    }


    fn clear_shared_chat(&mut self) {
        const WELCOME: &str = "Добро пожаловать в Таверну Странника";
        match self.run_db(|rt, db| rt.block_on(db.write_chat(WELCOME))) {
            Ok(()) => {
                self.chat_text = WELCOME.into();
                self.status = "Общий чат очищен.".into();
                self.command_status = "Общий чат очищен.".into();
            }
            Err(error) => {
                self.status = format!("Не удалось очистить общий чат: {error:#}");
                self.command_status = self.status.clone();
            }
        }
    }

    fn show_clear_chat_confirmation(&mut self, ctx: &egui::Context) {
        if !self.show_clear_chat_confirmation { return; }
        let mut confirm = false;
        let mut cancel = false;
        egui::Window::new("Очистить общий чат?")
            .open(&mut self.show_clear_chat_confirmation)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("История общего чата будет заменена приветственным сообщением.");
                ui.label("Это действие необратимо для текущей истории.");
                ui.horizontal(|ui| {
                    if ui.button("Очистить чат").clicked() { confirm = true; }
                    if ui.button("Отмена").clicked() { cancel = true; }
                });
            });
        if confirm {
            self.show_clear_chat_confirmation = false;
            self.clear_shared_chat();
        } else if cancel {
            self.show_clear_chat_confirmation = false;
            self.command_status = "Очистка чата отменена.".into();
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

    fn delete_inbox_message(&mut self, message_id: i64) {
        let Some(username) = self.current_user.as_ref().map(|user| user.username.clone()) else { return; };
        match self.run_db(|rt, db| rt.block_on(db.delete_private_message(message_id, &username))) {
            Ok(1) => {
                self.inbox.retain(|message| message.id != message_id);
                self.status = "Личное сообщение удалено.".into();
            }
            Ok(_) => {
                self.status = "Сообщение уже удалено или не принадлежит текущему пользователю.".into();
            }
            Err(error) => self.status = format!("Не удалось удалить личное сообщение: {error:#}"),
        }
        self.confirm_delete_message = None;
    }
}

impl TsChatApp {
    fn ensure_legacy_ui_textures(&mut self, ctx: &egui::Context) {
        if !self.ui_textures.is_empty() {
            return;
        }

        let assets: [(&str, &[u8]); 30] = [
            ("brand_logo", include_bytes!("../Resources/32323.png")),
            ("tavern_art", include_bytes!("../Resources/222.gif")),
            ("theme_day", include_bytes!("../Resources/b_day.png")),
            ("theme_night", include_bytes!("../Resources/b_night.png")),
            ("chat_icon", include_bytes!("../assets/ICOCOS1.png")),
            ("icq_logo", include_bytes!("../assets/icq_logo.png")),
            ("minimize", include_bytes!("../assets/remove.png")),
            ("close", include_bytes!("../assets/clear_search.png")),
            ("send_dark", include_bytes!("../assets/Send_mail.png")),
            ("send_light", include_bytes!("../assets/Send_mail_b.png")),
            ("attach_dark", include_bytes!("../assets/prikrep.png")),
            ("attach_light", include_bytes!("../assets/prikrep_b.png")),
            ("profile_dark", include_bytes!("../assets/profile.png")),
            ("profile_light", include_bytes!("../assets/profile_b.png")),
            ("file_dark", include_bytes!("../assets/Fail.png")),
            ("file_light", include_bytes!("../assets/Fail_b.png")),
            ("edit_dark", include_bytes!("../assets/redakt.png")),
            ("edit_light", include_bytes!("../assets/redakt_b.png")),
            ("fonts_dark", include_bytes!("../assets/fontes.png")),
            ("fonts_light", include_bytes!("../assets/fontes_b.png")),
            ("clear_dark", include_bytes!("../assets/clear.png")),
            ("clear_light", include_bytes!("../assets/clear_b.png")),
            ("pm_dark", include_bytes!("../assets/lmseg.png")),
            ("pm_light", include_bytes!("../assets/lmseg_b.png")),
            ("radio_dark", include_bytes!("../assets/radio.png")),
            ("radio_light", include_bytes!("../assets/radio_b.png")),
            ("notes_dark", include_bytes!("../assets/pencil.png")),
            ("notes_light", include_bytes!("../assets/pencil_b.png")),
            ("icq_dark", include_bytes!("../assets/icq_lgtp.png")),
            ("icq_light", include_bytes!("../assets/icq_lgtp_b.png")),
        ];

        let mut loaded = std::collections::HashMap::with_capacity(assets.len());
        for (name, bytes) in assets {
            let Ok(decoded) = image::load_from_memory(bytes) else { continue; };
            let rgba = decoded.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let color = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
            let texture = ctx.load_texture(
                format!("tschat-legacy-{name}"),
                color,
                egui::TextureOptions::LINEAR,
            );
            loaded.insert(name.to_string(), texture);
        }
        self.ui_textures = loaded;
    }

    fn draw_legacy_auth_screen(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let origin = ui.max_rect().min;
        let dark = self.dark_theme;
        let background = if dark { egui::Color32::BLACK } else { egui::Color32::WHITE };
        let foreground = if dark { egui::Color32::WHITE } else { egui::Color32::BLACK };
        let field_foreground = egui::Color32::BLACK;

        ui.painter().rect_filled(ui.max_rect(), 0.0, background);
        legacy_title_bar(ui, ctx, &self.ui_textures, origin, 590.0, -1.0, 509.0, 550.0);

        if self.screen == Screen::Login {
            legacy_static_image(ui, &self.ui_textures, "brand_logo", legacy_rect(origin, 345.0, 32.0, 193.0, 140.0));
            legacy_static_image(ui, &self.ui_textures, "tavern_art", legacy_rect(origin, 273.0, 95.0, 315.0, 315.0));

            let group = legacy_rect(origin, 17.0, 41.0, 260.0, 389.0);
            legacy_group(ui, group, background, egui::Color32::DARK_GRAY);
            legacy_text(ui, legacy_rect(origin, 29.0, 45.0, 220.0, 24.0), "Авторизация", foreground, 17.0);

            legacy_text(ui, legacy_rect(origin, 71.0, 86.0, 162.0, 24.0), "Имя пользователя", foreground, 15.0);
            legacy_text_edit(
                ui,
                legacy_rect(origin, 50.0, 113.0, 196.0, 30.0),
                &mut self.username,
                "Никнейм",
                false,
                field_foreground,
            );
            legacy_text(ui, legacy_rect(origin, 113.0, 159.0, 100.0, 24.0), "Пароль", foreground, 15.0);
            legacy_text_edit(
                ui,
                legacy_rect(origin, 50.0, 186.0, 196.0, 30.0),
                &mut self.password,
                "Пароль",
                true,
                field_foreground,
            );

            if let Some(response) = legacy_image_button(
                ui,
                &self.ui_textures,
                "icq_logo",
                legacy_rect(origin, 50.0, 237.0, 33.0, 33.0),
            ) {
                if response.clicked() {
                    self.status = "TSchat".into();
                }
            }
            if legacy_button(
                ui,
                legacy_rect(origin, 106.0, 237.0, 140.0, 33.0),
                "Вход",
                egui::Color32::BLACK,
                egui::Color32::WHITE,
                16.0,
            )
            .clicked()
            {
                self.login();
            }

            let theme_image = if dark { "theme_day" } else { "theme_night" };
            if let Some(response) = legacy_image_button(
                ui,
                &self.ui_textures,
                theme_image,
                legacy_rect(origin, 98.0, 301.0, 97.0, 45.0),
            ) {
                if response.clicked() {
                    self.dark_theme = !self.dark_theme;
                }
            }

            let status_text = if self.connected { "Подключено к MySQL" } else { &self.status };
            legacy_text(
                ui,
                legacy_rect(origin, 79.0, 349.0, 185.0, 31.0),
                status_text.to_string(),
                egui::Color32::from_rgb(46, 139, 87),
                10.0,
            );
            if legacy_button(
                ui,
                legacy_rect(origin, 75.0, 384.0, 145.0, 24.0),
                "Создать аккаунт",
                background,
                foreground,
                12.0,
            )
            .clicked()
            {
                self.screen = Screen::Register;
                self.status.clear();
                self.email.clear();
            }
        } else {
            legacy_static_image(ui, &self.ui_textures, "brand_logo", legacy_rect(origin, 80.0, 32.0, 193.0, 140.0));
            legacy_static_image(ui, &self.ui_textures, "tavern_art", legacy_rect(origin, 4.0, 95.0, 315.0, 315.0));

            let group = legacy_rect(origin, 334.0, 41.0, 243.0, 389.0);
            legacy_group(ui, group, background, egui::Color32::DARK_GRAY);
            legacy_text(ui, legacy_rect(origin, 346.0, 45.0, 215.0, 24.0), "Регистрация", foreground, 17.0);

            legacy_text(ui, legacy_rect(origin, 374.0, 68.0, 162.0, 24.0), "Имя пользователя", foreground, 14.0);
            legacy_text_edit(ui, legacy_rect(origin, 357.0, 99.0, 196.0, 30.0), &mut self.username, "Никнейм", false, field_foreground);
            legacy_text(ui, legacy_rect(origin, 360.0, 132.0, 188.0, 24.0), "Придумайте пароль", foreground, 13.0);
            legacy_text_edit(ui, legacy_rect(origin, 357.0, 159.0, 196.0, 30.0), &mut self.password, "Пароль", true, field_foreground);
            legacy_text(ui, legacy_rect(origin, 393.0, 192.0, 131.0, 24.0), "Введите почту", foreground, 14.0);
            legacy_text_edit(ui, legacy_rect(origin, 357.0, 219.0, 196.0, 30.0), &mut self.email, "Почта", false, field_foreground);

            if legacy_button(
                ui,
                legacy_rect(origin, 413.0, 265.0, 140.0, 33.0),
                "Регистрация",
                egui::Color32::BLACK,
                egui::Color32::WHITE,
                14.0,
            )
            .clicked()
            {
                self.register();
            }

            legacy_text(
                ui,
                legacy_rect(origin, 346.0, 316.0, 225.0, 36.0),
                self.status.clone(),
                egui::Color32::from_rgb(46, 139, 87),
                10.0,
            );
            if legacy_button(
                ui,
                legacy_rect(origin, 397.0, 350.0, 145.0, 25.0),
                "Авторизация",
                background,
                foreground,
                12.0,
            )
            .clicked()
            {
                self.screen = Screen::Login;
                self.status.clear();
            }
        }

        // The original form has a tiny theme checkbox in its lower-right corner.
        let checker = legacy_button(
            ui,
            legacy_rect(origin, 577.0, 426.0, 12.0, 9.0),
            if dark { "✓" } else { " " },
            background,
            foreground,
            8.0,
        );
        if checker.clicked() {
            self.dark_theme = !self.dark_theme;
        }
    }

    fn draw_legacy_chat_screen(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let origin = ui.max_rect().min;
        let dark = self.dark_theme;
        let background = if dark { egui::Color32::BLACK } else { egui::Color32::WHITE };
        let panel = background;
        let foreground = if dark { egui::Color32::WHITE } else { egui::Color32::BLACK };
        let border = if dark { egui::Color32::DARK_GRAY } else { egui::Color32::GRAY };
        let menu_fill = background;

        ui.painter().rect_filled(ui.max_rect(), 0.0, background);
        legacy_title_bar(ui, ctx, &self.ui_textures, origin, 796.0, 2.0, 716.0, 757.0);

        // Original WinForms MenuStrip: File, Fonts, Clear, Private messages, Entertainment and ICQ.
        let menu_y = 38.0;
        let mut x = 0.0;
        let menu_textures = self.ui_textures.clone();
        let mut menu_item = |ui: &mut egui::Ui, text: &str, icon: &str, width: f32| {
            let rect = legacy_rect(origin, x, menu_y, width, 27.0);
            let response = ui.put(
                rect,
                egui::Button::new(egui::RichText::new(text).color(foreground).size(10.0))
                    .fill(menu_fill)
                    .stroke(egui::Stroke::new(0.0, menu_fill)),
            );
            legacy_static_image(ui, &menu_textures, icon, legacy_rect(origin, x + 3.0, menu_y + 5.0, 17.0, 17.0));
            x += width;
            response
        };

        let file_icon = if dark { "file_dark" } else { "file_light" };
        if menu_item(ui, "Файл", file_icon, 67.0).clicked() {
            self.edit_profile_open = true;
        }
        let fonts_icon = if dark { "fonts_dark" } else { "fonts_light" };
        if menu_item(ui, "Шрифты", fonts_icon, 90.0).clicked() {
            self.chat_style_open = true;
        }
        let clear_icon = if dark { "clear_dark" } else { "clear_light" };
        if menu_item(ui, "Очистка сообщений", clear_icon, 161.0).clicked() {
            self.show_clear_chat_confirmation = true;
        }
        let pm_icon = if dark { "pm_dark" } else { "pm_light" };
        if menu_item(ui, "Личные сообщения", pm_icon, 150.0).clicked() {
            self.load_inbox();
        }
        let radio_icon = if dark { "radio_dark" } else { "radio_light" };
        if menu_item(ui, "Развлечения", radio_icon, 98.0).clicked() {
            self.radio_open = true;
        }
        let notes_icon = if dark { "notes_dark" } else { "notes_light" };
        if menu_item(ui, "Заметки", notes_icon, 78.0).clicked() {
            self.notes_open = true;
        }
        let icq_icon = if dark { "icq_dark" } else { "icq_light" };
        if menu_item(ui, "ICQ", icq_icon, 62.0).clicked() {
            self.status = "ICQ-модуль пока не перенесён.".into();
        }
        drop(menu_item);

        let chat_group = legacy_rect(origin, 14.0, 72.0, 437.0, 275.0);
        legacy_group(ui, chat_group, panel, border);
        legacy_text(ui, legacy_rect(origin, 22.0, 74.0, 180.0, 18.0), "Общий чат", foreground, 10.0);

        let chat_rect = legacy_rect(origin, 21.0, 83.0, 424.0, 258.0);
        let smileys = self.smiley_textures.clone();
        let chat_styles = self.chat_styles.clone();
        ui.allocate_ui_at_rect(chat_rect, |chat_ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(chat_ui, |chat_content| {
                    for line in self.chat_text.lines() {
                        show_chat_line(chat_content, line, &smileys, &chat_styles);
                    }
                });
        });

        let users_group = legacy_rect(origin, 498.0, 72.0, 285.0, 275.0);
        legacy_group(ui, users_group, panel, border);
        let avatar_rect = legacy_rect(origin, 504.0, 87.0, 120.0, 120.0);
        if let Some(texture) = &self.avatar_texture {
            ui.painter().image(
                texture.id(),
                avatar_rect,
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        } else {
            ui.painter().rect_filled(avatar_rect, 0.0, egui::Color32::DARK_GRAY);
            legacy_text(ui, legacy_rect(origin, 512.0, 137.0, 105.0, 20.0), "Аватар", foreground, 12.0);
        }
        let avatar_response = ui.interact(avatar_rect, egui::Id::new("legacy-own-avatar"), egui::Sense::click());
        if avatar_response.clicked() {
            self.edit_profile_open = true;
        }

        let info_group = legacy_rect(origin, 630.0, 80.0, 146.0, 127.0);
        legacy_group(ui, info_group, panel, border);
        let user = self.current_user.as_ref();
        legacy_text(ui, legacy_rect(origin, 636.0, 91.0, 132.0, 24.0), user.map(|u| u.username.as_str()).unwrap_or("Никнейм"), foreground, 12.0);
        legacy_text(ui, legacy_rect(origin, 636.0, 158.0, 132.0, 18.0), user.map(|u| u.mobile.as_str()).unwrap_or("Номер телефона"), foreground, 10.0);
        legacy_text(ui, legacy_rect(origin, 636.0, 181.0, 132.0, 20.0), user.map(|u| u.email.as_str()).unwrap_or("Почта"), foreground, 10.0);

        let friends_rect = legacy_rect(origin, 507.0, 217.0, 268.0, 119.0);
        ui.allocate_ui_at_rect(friends_rect, |friends_ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(friends_ui, |list_ui| {
                for name in self.friends.clone() {
                    let selected = self.selected_friend.as_deref() == Some(name.as_str());
                    if list_ui.selectable_label(selected, &name).clicked() {
                        self.selected_friend = Some(name.clone());
                        self.load_selected_profile();
                    }
                }
            });
        });

        let profile_icon = if dark { "profile_dark" } else { "profile_light" };
        legacy_static_image(
            ui,
            &self.ui_textures,
            profile_icon,
            legacy_rect(origin, 466.0, 91.0, 32.0, 243.0),
        );
        let profile_hit = ui.interact(
            legacy_rect(origin, 466.0, 91.0, 32.0, 243.0),
            egui::Id::new("legacy-profile-button"),
            egui::Sense::click(),
        );
        if profile_hit.clicked() {
            self.edit_profile_open = true;
        }

        legacy_static_image(
            ui,
            &self.ui_textures,
            "brand_logo",
            legacy_rect(origin, 533.0, 322.0, 218.0, 140.0),
        );

        let composer_group = legacy_rect(origin, 14.0, 353.0, 437.0, 90.0);
        legacy_group(ui, composer_group, panel, border);
        let input_rect = legacy_rect(origin, 55.0, 364.0, 277.0, 72.0);
        ui.put(
            input_rect,
            egui::TextEdit::multiline(&mut self.draft)
                .desired_width(input_rect.width())
                .desired_rows(3)
                .hint_text("Сообщение...")
                .text_color(foreground),
        );

        let send_icon = if dark { "send_dark" } else { "send_light" };
        if let Some(response) = legacy_image_button(
            ui,
            &self.ui_textures,
            send_icon,
            legacy_rect(origin, 338.0, 372.0, 50.0, 50.0),
        ) {
            if response.clicked() {
                self.send_chat();
            }
        }

        let attach_icon = if dark { "attach_dark" } else { "attach_light" };
        if let Some(response) = legacy_image_button(
            ui,
            &self.ui_textures,
            attach_icon,
            legacy_rect(origin, 396.0, 374.0, 45.0, 45.0),
        ) {
            if response.clicked() {
                self.status = "Прикрепление изображений в оригинальном клиенте было отключено.".into();
            }
        }

        let smiley_button = legacy_button(
            ui,
            legacy_rect(origin, 17.0, 400.0, 35.0, 35.0),
            "☺",
            background,
            foreground,
            22.0,
        );
        if smiley_button.clicked() {
            self.show_smileys = !self.show_smileys;
        }

        if self.show_smileys {
            let picker = legacy_rect(origin, 55.0, 87.0, 357.0, 96.0);
            legacy_group(ui, picker, background, border);
            let positions = [2.0, 46.0, 90.0, 134.0, 178.0, 222.0, 266.0, 311.0];
            for (index, (code, texture)) in self.smiley_textures.clone().iter().enumerate().take(16) {
                let row = index / 8;
                let column = index % 8;
                let rect = legacy_rect(origin, 55.0 + positions[column], 87.0 + 9.0 + row as f32 * 42.0, 42.0, 42.0);
                let response = ui.put(
                    rect,
                    egui::Image::new((texture.id(), rect.size())).sense(egui::Sense::click()),
                );
                if response.clicked() {
                    self.add_smiley(code);
                    self.show_smileys = false;
                }
            }
        }

        // The original menu exposes profile editing under File and chat appearance under Fonts.
        let current_user_name = self.current_user.as_ref().map(|u| u.username.clone()).unwrap_or_default();
        let connection = if self.connected { "MySQL: подключено" } else { "MySQL: нет подключения" };
        legacy_text(
            ui,
            legacy_rect(origin, 500.0, 440.0, 280.0, 10.0),
            format!("{connection}  {current_user_name}"),
            if self.connected { egui::Color32::from_rgb(46, 139, 87) } else { egui::Color32::GRAY },
            8.0,
        );
    }

    fn show_legacy_profile_window(&mut self, ctx: &egui::Context) {
        if !self.edit_profile_open {
            return;
        }
        let mut open = self.edit_profile_open;
        let mut close_requested = false;
        egui::Window::new("Редактирование профиля")
            .id(egui::Id::new("legacy-edit-profile-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([617.0, 384.0])
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let origin = rect.min;
                let dark = self.dark_theme;
                let bg = if dark { egui::Color32::BLACK } else { egui::Color32::WHITE };
                let fg = if dark { egui::Color32::WHITE } else { egui::Color32::BLACK };
                ui.painter().rect_filled(rect, 0.0, bg);
                ui.painter().rect_filled(legacy_rect(origin, 0.0, 0.0, 617.0, 35.0), 0.0, egui::Color32::from_rgb(0, 0, 128));
                legacy_static_image(ui, &self.ui_textures, "chat_icon", legacy_rect(origin, 1.0, 0.0, 35.0, 35.0));
                legacy_static_image(ui, &self.ui_textures, "minimize", legacy_rect(origin, 535.0, 0.0, 35.0, 35.0));
                if let Some(response) = legacy_image_button(ui, &self.ui_textures, "close", legacy_rect(origin, 576.0, 0.0, 35.0, 35.0)) {
                    if response.clicked() { close_requested = true; }
                }
                ui.painter().rect_filled(legacy_rect(origin, 0.0, 35.0, 617.0, 36.0), 0.0, egui::Color32::from_rgb(128, 0, 0));
                legacy_text(ui, legacy_rect(origin, 12.0, 39.0, 250.0, 26.0), "Редактировать профиль", fg, 17.0);

                legacy_group(ui, legacy_rect(origin, 12.0, 78.0, 593.0, 294.0), bg, egui::Color32::DARK_GRAY);
                legacy_text(ui, legacy_rect(origin, 31.0, 94.0, 190.0, 24.0), "Настоящее имя", fg, 12.0);
                legacy_text_edit(ui, legacy_rect(origin, 36.0, 123.0, 237.0, 30.0), &mut self.profile_name, "Имя", false, egui::Color32::BLACK);
                legacy_text(ui, legacy_rect(origin, 31.0, 156.0, 120.0, 24.0), "Никнейм", fg, 12.0);
                legacy_text_edit(ui, legacy_rect(origin, 36.0, 185.0, 237.0, 30.0), &mut self.profile_username, "Никнейм", false, egui::Color32::BLACK);
                legacy_text(ui, legacy_rect(origin, 31.0, 218.0, 100.0, 24.0), "Почта", fg, 12.0);
                legacy_text_edit(ui, legacy_rect(origin, 36.0, 247.0, 237.0, 30.0), &mut self.profile_email, "Почта", false, egui::Color32::BLACK);
                legacy_text(ui, legacy_rect(origin, 31.0, 280.0, 100.0, 24.0), "Телефон", fg, 12.0);
                legacy_text_edit(ui, legacy_rect(origin, 36.0, 309.0, 237.0, 30.0), &mut self.profile_mobile, "Телефон", false, egui::Color32::BLACK);

                legacy_group(ui, legacy_rect(origin, 296.0, 97.0, 291.0, 195.0), bg, egui::Color32::DARK_GRAY);
                legacy_text(ui, legacy_rect(origin, 302.0, 124.0, 180.0, 24.0), "Введите пароль", fg, 12.0);
                legacy_text_edit(ui, legacy_rect(origin, 307.0, 153.0, 237.0, 30.0), &mut self.new_password, "Новый пароль", true, egui::Color32::BLACK);
                legacy_text(ui, legacy_rect(origin, 302.0, 186.0, 200.0, 24.0), "Повторите пароль", fg, 12.0);
                legacy_text_edit(ui, legacy_rect(origin, 307.0, 215.0, 237.0, 30.0), &mut self.repeat_password, "Повторите пароль", true, egui::Color32::BLACK);

                if legacy_button(
                    ui,
                    legacy_rect(origin, 329.0, 301.0, 224.0, 38.0),
                    "Сохранить изменения",
                    egui::Color32::BLACK,
                    egui::Color32::WHITE,
                    14.0,
                )
                .clicked()
                {
                    self.save_profile();
                    if !self.status.starts_with("Не удалось") && !self.status.starts_with("Имя,") {
                        self.edit_profile_open = false;
                    }
                }
            });
        if close_requested { open = false; }
        self.edit_profile_open = open && self.edit_profile_open;
    }

    fn show_legacy_chat_style_window(&mut self, ctx: &egui::Context) {
        if !self.chat_style_open {
            return;
        }
        let mut open = self.chat_style_open;
        let mut close_requested = false;
        egui::Window::new("Оформление сообщений")
            .id(egui::Id::new("legacy-chat-style-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([390.0, 260.0])
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let origin = rect.min;
                let bg = if self.dark_theme { egui::Color32::BLACK } else { egui::Color32::WHITE };
                let fg = if self.dark_theme { egui::Color32::WHITE } else { egui::Color32::BLACK };
                ui.painter().rect_filled(rect, 0.0, bg);
                ui.painter().rect_filled(legacy_rect(origin, 0.0, 0.0, 390.0, 35.0), 0.0, egui::Color32::from_rgb(0, 0, 128));
                legacy_static_image(ui, &self.ui_textures, "chat_icon", legacy_rect(origin, 0.0, 0.0, 35.0, 35.0));
                if let Some(response) = legacy_image_button(ui, &self.ui_textures, "close", legacy_rect(origin, 349.0, 0.0, 35.0, 35.0)) {
                    if response.clicked() { close_requested = true; }
                }
                legacy_text(ui, legacy_rect(origin, 12.0, 43.0, 300.0, 24.0), "Оформление сообщений", fg, 15.0);

                legacy_text(ui, legacy_rect(origin, 12.0, 78.0, 180.0, 20.0), "Цвет", fg, 12.0);
                ui.allocate_ui_at_rect(legacy_rect(origin, 150.0, 75.0, 50.0, 28.0), |color_ui| {
                    color_ui.color_edit_button_srgba(&mut self.chat_style_color);
                });
                legacy_text(ui, legacy_rect(origin, 12.0, 112.0, 180.0, 20.0), "Шрифт", fg, 12.0);
                egui::ComboBox::from_id_source("legacy-chat-font")
                    .selected_text(&self.chat_style_font)
                    .show_ui(ui, |ui| {
                        for font in ["Candara", "Arial", "Tahoma", "Verdana", "Consolas"] {
                            ui.selectable_value(&mut self.chat_style_font, font.to_string(), font);
                        }
                    });
                ui.put(legacy_rect(origin, 12.0, 149.0, 350.0, 22.0), egui::Slider::new(&mut self.chat_style_size, 8..=36).text("Размер"));
                legacy_text(ui, legacy_rect(origin, 12.0, 179.0, 150.0, 20.0), "Начертание", fg, 12.0);
                egui::ComboBox::from_id_source("legacy-chat-style")
                    .selected_text(&self.chat_style_effect)
                    .show_ui(ui, |ui| {
                        for effect in ["Regular", "Bold", "Italic", "Underline", "Strikeout"] {
                            ui.selectable_value(&mut self.chat_style_effect, effect.to_string(), effect);
                        }
                    });
                if legacy_button(
                    ui,
                    legacy_rect(origin, 176.0, 216.0, 190.0, 34.0),
                    "Сохранить",
                    egui::Color32::BLACK,
                    egui::Color32::WHITE,
                    13.0,
                )
                .clicked()
                {
                    self.save_chat_style();
                    self.chat_style_open = false;
                }
            });
        if close_requested { open = false; }
        self.chat_style_open = open && self.chat_style_open;
    }

}


impl TsChatApp {
    fn show_legacy_friend_profile_window(&mut self, ctx: &egui::Context) {
        if !self.show_friend_profile {
            return;
        }
        let mut open = self.show_friend_profile;
        let mut close_requested = false;
        let mut send_requested = false;
        let profile = self.friend_profile.clone();

        egui::Window::new("Профиль друга")
            .id(egui::Id::new("legacy-friend-profile-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([595.0, 356.0])
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let origin = rect.min;
                let dark = self.dark_theme;
                let bg = if dark { egui::Color32::BLACK } else { egui::Color32::WHITE };
                let fg = if dark { egui::Color32::WHITE } else { egui::Color32::BLACK };
                ui.painter().rect_filled(rect, 0.0, bg);

                ui.painter().rect_filled(
                    legacy_rect(origin, 0.0, 0.0, 595.0, 35.0),
                    0.0,
                    egui::Color32::from_rgb(0, 0, 128),
                );
                legacy_static_image(ui, &self.ui_textures, "chat_icon", legacy_rect(origin, 1.0, 0.0, 35.0, 35.0));
                legacy_static_image(ui, &self.ui_textures, "minimize", legacy_rect(origin, 516.0, 0.0, 35.0, 35.0));
                if let Some(response) = legacy_image_button(
                    ui,
                    &self.ui_textures,
                    "close",
                    legacy_rect(origin, 557.0, 0.0, 35.0, 35.0),
                ) {
                    if response.clicked() {
                        close_requested = true;
                    }
                }

                ui.painter().rect_filled(legacy_rect(origin, 0.0, 36.0, 595.0, 36.0), 0.0, bg);
                legacy_text(ui, legacy_rect(origin, 12.0, 40.0, 200.0, 26.0), "Профиль друга", fg, 16.0);

                legacy_group(ui, legacy_rect(origin, 13.0, 78.0, 569.0, 266.0), bg, egui::Color32::DARK_GRAY);

                let avatar_rect = legacy_rect(origin, 34.0, 97.0, 150.0, 150.0);
                if let Some(texture) = &self.friend_avatar_texture {
                    ui.painter().image(
                        texture.id(),
                        avatar_rect,
                        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                } else {
                    ui.painter().rect_filled(avatar_rect, 0.0, egui::Color32::DARK_GRAY);
                }

                legacy_group(ui, legacy_rect(origin, 203.0, 91.0, 363.0, 238.0), bg, egui::Color32::DARK_GRAY);
                if let Some(user) = profile.as_ref() {
                    let labels = [
                        ("Настоящее имя", &user.display_name, 107.0),
                        ("Никнейм", &user.username, 155.0),
                        ("Почта", &user.email, 199.0),
                        ("Телефон", &user.mobile, 293.0),
                    ];
                    for (label, value, y) in labels {
                        legacy_text(ui, legacy_rect(origin, 209.0, y, 132.0, 23.0), label, fg, 11.0);
                        legacy_text(ui, legacy_rect(origin, 347.0, y, 205.0, 23.0), value, fg, 11.0);
                    }
                }

                let send_icon = if dark { "send_dark" } else { "send_light" };
                if let Some(response) = legacy_image_button(
                    ui,
                    &self.ui_textures,
                    send_icon,
                    legacy_rect(origin, 82.0, 253.0, 50.0, 50.0),
                ) {
                    if response.clicked() {
                        send_requested = true;
                    }
                }
                let send_label = legacy_text(
                    ui,
                    legacy_rect(origin, 21.0, 306.0, 180.0, 23.0),
                    "Отправить сообщение",
                    fg,
                    11.0,
                );
                if send_label.clicked() {
                    send_requested = true;
                }
            });

        if close_requested {
            open = false;
        }
        self.show_friend_profile = open && self.show_friend_profile;

        if send_requested {
            if let Some(user) = profile {
                self.selected_friend = Some(user.username);
                self.private_subject.clear();
                self.private_text.clear();
                self.private_message_open = true;
                self.show_friend_profile = false;
            }
        }
    }

    fn show_legacy_private_message_window(&mut self, ctx: &egui::Context) {
        if !self.private_message_open {
            return;
        }
        let Some(recipient) = self.selected_friend.clone() else {
            self.private_message_open = false;
            self.status = "Выберите собеседника перед отправкой личного сообщения.".into();
            return;
        };

        let mut open = self.private_message_open;
        let mut close_requested = false;
        let mut send_requested = false;

        egui::Window::new(format!("Личное сообщение для: {recipient}"))
            .id(egui::Id::new("legacy-private-message-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([643.0, 430.0])
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let origin = rect.min;
                let dark = self.dark_theme;
                let bg = if dark { egui::Color32::BLACK } else { egui::Color32::WHITE };
                let fg = if dark { egui::Color32::WHITE } else { egui::Color32::BLACK };
                ui.painter().rect_filled(rect, 0.0, bg);

                ui.painter().rect_filled(
                    legacy_rect(origin, 0.0, 0.0, 643.0, 35.0),
                    0.0,
                    egui::Color32::from_rgb(0, 0, 128),
                );
                legacy_static_image(ui, &self.ui_textures, "chat_icon", legacy_rect(origin, 1.0, 0.0, 35.0, 35.0));
                legacy_static_image(ui, &self.ui_textures, "minimize", legacy_rect(origin, 564.0, 0.0, 35.0, 35.0));
                if let Some(response) = legacy_image_button(
                    ui,
                    &self.ui_textures,
                    "close",
                    legacy_rect(origin, 605.0, 0.0, 35.0, 35.0),
                ) {
                    if response.clicked() {
                        close_requested = true;
                    }
                }

                ui.painter().rect_filled(legacy_rect(origin, 0.0, 35.0, 643.0, 36.0), 0.0, bg);
                legacy_text(
                    ui,
                    legacy_rect(origin, 12.0, 39.0, 330.0, 26.0),
                    "Отправить личное сообщение",
                    fg,
                    16.0,
                );
                legacy_text(
                    ui,
                    legacy_rect(origin, 18.0, 93.0, 260.0, 26.0),
                    "Тема сообщения",
                    fg,
                    12.0,
                );
                ui.put(
                    legacy_rect(origin, 18.0, 122.0, 606.0, 30.0),
                    egui::TextEdit::singleline(&mut self.private_subject)
                        .desired_width(606.0)
                        .hint_text("Тема")
                        .text_color(egui::Color32::BLACK),
                );
                legacy_text(
                    ui,
                    legacy_rect(origin, 18.0, 167.0, 260.0, 26.0),
                    "Текст сообщения",
                    fg,
                    12.0,
                );
                ui.put(
                    legacy_rect(origin, 18.0, 196.0, 606.0, 159.0),
                    egui::TextEdit::multiline(&mut self.private_text)
                        .desired_width(606.0)
                        .desired_rows(7)
                        .hint_text("Сообщение...")
                        .text_color(egui::Color32::BLACK),
                );

                let send_icon = if dark { "send_dark" } else { "send_light" };
                if let Some(response) = legacy_image_button(
                    ui,
                    &self.ui_textures,
                    send_icon,
                    legacy_rect(origin, 574.0, 361.0, 50.0, 50.0),
                ) {
                    if response.clicked() {
                        send_requested = true;
                    }
                }
                legacy_text(
                    ui,
                    legacy_rect(origin, 18.0, 371.0, 400.0, 22.0),
                    format!("Получатель: {recipient}"),
                    fg,
                    10.0,
                );
            });

        if close_requested {
            open = false;
        }
        if send_requested {
            self.send_private();
            if self.status.starts_with("Сообщение отправлено пользователю ") {
                open = false;
            }
        }
        self.private_message_open = open && self.private_message_open;
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

        if self.last_viewport_screen != self.screen {
            let size = match self.screen {
                Screen::Login | Screen::Register => [590.0, 437.0],
                Screen::Chat => [796.0, 451.0],
            };
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(size[0], size[1])));
            ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(false));
            self.last_viewport_screen = self.screen;
        }

        self.ensure_avatar_texture(ctx);
        self.ensure_smiley_textures(ctx);
        self.ensure_legacy_ui_textures(ctx);
        if self.show_friend_profile {
            self.ensure_friend_avatar_texture(ctx);
        }

        if self.dark_theme {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        let canvas = if self.dark_theme { egui::Color32::BLACK } else { egui::Color32::WHITE };
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(canvas).inner_margin(0.0))
            .show(ctx, |ui| {
                match self.screen {
                    Screen::Login | Screen::Register => self.draw_legacy_auth_screen(ctx, ui),
                    Screen::Chat => self.draw_legacy_chat_screen(ctx, ui),
                }
            });

        self.show_legacy_profile_window(ctx);
        self.show_legacy_chat_style_window(ctx);
        self.show_legacy_friend_profile_window(ctx);
        self.show_legacy_private_message_window(ctx);

        if self.show_inbox {
            let inbox_items = self.inbox.clone();
            let mut close_inbox = false;
            let mut delete_requested = None;
            let mut reply_requested: Option<(String, String)> = None;
            egui::Window::new("Входящие сообщения").show(ctx, |ui| {
                if ui.button("Закрыть").clicked() {
                    close_inbox = true;
                }
                for message in inbox_items {
                    ui.collapsing(
                        format!(
                            "{} — {}{}",
                            message.from_user,
                            message.subject,
                            if message.is_read { "" } else { " • новое" }
                        ),
                        |ui| {
                            ui.label(&message.message);
                            ui.horizontal(|ui| {
                                if !message.is_read && ui.button("Отметить прочитанным").clicked() {
                                    self.mark_message_read(message.id);
                                }
                                if ui.button("Ответить").clicked() {
                                    reply_requested = Some((message.from_user.clone(), message.subject.clone()));
                                }
                                if ui.button("Удалить").clicked() {
                                    delete_requested = Some(message.id);
                                }
                            });
                        },
                    );
                }
            });
            if close_inbox {
                self.show_inbox = false;
            }
            if let Some((sender, subject)) = reply_requested {
                self.selected_friend = Some(sender.clone());
                self.private_subject = if subject.trim().is_empty() {
                    "Re:".into()
                } else {
                    format!("Re: {subject}")
                };
                self.private_message_open = true;
                self.show_inbox = false;
                self.status = format!("Ответьте пользователю {sender} в окне личных сообщений.");
            }
            if let Some(message_id) = delete_requested {
                self.confirm_delete_message = Some(message_id);
            }
        }

        if let Some(message_id) = self.confirm_delete_message {
            let mut confirm = false;
            let mut cancel = false;
            egui::Window::new("Удаление личного сообщения")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Удалить это сообщение из ваших входящих?");
                    ui.horizontal(|ui| {
                        if ui.button("Удалить").clicked() {
                            confirm = true;
                        }
                        if ui.button("Отмена").clicked() {
                            cancel = true;
                        }
                    });
                });
            if confirm {
                self.delete_inbox_message(message_id);
            } else if cancel {
                self.confirm_delete_message = None;
            }
        }

        self.show_notes_window(ctx);
        self.show_radio_window(ctx);
        self.show_clear_chat_confirmation(ctx);
        #[cfg(feature = "voice")]
        self.show_voice_window(ctx);
    }
}
