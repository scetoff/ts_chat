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
    chat_refresh_pending: Option<std::sync::mpsc::Receiver<Result<String, String>>>,
    draft: String,
    selected_friend: Option<String>,
    private_subject: String,
    private_text: String,
    inbox: Vec<crate::models::PrivateMessage>,
    inbox_pending: Option<std::sync::mpsc::Receiver<Result<Vec<crate::models::PrivateMessage>, String>>>,
    inbox_loading: bool,
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
    radio_open: bool,
    radio_stations: Vec<RadioStation>,
    selected_radio_station: usize,
    radio_volume: f32,
    radio_muted: bool,
    radio_url_editor_open: bool,
    radio_status: String,
    radio_player: Option<RadioPlayback>,
    radio_pending: Option<(std::sync::mpsc::Receiver<Result<RadioPlayback, String>>, String)>,
    radio_animation: Option<AnimatedGif>,
    radio_window_pos: egui::Pos2,
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
    profile_expanded: bool,
    chat_style_open: bool,
    last_viewport_size: [f32; 2],
    ui_textures: std::collections::HashMap<String, egui::TextureHandle>,
    tavern_animation: Option<AnimatedGif>,
}

#[derive(Clone)]
struct AnimatedGif {
    frames: Vec<egui::TextureHandle>,
    frame_durations: Vec<Duration>,
    current_frame: usize,
    next_frame_at: Instant,
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

const WELCOME_CHAT: &str = "Добро пожаловать в Таверну Странника";

fn normalize_legacy_welcome(chat: &str) -> String {
    let placeholder: String = WELCOME_CHAT
        .chars()
        .map(|character| if character.is_ascii() { character } else { '?' })
        .collect();
    let trimmed = chat.trim();
    if let Some(rest) = trimmed.strip_prefix(&placeholder) {
        if rest.is_empty() || rest.starts_with('\n') {
            return format!("{WELCOME_CHAT}{rest}");
        }
    }
    chat.to_owned()
}

fn styled_rich_text<'a>(text: &'a str, style: Option<&ChatStyle>, dark_theme: bool) -> egui::RichText {
    let Some(style) = style else {
        return egui::RichText::new(text).size(15.0);
    };
    // Keep user-selected sizes, but make the legacy 16px default more readable.
    let display_size = (style.font_size + 2).clamp(10, 50);
    let mut rich = egui::RichText::new(text)
        .size(display_size as f32);
    if let Some(color) = parse_legacy_argb(&style.color) {
        let color = if dark_theme && u16::from(color.r()) + u16::from(color.g()) + u16::from(color.b()) < 180 {
            egui::Color32::WHITE
        } else {
            color
        };
        rich = rich.color(color);
    }
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
    dark_theme: bool,
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
                        ui.label(styled_rich_text(&rest[..position], author_style, dark_theme));
                    }
                    if let Some((_, texture)) = smileys.iter().find(|(candidate, _)| candidate == code) {
                        ui.add(egui::Image::new((texture.id(), egui::vec2(28.0, 28.0))));
                    } else {
                        ui.label(code);
                    }
                    rest = &rest[position + code.len()..];
                }
                None => {
                    if !rest.is_empty() { ui.label(styled_rich_text(rest, author_style, dark_theme)); }
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

fn legacy_animated_image(
    ui: &egui::Ui,
    animation: Option<&AnimatedGif>,
    rect: egui::Rect,
) {
    if let Some(animation) = animation {
        if let Some(texture) = animation.frames.get(animation.current_frame) {
            ui.painter().image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }
}

fn decode_gif_animation(ctx: &egui::Context, prefix: &str, bytes: &[u8]) -> Option<AnimatedGif> {
    use image::AnimationDecoder;
    let decoder = image::codecs::gif::GifDecoder::new(Cursor::new(bytes)).ok()?;
    let decoded = decoder.into_frames().collect_frames().ok()?;
    if decoded.is_empty() { return None; }

    let mut frames = Vec::with_capacity(decoded.len());
    let mut frame_durations = Vec::with_capacity(decoded.len());
    for (index, frame) in decoded.into_iter().enumerate() {
        let (numerator_ms, denominator_ms) = frame.delay().numer_denom_ms();
        let delay_ms = if numerator_ms == 0 || denominator_ms == 0 {
            100
        } else {
            (numerator_ms / denominator_ms).clamp(20, 1000)
        };
        let rgba = frame.into_buffer();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
        frames.push(ctx.load_texture(
            format!("{prefix}-{index}"),
            image,
            egui::TextureOptions::LINEAR,
        ));
        frame_durations.push(Duration::from_millis(delay_ms as u64));
    }
    let first_delay = frame_durations.first().copied().unwrap_or(Duration::from_millis(100));
    Some(AnimatedGif {
        frames,
        frame_durations,
        current_frame: 0,
        next_frame_at: Instant::now() + first_delay,
    })
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
            chat_refresh_pending: None,
            draft: String::new(), selected_friend: None, private_subject: String::new(),
            private_text: String::new(), inbox: Vec::new(),
            inbox_pending: None, inbox_loading: false,
            status: "Соединение проверяется при входе или регистрации.".into(),
            dark_theme: true, connected: false, show_inbox: false, private_message_open: false,
            confirm_delete_message: None,
            last_refresh: Instant::now(),
            profile_username: String::new(), profile_name: String::new(), profile_email: String::new(), profile_mobile: String::new(),
            avatar_texture: None, avatar_cache_key: None,
            friend_avatar_texture: None, friend_avatar_cache_key: None,
            new_password: String::new(), repeat_password: String::new(),
            friend_profile: None, show_friend_profile: false,
            radio_open: false,
            radio_window_pos: egui::pos2(100.0, 70.0),
            radio_stations,
            selected_radio_station: 0,
            radio_volume: 0.75,
            radio_muted: false,
            radio_url_editor_open: false,
            radio_status: "Выберите станцию и укажите её URL в настройках.".into(),
            radio_player: None,
            radio_pending: None,
            radio_animation: None,
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
            profile_expanded: true,
            chat_style_open: false,
            last_viewport_size: [590.0, 437.0],
            ui_textures: std::collections::HashMap::new(),
            tavern_animation: None,
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
                self.chat_text = normalize_legacy_welcome(&chat); self.screen = Screen::Chat; self.status.clear();
            }
            Ok(None) => self.status = "Проверьте никнейм и пароль.".into(),
            Err(e) => self.status = format!("Ошибка авторизации: {e:#}"),
        }
    }

    fn random_tavern_avatar_base64() -> anyhow::Result<String> {
        let sheet = image::load_from_memory(include_bytes!("../assets/tavern_avatar_sheet.png"))?.to_rgba8();
        let cell_width = sheet.width() / 5;
        let cell_height = sheet.height() / 2;
        anyhow::ensure!(cell_width > 0 && cell_height > 0, "Спрайт-лист аватаров имеет неверный размер");

        use std::hash::{BuildHasher, Hash, Hasher};
        let mut random = std::collections::hash_map::RandomState::new().build_hasher();
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
            .hash(&mut random);
        let index = (random.finish() % 10) as u32;
        let column = index % 5;
        let row = index / 5;
        let cell_size = cell_width.min(cell_height);
        let inset = (cell_size / 40).max(2);
        let crop_size = cell_size.saturating_sub(inset * 2);
        let x = column * cell_width + (cell_width - crop_size) / 2;
        let y = row * cell_height + (cell_height - crop_size) / 2;
        let cropped = image::imageops::crop_imm(&sheet, x, y, crop_size, crop_size).to_image();
        let mut avatar = image::imageops::resize(&cropped, 256, 256, image::imageops::FilterType::Lanczos3);

        let center = 127.5_f32;
        let radius_squared = 127.0_f32 * 127.0_f32;
        for y in 0..256 {
            for x in 0..256 {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                if dx * dx + dy * dy > radius_squared {
                    avatar.get_pixel_mut(x, y).0[3] = 0;
                }
            }
        }

        let mut png = Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(avatar).write_to(&mut png, image::ImageFormat::Png)?;
        use base64::Engine;
        Ok(base64::engine::general_purpose::STANDARD.encode(png.into_inner()))
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
        let avatar = match Self::random_tavern_avatar_base64() {
            Ok(avatar) => avatar,
            Err(error) => { self.status = format!("Не удалось подготовить аватарку: {error:#}"); return; }
        };
        match self.run_db(|rt, db| rt.block_on(db.register(&username, &hash, &email, &avatar))) {
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


    fn execute_command(&mut self, input: &str) {
        match parse_command(input) {
            ChatCommand::OpenRadio => {
                self.open_radio();
                self.command_status = "Открыто радио.".into();
            }
            ChatCommand::CloseRadio => {
                self.radio_open = false;
                self.radio_pending = None;
                if let Some(player) = self.radio_player.take() {
                    player.stop();
                }
                self.radio_status = "Окно радио закрыто; воспроизведение остановлено.".into();
                self.command_status = "Окно радио закрыто, воспроизведение остановлено.".into();
            }
            ChatCommand::PlayRadio => {
                self.open_radio();
                if let Some(player) = self.radio_player.as_ref() {
                    if player.is_paused() {
                        player.toggle_pause();
                        self.radio_status = "Воспроизведение продолжено.".into();
                    }
                    self.command_status = "Команда воспроизведения выполнена.".into();
                } else if self.radio_pending.is_some() || self.radio_autoplay_pending.is_some() {
                    self.command_status = "Подключение к настроенной радиостанции…".into();
                } else {
                    self.command_status = "В config.toml не задан URL радиостанции.".into();
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

    fn open_radio(&mut self) {
        self.radio_open = true;
        if self.radio_player.is_some() || self.radio_pending.is_some() || self.radio_autoplay_pending.is_some() {
            return;
        }

        let first_configured = self.radio_stations.iter().position(|station| {
            let Ok(url) = url::Url::parse(station.url.trim()) else { return false; };
            matches!(url.scheme(), "http" | "https") && url.host_str().is_some()
        });

        if let Some(index) = first_configured {
            self.selected_radio_station = index;
            self.radio_autoplay_pending = Some(index);
            self.radio_status = format!(
                "Подключение к настроенной станции «{}»…",
                self.radio_stations[index].name
            );
        } else {
            self.radio_status = "В config.toml не найдена радиостанция с корректным HTTP(S) URL.".into();
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
        self.ensure_radio_animation(ctx);
        self.advance_radio_animation(ctx);
        let radio_animation = self.radio_animation.clone();

        let mut open = self.radio_open;
        let mut drag_delta = egui::Vec2::ZERO;
        let mut close_requested = false;
        let mut minimize_requested = false;
        let mut start_request: Option<(String, String)> = self.radio_autoplay_pending
            .take()
            .and_then(|index| self.radio_stations.get(index).map(|station| (station.url.trim().to_string(), station.name.clone())));
        let mut stop_requested = false;
        let mut pause_requested = false;
        let mut edit_url_requested = false;
        let mut toggle_mute_requested = false;
        let paused = self.radio_player.as_ref().map(RadioPlayback::is_paused).unwrap_or(true);
        let player_present = self.radio_player.is_some();
        let displayed_radio_status = self.radio_status.clone();
        let connecting = self.radio_pending.is_some();
        let stations = &mut self.radio_stations;
        let selected = &mut self.selected_radio_station;
        let volume = &mut self.radio_volume;
        let textures = self.ui_textures.clone();
        let dark = self.dark_theme;
        let muted = self.radio_muted;

        egui::Window::new("Радио Онлайн")
            .id(egui::Id::new("legacy-radio-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([344.0, 512.0])
            .current_pos(self.radio_window_pos)
            .movable(false)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let origin = rect.min;
                ui.painter().rect_filled(rect, 0.0, egui::Color32::BLACK);

                ui.painter().rect_filled(
                    legacy_rect(origin, 0.0, 0.0, 344.0, 35.0),
                    0.0,
                    egui::Color32::from_rgb(0, 0, 128),
                );
                let drag_handle = ui.interact(
                    legacy_rect(origin, 36.0, 0.0, 220.0, 35.0),
                    egui::Id::new("legacy-radio-window-drag"),
                    egui::Sense::drag(),
                );
                if drag_handle.dragged() { drag_delta = ctx.input(|input| input.pointer.delta()); }
                legacy_static_image(ui, &textures, "chat_icon", legacy_rect(origin, 0.0, 0.0, 35.0, 35.0));
                if let Some(response) = legacy_image_button(
                    ui, &textures, "minimize", legacy_rect(origin, 262.0, 0.0, 35.0, 35.0),
                ) {
                    if response.clicked() { minimize_requested = true; }
                }
                if let Some(response) = legacy_image_button(
                    ui, &textures, "close", legacy_rect(origin, 303.0, 0.0, 35.0, 35.0),
                ) {
                    if response.clicked() { close_requested = true; }
                }

                let list_rect = legacy_rect(origin, 0.0, 36.0, 292.0, 416.0);
                ui.painter().rect_filled(list_rect, 0.0, egui::Color32::BLACK);
                ui.allocate_ui_at_rect(list_rect, |list_ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(416.0)
                        .show(list_ui, |list_ui| {
                            for (index, station) in stations.iter().enumerate() {
                                let response = list_ui.selectable_label(
                                    *selected == index,
                                    egui::RichText::new(&station.name)
                                        .size(14.0)
                                        .color(egui::Color32::WHITE),
                                );
                                if response.clicked() {
                                    *selected = index;
                                    if let Some(station) = stations.get(index) {
                                        start_request = Some((station.url.trim().to_string(), station.name.clone()));
                                    }
                                }
                            }
                        });
                });

                if let Some(response) = legacy_image_button(
                    ui, &textures, "radio_start", legacy_rect(origin, 298.0, 55.0, 40.0, 40.0),
                ) {
                    if response.clicked() {
                        if let Some(station) = stations.get(*selected) {
                            start_request = Some((station.url.trim().to_string(), station.name.clone()));
                        }
                    }
                }
                if let Some(response) = legacy_image_button(
                    ui, &textures, "radio_pause", legacy_rect(origin, 298.0, 120.0, 40.0, 40.0),
                ) {
                    if response.clicked() { pause_requested = true; }
                }
                if let Some(response) = legacy_image_button(
                    ui, &textures, "radio_stop", legacy_rect(origin, 298.0, 190.0, 40.0, 40.0),
                ) {
                    if response.clicked() { stop_requested = true; }
                }

                let edit_icon = if dark { "edit_dark" } else { "edit_light" };
                if let Some(response) = legacy_image_button(
                    ui, &textures, edit_icon, legacy_rect(origin, 298.0, 250.0, 40.0, 40.0),
                ) {
                    if response.clicked() { edit_url_requested = true; }
                }
                legacy_text(
                    ui,
                    legacy_rect(origin, 294.0, 296.0, 48.0, 48.0),
                    if player_present { "Играет" } else if connecting { "Связь…" } else { "URL" },
                    egui::Color32::WHITE,
                    9.0,
                );
                legacy_animated_image(
                    ui, radio_animation.as_ref(), legacy_rect(origin, 291.0, 402.0, 46.0, 50.0),
                );

                ui.painter().rect_filled(
                    legacy_rect(origin, 0.0, 463.0, 344.0, 49.0),
                    0.0,
                    egui::Color32::BLACK,
                );
                let status_color = if displayed_radio_status.starts_with("Не удалось")
                    || displayed_radio_status.starts_with("В config.toml")
                {
                    egui::Color32::from_rgb(255, 110, 110)
                } else {
                    egui::Color32::from_rgb(120, 220, 140)
                };
                legacy_text(
                    ui,
                    legacy_rect(origin, 2.0, 447.0, 288.0, 14.0),
                    displayed_radio_status.clone(),
                    status_color,
                    8.5,
                );

                let sound_icon = if muted { "radio_sound_off" } else { "radio_sound_on" };
                if let Some(response) = legacy_image_button(
                    ui, &textures, sound_icon, legacy_rect(origin, 26.0, 466.0, 40.0, 40.0),
                ) {
                    if response.clicked() { toggle_mute_requested = true; }
                }
                legacy_text(
                    ui,
                    legacy_rect(origin, 130.0, 475.0, 30.0, 16.0),
                    format!("{:.0}", *volume * 100.0),
                    egui::Color32::from_rgb(0, 255, 0),
                    10.0,
                );
                ui.put(
                    legacy_rect(origin, 166.0, 476.0, 173.0, 26.0),
                    egui::Slider::new(volume, 0.0..=1.0).show_value(false),
                );
            });

        if close_requested || minimize_requested {
            open = false;
        }
        self.radio_window_pos += drag_delta;
        self.radio_open = open && self.radio_open;
        if !self.radio_open {
            self.radio_pending = None;
            if let Some(player) = self.radio_player.take() {
                player.stop();
            }
            self.radio_status = "Окно радио закрыто; воспроизведение остановлено.".into();
        }

        if edit_url_requested {
            self.radio_url_editor_open = true;
        }
        if toggle_mute_requested {
            self.radio_muted = !self.radio_muted;
        }
        if let Some(player) = self.radio_player.as_ref() {
            player.set_volume(if self.radio_muted { 0.0 } else { self.radio_volume });
        }
        if pause_requested {
            if let Some(player) = self.radio_player.as_ref() {
                player.toggle_pause();
                self.radio_status = if player.is_paused() {
                    "Воспроизведение приостановлено.".into()
                } else {
                    "Воспроизведение продолжено.".into()
                };
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
                self.radio_status = "У этой станции не задан URL потока. Нажмите значок настройки адреса.".into();
                self.radio_url_editor_open = true;
            } else {
                self.radio_player = None;
                self.radio_pending = None;
                self.radio_status = format!("Подключение к станции «{station_name}»...");
                let (sender, receiver) = std::sync::mpsc::channel();
                let volume = if self.radio_muted { 0.0 } else { self.radio_volume };
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

        self.show_radio_url_editor_window(ctx);
    }

    fn show_radio_url_editor_window(&mut self, ctx: &egui::Context) {
        if !self.radio_url_editor_open {
            return;
        }
        let selected_index = self.selected_radio_station;
        let Some(station) = self.radio_stations.get(selected_index) else {
            self.radio_url_editor_open = false;
            return;
        };
        let station_name = station.name.clone();
        let mut url = station.url.clone();
        let mut open = self.radio_url_editor_open;
        let mut save_requested = false;
        egui::Window::new(format!("Адрес потока: {station_name}"))
            .default_size([420.0, 130.0])
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("URL потока (HTTP или HTTPS)");
                ui.text_edit_singleline(&mut url);
                if ui.button("Сохранить адрес").clicked() {
                    save_requested = true;
                }
            });
        if save_requested {
            if let Some(station) = self.radio_stations.get_mut(selected_index) {
                station.url = url;
            }
            self.save_radio_settings();
            self.radio_autoplay_pending = Some(selected_index);
            self.radio_open = true;
            open = false;
        }
        self.radio_url_editor_open = open;
    }


    fn refresh_chat(&mut self) {
        // Never block egui's UI thread while waiting for the shared, single-
        // connection MySQL pool. Inbox reads and the 10-second chat poll can
        // otherwise wait on each other and make the whole window appear frozen.
        if self.chat_refresh_pending.is_some() {
            return;
        }
        let Some(db) = self.db.clone() else {
            return;
        };

        let (sender, receiver) = std::sync::mpsc::channel();
        self.chat_refresh_pending = Some(receiver);
        self.runtime.spawn(async move {
            let result = db.read_chat().await.map_err(|error| format!("{error:#}"));
            let _ = sender.send(result);
        });
    }

    fn poll_chat_refresh(&mut self, ctx: &egui::Context) {
        let result = self.chat_refresh_pending.as_ref().map(|receiver| receiver.try_recv());
        match result {
            Some(Ok(Ok(chat))) => {
                self.chat_text = normalize_legacy_welcome(&chat);
                self.chat_refresh_pending = None;
            }
            Some(Ok(Err(error))) => {
                self.chat_refresh_pending = None;
                if self.status.is_empty() {
                    self.status = format!("Не удалось обновить общий чат: {error}");
                }
            }
            Some(Err(std::sync::mpsc::TryRecvError::Disconnected)) => {
                self.chat_refresh_pending = None;
                if self.status.is_empty() {
                    self.status = "Обновление общего чата прервано.".into();
                }
            }
            Some(Err(std::sync::mpsc::TryRecvError::Empty)) => {
                ctx.request_repaint_after(Duration::from_millis(150));
            }
            None => {}
        }
    }

    fn send_chat(&mut self) {
        let Some(user) = self.current_user.as_ref() else { return; };
        let message = self.draft.trim().to_string();
        if message.is_empty() { return; }
        let line = format!("{}: {}", user.username, message);
        let next = if self.chat_text.trim().is_empty() {
            line
        } else {
            format!("{}\n\n{}", self.chat_text.trim_end(), line)
        };
        match self.run_db(|rt, db| rt.block_on(db.write_chat(&next))) {
            Ok(()) => {
                self.chat_text = next;
                self.draft.clear();
                self.status = "Сообщение отправлено.".into();
            }
            Err(error) => self.status = format!("Ошибка отправки сообщения: {error:#}"),
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
        if self.inbox_loading {
            self.show_inbox = true;
            return;
        }
        let Some(user) = self.current_user.as_ref() else { return; };
        let Some(db) = self.db.clone() else {
            self.status = "Сначала подключитесь к базе данных.".into();
            return;
        };

        let name = user.username.clone();
        let (sender, receiver) = std::sync::mpsc::channel();
        self.inbox_pending = Some(receiver);
        self.inbox_loading = true;
        self.show_inbox = true;
        self.status = "Загрузка входящих сообщений…".into();

        self.runtime.spawn(async move {
            let result = db.inbox(&name).await.map_err(|error| format!("{error:#}"));
            let _ = sender.send(result);
        });
    }

    fn poll_inbox_load(&mut self, ctx: &egui::Context) {
        let result = self.inbox_pending.as_ref().map(|receiver| receiver.try_recv());
        match result {
            Some(Ok(Ok(items))) => {
                self.inbox = items;
                self.inbox_pending = None;
                self.inbox_loading = false;
                self.status.clear();
            }
            Some(Ok(Err(error))) => {
                self.inbox_pending = None;
                self.inbox_loading = false;
                self.status = format!("Ошибка загрузки входящих: {error}");
            }
            Some(Err(std::sync::mpsc::TryRecvError::Disconnected)) => {
                self.inbox_pending = None;
                self.inbox_loading = false;
                self.status = "Загрузка входящих прервана: задача базы данных завершилась неожиданно.".into();
            }
            Some(Err(std::sync::mpsc::TryRecvError::Empty)) => {
                ctx.request_repaint_after(Duration::from_millis(150));
            }
            None => {}
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

        let assets: [(&str, &[u8]); 27] = [
            ("brand_logo", include_bytes!("../Resources/32323.png")),
            ("theme_day", include_bytes!("../Resources/b_day.png")),
            ("theme_night", include_bytes!("../Resources/b_night.png")),
            ("chat_icon", include_bytes!("../assets/ICOCOS1.png")),
            ("radio_start", include_bytes!("../assets/start_b.png")),
            ("radio_pause", include_bytes!("../assets/pause_b.png")),
            ("radio_stop", include_bytes!("../assets/stop_b.png")),
            ("radio_sound_on", include_bytes!("../assets/sound_on.png")),
            ("radio_sound_off", include_bytes!("../assets/sound_off.png")),
            ("minimize", include_bytes!("../assets/remove.png")),
            ("close", include_bytes!("../assets/clear_search.png")),
            ("send_dark", include_bytes!("../assets/Send_mail.png")),
            ("send_light", include_bytes!("../assets/Send_mail_b.png")),
            ("attach_dark", include_bytes!("../assets/prikrep.png")),
            ("attach_light", include_bytes!("../assets/prikrep_b.png")),
            ("profile_dark", include_bytes!("../assets/profile.png")),
            ("profile_light", include_bytes!("../assets/profile_b.png")),
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

    fn ensure_tavern_animation(&mut self, ctx: &egui::Context) {
        if self.tavern_animation.is_none() {
            self.tavern_animation = decode_gif_animation(
                ctx,
                "tschat-tavern-gif-frame",
                include_bytes!("../Resources/222.gif"),
            );
        }
    }

    fn ensure_radio_animation(&mut self, ctx: &egui::Context) {
        if self.radio_animation.is_none() {
            self.radio_animation = decode_gif_animation(
                ctx,
                "tschat-radio-penguin-frame",
                include_bytes!("../assets/pingvin.gif"),
            );
        }
    }

    fn advance_tavern_animation(&mut self, ctx: &egui::Context) {
        let Some(animation) = self.tavern_animation.as_mut() else {
            return;
        };
        if animation.frames.len() <= 1 {
            return;
        }

        let now = Instant::now();
        if now >= animation.next_frame_at {
            animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
            let delay = animation.frame_durations[animation.current_frame];
            animation.next_frame_at = now + delay;
        }

        let remaining = animation.next_frame_at.saturating_duration_since(Instant::now());
        ctx.request_repaint_after(remaining.max(Duration::from_millis(1)));
    }

    fn advance_radio_animation(&mut self, ctx: &egui::Context) {
        let Some(animation) = self.radio_animation.as_mut() else {
            return;
        };
        if animation.frames.len() <= 1 {
            return;
        }

        let now = Instant::now();
        if now >= animation.next_frame_at {
            animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
            let delay = animation.frame_durations[animation.current_frame];
            animation.next_frame_at = now + delay;
        }

        let remaining = animation.next_frame_at.saturating_duration_since(Instant::now());
        ctx.request_repaint_after(remaining.max(Duration::from_millis(1)));
    }

    fn draw_legacy_auth_screen(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let dark = self.dark_theme;
        let bg = if dark { egui::Color32::from_rgb(18, 25, 23) } else { egui::Color32::from_rgb(242, 240, 233) };
        let card = if dark { egui::Color32::from_rgb(30, 39, 36) } else { egui::Color32::WHITE };
        let accent = egui::Color32::from_rgb(204, 157, 99);
        ui.painter().rect_filled(ui.max_rect(), 0.0, bg);

        ui.vertical(|ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_space(18.0);
                ui.label(egui::RichText::new("TSchat").size(19.0).strong().color(accent));
                ui.label(egui::RichText::new("Таверна странника").size(14.0).color(if dark { egui::Color32::from_rgb(205, 213, 207) } else { egui::Color32::from_rgb(70, 80, 74) }));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(12.0);
                    if ui.button("×").on_hover_text("Закрыть").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                    if ui.button("—").on_hover_text("Свернуть").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true)); }
                    if ui.button(if dark { "☀" } else { "☾" }).on_hover_text("Сменить тему").clicked() { self.dark_theme = !self.dark_theme; }
                });
                ui.add_space(10.0);
            });
            ui.add_space(18.0);
            ui.horizontal(|ui| {
                ui.add_space(24.0);
                egui::Frame::none().fill(card).rounding(egui::Rounding::same(14.0)).inner_margin(20.0).show(ui, |ui| {
                    ui.set_min_width(340.0);
                    ui.set_max_width(340.0);
                    ui.set_min_height(330.0);
                    ui.vertical_centered(|ui| {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(175.0, 145.0), egui::Sense::hover());
                        legacy_animated_image(ui, self.tavern_animation.as_ref(), rect);
                        ui.add_space(4.0);
                        ui.label(egui::RichText::new("Место для своих").size(23.0).strong().color(accent));
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("Небольшой локальный чат,
где всегда можно собраться.").size(14.0));
                    });
                });
                ui.add_space(18.0);
                egui::Frame::none().fill(card).rounding(egui::Rounding::same(14.0)).inner_margin(20.0).show(ui, |ui| {
                    ui.set_min_width(350.0);
                    ui.set_max_width(390.0);
                    ui.set_min_height(330.0);
                    ui.vertical(|ui| {
                        let registering = self.screen == Screen::Register;
                        ui.label(egui::RichText::new(if registering { "Создать аккаунт" } else { "С возвращением" }).size(24.0).strong());
                        ui.label(egui::RichText::new(if registering { "Присоединяйтесь к Таверне странника" } else { "Войдите, чтобы продолжить общение" }).size(13.0).color(if dark { egui::Color32::LIGHT_GRAY } else { egui::Color32::DARK_GRAY }));
                        ui.add_space(18.0);
                        ui.label("Имя пользователя");
                        ui.add(egui::TextEdit::singleline(&mut self.username).desired_width(f32::INFINITY).hint_text("Никнейм"));
                        ui.add_space(8.0);
                        ui.label(if registering { "Придумайте пароль" } else { "Пароль" });
                        ui.add(egui::TextEdit::singleline(&mut self.password).desired_width(f32::INFINITY).password(true).hint_text("Пароль"));
                        if registering {
                            ui.add_space(8.0);
                            ui.label("Электронная почта");
                            ui.add(egui::TextEdit::singleline(&mut self.email).desired_width(f32::INFINITY).hint_text("name@example.com"));
                        }
                        ui.add_space(14.0);
                        let primary = egui::Button::new(egui::RichText::new(if registering { "Зарегистрироваться" } else { "Войти" }).strong().color(egui::Color32::from_rgb(28, 32, 29))).fill(accent);
                        if ui.add_sized([ui.available_width(), 40.0], primary).clicked() {
                            if registering { self.register(); } else { self.login(); }
                        }
                        ui.add_space(8.0);
                        if ui.button(if registering { "Уже есть аккаунт? Войти" } else { "Создать аккаунт" }).clicked() {
                            self.screen = if registering { Screen::Login } else { Screen::Register };
                            self.status.clear();
                            self.email.clear();
                        }
                        if !self.status.is_empty() {
                            ui.add_space(8.0);
                            let color = if self.status.starts_with("Ошибка") || self.status.starts_with("Не удалось") || self.status.starts_with("Проверьте") { egui::Color32::from_rgb(225, 115, 105) } else { egui::Color32::from_rgb(130, 190, 145) };
                            ui.label(egui::RichText::new(self.status.clone()).color(color).size(12.0));
                        }
                    });
                });
                ui.add_space(24.0);
            });
        });
    }

    fn draw_legacy_chat_screen(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let dark = self.dark_theme;
        let bg = if dark { egui::Color32::from_rgb(18, 25, 23) } else { egui::Color32::from_rgb(242, 240, 233) };
        let surface = if dark { egui::Color32::from_rgb(30, 39, 36) } else { egui::Color32::WHITE };
        let inset = if dark { egui::Color32::from_rgb(23, 31, 28) } else { egui::Color32::from_rgb(248, 247, 243) };
        let accent = egui::Color32::from_rgb(204, 157, 99);
        let secondary = if dark { egui::Color32::from_rgb(154, 169, 160) } else { egui::Color32::from_rgb(99, 110, 103) };
        ui.painter().rect_filled(ui.max_rect(), 0.0, bg);

        ui.vertical(|ui| {
            ui.add_space(7.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(egui::RichText::new("TSchat").size(18.0).strong().color(accent));
                ui.label(egui::RichText::new("Таверна странника").size(13.0).color(secondary));
                ui.separator();
                ui.label(egui::RichText::new(if self.connected { "● В сети" } else { "● Не подключено" }).size(12.0).color(if self.connected { egui::Color32::from_rgb(115, 190, 137) } else { secondary }));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(14.0);
                    if ui.button("×").on_hover_text("Закрыть").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                    if ui.button("—").on_hover_text("Свернуть").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true)); }
                    if ui.button(if dark { "☀" } else { "☾" }).on_hover_text("Сменить тему").clicked() { self.dark_theme = !self.dark_theme; }
                    if ui.button("♫").on_hover_text("Радио").clicked() { self.open_radio(); }
                    if ui.button("✉").on_hover_text("Личные сообщения").clicked() { self.load_inbox(); }
                    if ui.button("Aa").on_hover_text("Оформление").clicked() { self.chat_style_open = true; }
                    if ui.button("⌫").on_hover_text("Очистить чат").clicked() { self.show_clear_chat_confirmation = true; }
                });
                ui.add_space(8.0);
            });
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(7.0);

            let available_width = ui.available_width();
            let nav_width = 58.0;
            let chats_width = 224.0;
            let details_width = if self.profile_expanded { 226.0 } else { 0.0 };
            let gaps = 8.0 * if self.profile_expanded { 3.0 } else { 2.0 };
            let main_width = (available_width - nav_width - chats_width - details_width - gaps).max(420.0);
            let body_height = (ui.available_height() - 16.0).max(480.0);
            ui.horizontal(|ui| {
                egui::Frame::none().fill(surface).rounding(egui::Rounding::same(14.0)).inner_margin(8.0).show(ui, |nav| {
                    nav.set_min_width(nav_width);
                    nav.set_max_width(nav_width);
                    nav.set_min_height(body_height);
                    nav.vertical_centered(|nav| {
                        nav.add_space(10.0);
                        if nav.button("▣").on_hover_text("Чаты").clicked() { self.selected_friend = None; }
                        if nav.button("♙").on_hover_text("Собеседники").clicked() { self.profile_expanded = true; }
                        if nav.button("♫").on_hover_text("Радио").clicked() { self.open_radio(); }
                        if nav.button("⚙").on_hover_text("Профиль").clicked() { self.edit_profile_open = true; }
                        nav.with_layout(egui::Layout::bottom_up(egui::Align::Center), |nav| {
                            if let Some(texture) = &self.avatar_texture {
                                if nav.add(egui::Image::new((texture.id(), egui::vec2(36.0, 36.0))).sense(egui::Sense::click())).clicked() {
                                    self.edit_profile_open = true;
                                }
                            }
                        });
                    });
                });

                ui.add_space(8.0);
                egui::Frame::none().fill(surface).rounding(egui::Rounding::same(14.0)).inner_margin(12.0).show(ui, |list| {
                    list.set_min_width(chats_width);
                    list.set_max_width(chats_width);
                    list.set_min_height(body_height);
                    list.vertical(|list| {
                        list.horizontal(|list| {
                            list.label(egui::RichText::new("Чаты").size(18.0).strong());
                            list.with_layout(egui::Layout::right_to_left(egui::Align::Center), |list| {
                                if list.small_button("+").on_hover_text("Новый чат").clicked() {
                                    self.status = "Личные сообщения доступны через значок конверта.".into();
                                }
                            });
                        });
                        list.add_space(8.0);
                        list.add(egui::TextEdit::singleline(&mut self.command_input).desired_width(f32::INFINITY).hint_text("Поиск"));
                        let selected = self.selected_friend.is_none();
                        let chat_item = egui::Button::new(egui::RichText::new("◉  Общий чат").strong().color(if selected { accent } else { secondary }))
                            .fill(if selected { inset } else { surface });
                        let chat_item_width = list.available_width();
                        if list.add_sized([chat_item_width, 42.0], chat_item).clicked() {
                            self.selected_friend = None;
                        }
                        list.label(egui::RichText::new("УЧАСТНИКИ").size(10.0).color(secondary).strong());
                        egui::ScrollArea::vertical().auto_shrink([false, false]).show(list, |people| {
                            for name in self.friends.clone() {
                                let is_selected = self.selected_friend.as_deref() == Some(name.as_str());
                                let response = people.selectable_label(is_selected, format!("◦  {name}"));
                                if response.clicked() {
                                    self.selected_friend = Some(name.clone());
                                    self.load_selected_profile();
                                }
                            }
                        });
                    });
                });

                ui.add_space(8.0);
                egui::Frame::none().fill(surface).rounding(egui::Rounding::same(14.0)).inner_margin(14.0).show(ui, |chat| {
                    chat.set_min_width(main_width);
                    chat.set_max_width(main_width);
                    chat.set_min_height(body_height);
                    chat.vertical(|chat| {
                        chat.horizontal(|chat| {
                            let title = "Общий чат";
                            chat.label(egui::RichText::new(title).size(19.0).strong());
                            chat.label(egui::RichText::new(format!("{} участника", self.friends.len() + 1)).size(11.0).color(secondary));
                            chat.with_layout(egui::Layout::right_to_left(egui::Align::Center), |chat| {
                                if chat.button("•••").on_hover_text("Действия").clicked() { self.show_clear_chat_confirmation = true; }
                                if chat.button("⌕").on_hover_text("Найти в чате").clicked() { self.status = "Поиск по истории скоро появится.".into(); }
                            });
                        });
                        chat.separator();
                        let smileys = self.smiley_textures.clone();
                        let styles = self.chat_styles.clone();
                        egui::Frame::none().fill(inset).rounding(egui::Rounding::same(12.0)).inner_margin(12.0).show(chat, |content| {
                            content.set_min_height((content.available_height() - 100.0).max(260.0));
                            egui::ScrollArea::vertical().stick_to_bottom(true).auto_shrink([false, false]).show(content, |content| {
                                for line in self.chat_text.lines() {
                                    show_chat_line(content, line, &smileys, &styles, dark);
                                    content.add_space(7.0);
                                }
                            });
                        });
                        chat.add_space(10.0);
                        chat.horizontal(|chat| {
                            if chat.button("☺").on_hover_text("Смайлики").clicked() { self.show_smileys = !self.show_smileys; }
                            let input_width = (chat.available_width() - 100.0).max(180.0);
                            chat.add(egui::TextEdit::multiline(&mut self.draft).desired_rows(2).desired_width(input_width).hint_text("Сообщение…"));
                            if chat.add_sized([78.0, 48.0], egui::Button::new(egui::RichText::new("➤").size(20.0).strong().color(egui::Color32::from_rgb(28, 32, 29))).fill(accent)).clicked() {
                                self.send_chat();
                            }
                        });
                        if self.show_smileys {
                            chat.horizontal_wrapped(|chat| {
                                for (code, texture) in self.smiley_textures.clone() {
                                    if chat.add(egui::Image::new((texture.id(), egui::vec2(28.0, 28.0))).sense(egui::Sense::click())).clicked() {
                                        self.add_smiley(&code);
                                        self.show_smileys = false;
                                    }
                                }
                            });
                        }
                        if !self.status.is_empty() {
                            let color = if self.status.starts_with("Ошибка") || self.status.starts_with("Не удалось") { egui::Color32::from_rgb(225, 115, 105) } else { secondary };
                            chat.label(egui::RichText::new(self.status.clone()).size(10.0).color(color));
                        }
                    });
                });

                if self.profile_expanded {
                    ui.add_space(8.0);
                    egui::Frame::none().fill(surface).rounding(egui::Rounding::same(14.0)).inner_margin(14.0).show(ui, |details| {
                        details.set_min_width(details_width);
                        details.set_max_width(details_width);
                        details.set_min_height(body_height);
                        details.vertical(|details| {
                            details.label(egui::RichText::new("Участник").size(17.0).strong());
                            details.add_space(12.0);
                            if let Some(texture) = &self.avatar_texture {
                                details.add(egui::Image::new((texture.id(), egui::vec2(72.0, 72.0))));
                            }
                            if let Some(user) = &self.current_user {
                                details.label(egui::RichText::new(&user.username).size(15.0).strong());
                                details.label(egui::RichText::new(if self.connected { "● В сети" } else { "● Не в сети" }).size(11.0).color(egui::Color32::from_rgb(115, 190, 137)));
                            }
                            details.add_space(10.0);
                            if details.button("Профиль").clicked() { self.edit_profile_open = true; }
                            details.separator();
                            details.label(egui::RichText::new("Собеседники").strong());
                            egui::ScrollArea::vertical().max_height(280.0).show(details, |people| {
                                for name in self.friends.clone() {
                                    if people.selectable_label(self.selected_friend.as_deref() == Some(name.as_str()), &name).clicked() {
                                        self.selected_friend = Some(name.clone());
                                        self.load_selected_profile();
                                    }
                                }
                            });
                        });
                    });
                }
            });
        });
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
        let mut save_requested = false;

        egui::Window::new("Оформление сообщений")
            .id(egui::Id::new("legacy-chat-style-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([420.0, 292.0])
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let origin = rect.min;
                let bg = if self.dark_theme { egui::Color32::BLACK } else { egui::Color32::WHITE };
                let fg = if self.dark_theme { egui::Color32::WHITE } else { egui::Color32::BLACK };
                let border = if self.dark_theme { egui::Color32::DARK_GRAY } else { egui::Color32::GRAY };
                ui.painter().rect_filled(rect, 0.0, bg);
                ui.painter().rect_filled(
                    legacy_rect(origin, 0.0, 0.0, 420.0, 35.0),
                    0.0,
                    egui::Color32::from_rgb(0, 0, 128),
                );
                legacy_static_image(ui, &self.ui_textures, "chat_icon", legacy_rect(origin, 0.0, 0.0, 35.0, 35.0));
                if let Some(response) = legacy_image_button(
                    ui,
                    &self.ui_textures,
                    "close",
                    legacy_rect(origin, 379.0, 0.0, 35.0, 35.0),
                ) {
                    if response.clicked() { close_requested = true; }
                }
                legacy_text(ui, legacy_rect(origin, 48.0, 5.0, 300.0, 25.0), "Оформление сообщений", egui::Color32::WHITE, 16.0);

                legacy_group(ui, legacy_rect(origin, 18.0, 47.0, 384.0, 178.0), bg, border);

                legacy_text(ui, legacy_rect(origin, 32.0, 59.0, 112.0, 24.0), "Цвет текста", fg, 13.0);
                ui.allocate_ui_at_rect(legacy_rect(origin, 160.0, 56.0, 70.0, 28.0), |color_ui| {
                    color_ui.color_edit_button_srgba(&mut self.chat_style_color);
                });

                legacy_text(ui, legacy_rect(origin, 32.0, 96.0, 112.0, 24.0), "Шрифт", fg, 13.0);
                ui.allocate_ui_at_rect(legacy_rect(origin, 160.0, 90.0, 220.0, 28.0), |font_ui| {
                    egui::ComboBox::from_id_salt("legacy-chat-font")
                        .width(210.0)
                        .selected_text(&self.chat_style_font)
                        .show_ui(font_ui, |ui| {
                            for font in ["Candara", "Arial", "Tahoma", "Verdana", "Consolas"] {
                                ui.selectable_value(&mut self.chat_style_font, font.to_string(), font);
                            }
                        });
                });

                legacy_text(
                    ui,
                    legacy_rect(origin, 32.0, 135.0, 112.0, 24.0),
                    format!("Размер: {} pt", self.chat_style_size),
                    fg,
                    13.0,
                );
                ui.allocate_ui_at_rect(legacy_rect(origin, 160.0, 130.0, 220.0, 30.0), |size_ui| {
                    size_ui.add(egui::Slider::new(&mut self.chat_style_size, 10..=36).show_value(false));
                });

                legacy_text(ui, legacy_rect(origin, 32.0, 176.0, 112.0, 24.0), "Начертание", fg, 13.0);
                ui.allocate_ui_at_rect(legacy_rect(origin, 160.0, 169.0, 220.0, 28.0), |style_ui| {
                    egui::ComboBox::from_id_salt("legacy-chat-style")
                        .width(210.0)
                        .selected_text(&self.chat_style_effect)
                        .show_ui(style_ui, |ui| {
                            for effect in ["Regular", "Bold", "Italic", "Underline", "Strikeout"] {
                                ui.selectable_value(&mut self.chat_style_effect, effect.to_string(), effect);
                            }
                        });
                });

                legacy_text(ui, legacy_rect(origin, 32.0, 235.0, 110.0, 22.0), "Пример", fg, 13.0);
                let preview_style = ChatStyle {
                    color: legacy_color_hex(self.chat_style_color),
                    font: self.chat_style_font.clone(),
                    font_size: self.chat_style_size,
                    font_style: self.chat_style_effect.clone(),
                };
                let preview_rect = legacy_rect(origin, 150.0, 229.0, 230.0, 30.0);
                legacy_group(ui, preview_rect, bg, border);
                ui.put(
                    legacy_rect(origin, 158.0, 231.0, 216.0, 26.0),
                    egui::Label::new(styled_rich_text("Привет! Это пример сообщения.", Some(&preview_style), self.dark_theme)),
                );

                if legacy_button(
                    ui,
                    legacy_rect(origin, 215.0, 263.0, 165.0, 25.0),
                    "Сохранить",
                    egui::Color32::BLACK,
                    egui::Color32::WHITE,
                    14.0,
                ).clicked() {
                    save_requested = true;
                }
            });

        if close_requested { open = false; }
        self.chat_style_open = open && self.chat_style_open;
        if save_requested {
            self.save_chat_style();
            self.chat_style_open = false;
        }
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
        self.poll_chat_refresh(ctx);
        self.poll_inbox_load(ctx);

        if self.screen == Screen::Chat {
            ctx.request_repaint_after(Duration::from_secs(10));
            if self.last_refresh.elapsed() >= Duration::from_secs(10) {
                self.refresh_chat();
                self.last_refresh = Instant::now();
            }
        }

        let target_size = match self.screen {
            Screen::Login | Screen::Register => [920.0, 620.0],
            Screen::Chat if self.profile_expanded => [1440.0, 900.0],
            Screen::Chat => [1180.0, 820.0],
        };
        if self.last_viewport_size != target_size {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(target_size[0], target_size[1])));
            ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(true));
            self.last_viewport_size = target_size;
        }

        self.ensure_avatar_texture(ctx);
        self.ensure_smiley_textures(ctx);
        self.ensure_legacy_ui_textures(ctx);
        if matches!(self.screen, Screen::Login | Screen::Register) {
            self.ensure_tavern_animation(ctx);
            self.advance_tavern_animation(ctx);
        }
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
                if ui.button("Обновить").clicked() {
                    self.load_inbox();
                }
                if ui.button("Закрыть").clicked() {
                    close_inbox = true;
                }
                if self.inbox_loading && inbox_items.is_empty() {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Загружаем входящие сообщения…");
                    });
                } else if inbox_items.is_empty() {
                    ui.label("Входящих сообщений пока нет.");
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

        self.show_radio_window(ctx);
        self.show_clear_chat_confirmation(ctx);
        #[cfg(feature = "voice")]
        self.show_voice_window(ctx);
    }
}
