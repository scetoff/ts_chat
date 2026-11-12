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
    radio_cover_textures: Vec<egui::TextureHandle>,
    radio_favorites: std::collections::HashSet<usize>,
    smiley_textures: Vec<(String, egui::TextureHandle)>,
    selected_sticker_pack: usize,
    window_positions: std::collections::HashMap<&'static str, egui::Pos2>,
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
    #[cfg(feature = "voice")]
    voice_passive_enabled: bool,
    #[cfg(feature = "voice")]
    voice_last_command: Option<(String, Instant)>,
    show_smileys: bool,
    edit_profile_open: bool,
    profile_expanded: bool,
    chat_style_open: bool,
    quick_settings_open: bool,
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
        return egui::RichText::new(text).size(16.0);
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

const SMILEY_CODES: [&str; 32] = [
    ":fuu:", ":beer:", ":sorry:", ":read:", ":girl:", ":lich:", ":sing:", ":haha:",
    ":allbear:", ":say:", ":money:", ":no:", ":king:", ":go:", ":good:", ":nice:",
    ":tavern_keeper_01:", ":tavern_keeper_02:", ":tavern_keeper_03:", ":tavern_keeper_04:",
    ":tavern_keeper_05:", ":tavern_keeper_06:", ":tavern_keeper_07:", ":tavern_keeper_08:",
    ":tavern_keeper_09:", ":tavern_keeper_10:", ":tavern_keeper_11:", ":tavern_keeper_12:",
    ":tavern_keeper_13:", ":tavern_keeper_14:", ":tavern_keeper_15:", ":tavern_keeper_16:",
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



fn paint_sketch_icon(painter: &egui::Painter, rect: egui::Rect, icon: &str, color: egui::Color32) {
    let c = rect.center();
    let stroke = egui::Stroke::new(1.8, color);
    let line = |a, b| painter.line_segment([a, b], stroke);
    match icon {
        "chat" => {
            let r = egui::Rect::from_center_size(c + egui::vec2(0.0, -1.0), egui::vec2(rect.width() * 0.66, rect.height() * 0.52));
            painter.rect_stroke(r, 5.0, stroke);
            line(egui::pos2(r.left() + 5.0, r.bottom() - 1.0), egui::pos2(r.left() + 1.0, r.bottom() + 5.0));
            line(egui::pos2(r.left() + 1.0, r.bottom() + 5.0), egui::pos2(r.left() + 10.0, r.bottom() - 1.0));
        }
        "users" => {
            painter.circle_stroke(c + egui::vec2(0.0, -5.0), 4.0, stroke);
            painter.circle_stroke(c + egui::vec2(-9.0, -3.0), 3.0, stroke);
            painter.circle_stroke(c + egui::vec2(9.0, -3.0), 3.0, stroke);
            line(c + egui::vec2(-7.0, 8.0), c + egui::vec2(-7.0, 5.0));
            painter.add(egui::Shape::line(vec![c + egui::vec2(-7.0, 5.0), c + egui::vec2(-4.0, 1.0), c + egui::vec2(4.0, 1.0), c + egui::vec2(7.0, 5.0), c + egui::vec2(7.0, 8.0)], stroke));
            painter.add(egui::Shape::line(vec![c + egui::vec2(-14.0, 7.0), c + egui::vec2(-13.0, 3.0), c + egui::vec2(-10.0, 1.0)], stroke));
            painter.add(egui::Shape::line(vec![c + egui::vec2(10.0, 1.0), c + egui::vec2(13.0, 3.0), c + egui::vec2(14.0, 7.0)], stroke));
        }
        "music" => {
            line(c + egui::vec2(1.0, -10.0), c + egui::vec2(1.0, 5.0));
            line(c + egui::vec2(1.0, -10.0), c + egui::vec2(9.0, -12.0));
            line(c + egui::vec2(9.0, -12.0), c + egui::vec2(9.0, 3.0));
            painter.circle_filled(c + egui::vec2(-3.0, 6.0), 3.5, color);
            painter.circle_filled(c + egui::vec2(5.0, 4.0), 3.5, color);
        }
        "settings" => {
            painter.circle_stroke(c, 6.0, stroke);
            painter.circle_stroke(c, 2.0, stroke);
            for i in 0..8 {
                let a = i as f32 * std::f32::consts::TAU / 8.0;
                line(c + egui::vec2(a.cos() * 8.0, a.sin() * 8.0), c + egui::vec2(a.cos() * 12.0, a.sin() * 12.0));
            }
        }
        "bell" => {
            painter.add(egui::Shape::line(vec![c + egui::vec2(-9.0, 6.0), c + egui::vec2(-6.0, 2.0), c + egui::vec2(-5.0, -5.0), c, c + egui::vec2(5.0, -5.0), c + egui::vec2(6.0, 2.0), c + egui::vec2(9.0, 6.0)], stroke));
            line(c + egui::vec2(-9.0, 6.0), c + egui::vec2(9.0, 6.0));
            painter.circle_filled(c + egui::vec2(0.0, 9.0), 1.7, color);
        }
        "profile" => {
            painter.circle_stroke(c, 8.5, stroke);
            painter.circle_filled(c + egui::vec2(0.0, -3.0), 1.2, color);
            line(c + egui::vec2(0.0, 0.0), c + egui::vec2(0.0, 5.0));
        }
        "file" => {
            let r = egui::Rect::from_center_size(c, egui::vec2(14.0, 17.0));
            painter.rect_stroke(r, 2.0, stroke);
            line(egui::pos2(r.left() + 3.0, c.y - 2.0), egui::pos2(r.right() - 3.0, c.y - 2.0));
            line(egui::pos2(r.left() + 3.0, c.y + 3.0), egui::pos2(r.right() - 3.0, c.y + 3.0));
        }
        "link" => {
            painter.add(egui::Shape::line(vec![c + egui::vec2(-1.0, 5.0), c + egui::vec2(-4.0, 8.0), c + egui::vec2(-8.0, 8.0), c + egui::vec2(-10.0, 6.0), c + egui::vec2(-10.0, 3.0), c + egui::vec2(-5.0, -2.0)], stroke));
            painter.add(egui::Shape::line(vec![c + egui::vec2(1.0, -5.0), c + egui::vec2(4.0, -8.0), c + egui::vec2(8.0, -8.0), c + egui::vec2(10.0, -6.0), c + egui::vec2(10.0, -3.0), c + egui::vec2(5.0, 2.0)], stroke));
            line(c + egui::vec2(-5.0, 5.0), c + egui::vec2(5.0, -5.0));
        }
        "phone" => {
            painter.add(egui::Shape::line(vec![
                c + egui::vec2(-8.0, -9.0), c + egui::vec2(-10.0, -6.0),
                c + egui::vec2(-8.0, 0.0), c + egui::vec2(-3.0, 6.0),
                c + egui::vec2(3.0, 9.0), c + egui::vec2(7.0, 8.0),
                c + egui::vec2(9.0, 5.0), c + egui::vec2(5.0, 1.0),
                c + egui::vec2(2.0, 3.0), c + egui::vec2(-2.0, 0.0),
                c + egui::vec2(-4.0, -4.0), c + egui::vec2(-2.0, -7.0),
                c + egui::vec2(-6.0, -10.0), c + egui::vec2(-8.0, -9.0),
            ], stroke));
        }
        "search" => {
            painter.circle_stroke(c + egui::vec2(-2.0, -2.0), 6.0, stroke);
            line(c + egui::vec2(2.0, 2.0), c + egui::vec2(9.0, 9.0));
        }
        "more" => for x in [-7.0, 0.0, 7.0] { painter.circle_filled(c + egui::vec2(x, 0.0), 1.8, color); },
        "close" => {
            line(c + egui::vec2(-5.0, -5.0), c + egui::vec2(5.0, 5.0));
            line(c + egui::vec2(5.0, -5.0), c + egui::vec2(-5.0, 5.0));
        }
        "send" => {
            painter.add(egui::Shape::closed_line(vec![c + egui::vec2(-9.0, -7.0), c + egui::vec2(10.0, 0.0), c + egui::vec2(-9.0, 7.0), c + egui::vec2(-5.0, 0.0)], stroke));
            line(c + egui::vec2(-5.0, 0.0), c + egui::vec2(10.0, 0.0));
        }
        "smile" => {
            painter.circle_stroke(c, 10.0, stroke);
            painter.circle_filled(c + egui::vec2(-3.5, -3.0), 1.2, color);
            painter.circle_filled(c + egui::vec2(3.5, -3.0), 1.2, color);
            painter.add(egui::Shape::line(vec![c + egui::vec2(-4.0, 2.0), c + egui::vec2(-2.0, 4.0), c, c + egui::vec2(2.0, 4.0), c + egui::vec2(4.0, 2.0)], stroke));
        }
        "attach" => {
            painter.add(egui::Shape::line(vec![c + egui::vec2(3.0, -9.0), c + egui::vec2(-5.0, -1.0), c + egui::vec2(-5.0, 5.0), c + egui::vec2(-2.0, 8.0), c + egui::vec2(1.0, 8.0), c + egui::vec2(8.0, 1.0), c + egui::vec2(8.0, -2.0), c + egui::vec2(5.0, -5.0), c + egui::vec2(2.0, -5.0), c + egui::vec2(-3.0, 0.0)], stroke));
        }
        "minimize" => { line(c + egui::vec2(-6.0, 0.0), c + egui::vec2(6.0, 0.0)); }
        "maximize" => { painter.rect_stroke(egui::Rect::from_center_size(c, egui::vec2(11.0, 11.0)), 1.0, stroke); }
        _ => { painter.circle_stroke(c, 7.0, stroke); }
    }
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
    painter.rect_filled(rect, 10.0, fill);
    painter.rect_stroke(rect, 10.0, egui::Stroke::new(1.0, border));
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
            radio_favorites: std::collections::HashSet::new(),
            radio_stations,
            selected_radio_station: 0,
            radio_volume: 0.75,
            radio_muted: false,
            radio_url_editor_open: false,
            radio_status: "Выберите станцию и укажите её URL в настройках.".into(),
            radio_player: None,
            radio_pending: None,
            radio_animation: None,
            radio_cover_textures: Vec::new(),
            smiley_textures: Vec::new(),
            selected_sticker_pack: 0,
            window_positions: std::collections::HashMap::new(),
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
            #[cfg(feature = "voice")]
            voice_passive_enabled: false,
            #[cfg(feature = "voice")]
            voice_last_command: None,
            show_smileys: false,
            edit_profile_open: false,
            profile_expanded: true,
            chat_style_open: false,
            quick_settings_open: false,
            last_viewport_size: [460.0, 467.0],
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
            Ok(Some((mut user, stored_hash))) => {
                if !Self::verify_password(&password, &stored_hash) {
                    self.status = "Проверьте никнейм и пароль.".into();
                    return;
                }

                // Backfill legacy accounts once. New registrations already get
                // a random bundled portrait, and users can replace it below.
                let missing_avatars = self.run_db(|rt, db| rt.block_on(db.users_missing_avatar()))
                    .unwrap_or_default();
                if !missing_avatars.is_empty() {
                    if let Ok(avatar_pool) = Self::tavern_avatar_pool_base64() {
                        for name in missing_avatars {
                            let avatar = avatar_pool[Self::avatar_index(&name)].clone();
                            if self.run_db(|rt, db| rt.block_on(db.update_avatar(&name, &avatar))).is_ok()
                                && name == user.username
                            {
                                user.avatar_base64 = avatar;
                            }
                        }
                    }
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

    fn tavern_avatar_pool_base64() -> anyhow::Result<Vec<String>> {
        let sheet = image::load_from_memory(include_bytes!("../assets/tavern_avatar_sheet.png"))?.to_rgba8();
        let cell_width = sheet.width() / 5;
        let cell_height = sheet.height() / 2;
        anyhow::ensure!(cell_width > 0 && cell_height > 0, "Спрайт-лист аватаров имеет неверный размер");

        let cell_size = cell_width.min(cell_height);
        let inset = (cell_size / 40).max(2);
        let crop_size = cell_size.saturating_sub(inset * 2);
        let mut avatars = Vec::with_capacity(10);
        for index in 0..10_u32 {
            let column = index % 5;
            let row = index / 5;
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
            avatars.push(base64::engine::general_purpose::STANDARD.encode(png.into_inner()));
        }
        Ok(avatars)
    }

    fn avatar_index(seed: &str) -> usize {
        use std::hash::{BuildHasher, Hash, Hasher};
        let mut random = std::collections::hash_map::RandomState::new().build_hasher();
        seed.hash(&mut random);
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
            .hash(&mut random);
        (random.finish() % 10) as usize
    }

    fn random_tavern_avatar_base64() -> anyhow::Result<String> {
        let avatars = Self::tavern_avatar_pool_base64()?;
        Ok(avatars[Self::avatar_index("")].clone())
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
            let source = image::load_from_memory(&bytes)?.to_rgba8();
            let side = source.width().min(source.height());
            anyhow::ensure!(side > 0, "Изображение пустое");
            let x = (source.width() - side) / 2;
            let y = (source.height() - side) / 2;
            let cropped = image::imageops::crop_imm(&source, x, y, side, side).to_image();
            let mut resized = image::imageops::resize(&cropped, 256, 256, image::imageops::FilterType::Lanczos3);
            for y in 0..256 {
                for x in 0..256 {
                    let dx = x as f32 - 127.5;
                    let dy = y as f32 - 127.5;
                    if dx * dx + dy * dy > 127.0 * 127.0 {
                        resized.get_pixel_mut(x, y).0[3] = 0;
                    }
                }
            }
            let mut png = Cursor::new(Vec::new());
            image::DynamicImage::ImageRgba8(resized).write_to(&mut png, image::ImageFormat::Png)?;
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
        let mut assets: Vec<(&str, &[u8])> = vec![
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
        assets.extend([
            (":tavern_keeper_01:", &include_bytes!("../assets/stickers/bartender_male/01.png")[..]),
            (":tavern_keeper_02:", &include_bytes!("../assets/stickers/bartender_male/02.png")[..]),
            (":tavern_keeper_03:", &include_bytes!("../assets/stickers/bartender_male/03.png")[..]),
            (":tavern_keeper_04:", &include_bytes!("../assets/stickers/bartender_male/04.png")[..]),
            (":tavern_keeper_05:", &include_bytes!("../assets/stickers/bartender_male/05.png")[..]),
            (":tavern_keeper_06:", &include_bytes!("../assets/stickers/bartender_male/06.png")[..]),
            (":tavern_keeper_07:", &include_bytes!("../assets/stickers/bartender_male/07.png")[..]),
            (":tavern_keeper_08:", &include_bytes!("../assets/stickers/bartender_male/08.png")[..]),
            (":tavern_keeper_09:", &include_bytes!("../assets/stickers/bartender_male/09.png")[..]),
            (":tavern_keeper_10:", &include_bytes!("../assets/stickers/bartender_male/10.png")[..]),
            (":tavern_keeper_11:", &include_bytes!("../assets/stickers/bartender_male/11.png")[..]),
            (":tavern_keeper_12:", &include_bytes!("../assets/stickers/bartender_male/12.png")[..]),
            (":tavern_keeper_13:", &include_bytes!("../assets/stickers/bartender_male/13.png")[..]),
            (":tavern_keeper_14:", &include_bytes!("../assets/stickers/bartender_male/14.png")[..]),
            (":tavern_keeper_15:", &include_bytes!("../assets/stickers/bartender_male/15.png")[..]),
            (":tavern_keeper_16:", &include_bytes!("../assets/stickers/bartender_male/16.png")[..]),
        ]);

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
        // Passive mode processes short chunks to keep Whisper responsive.
        let max_recording_secs = if self.voice_passive_enabled { 6 } else { 12 };
        if self.voice_recording_started
            .map(|started| started.elapsed() >= Duration::from_secs(max_recording_secs))
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
                    self.voice_status = "Whisper-модель загружена. Микрофон включается только по вашему запросу.".into();
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
                    let command = parse_command(&text);
                    if command != ChatCommand::Unknown {
                        let duplicate = self.voice_passive_enabled && self.voice_last_command.as_ref().is_some_and(|(previous, at)| {
                            previous == &text && at.elapsed() < Duration::from_secs(8)
                        });
                        if !duplicate {
                            self.execute_command(&text);
                            self.voice_last_command = Some((text.clone(), Instant::now()));
                        }
                        self.voice_status = format!("Команда распознана: {text}");
                    } else {
                        self.voice_status = format!("Распознано, но команды нет: {text}");
                    }
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

        if !self.voice_open {
            if self.voice_passive_enabled || self.voice_recorder.is_some() {
                self.voice_passive_enabled = false;
                self.voice_recorder = None;
                self.voice_recording_started = None;
                self.voice_status = "Голосовое управление выключено; микрофон освобождён.".into();
            }
            return;
        }

        let model_loaded = self.voice_model.is_some();
        let loading = self.voice_model_pending.is_some();
        let transcribing = self.voice_transcription_pending.is_some();
        let recording = self.voice_recorder.is_some();
        let passive_enabled = self.voice_passive_enabled;
        let model_path = self.voice_model_path.clone();
        let status = self.voice_status.clone();
        let mut choose_model = false;
        let mut start_recording = false;
        let mut stop_recording = false;
        let mut set_passive: Option<bool> = None;

        if loading || transcribing || recording {
            ctx.request_repaint_after(Duration::from_millis(200));
        }

        egui::Window::new("Голосовое управление")
            .open(&mut self.voice_open)
            .default_size([500.0, 270.0])
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Локальное распознавание речи на русском языке (Whisper).");
                ui.label("Микрофон используется только пока включён один из режимов ниже.");
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
                let mut enabled = passive_enabled;
                if ui.add_enabled(model_loaded && !loading && !transcribing,
                    egui::Checkbox::new(&mut enabled, "Пассивный помощник (без записи каждой команды)")).changed() {
                    set_passive = Some(enabled);
                }
                if passive_enabled {
                    ui.label("Помощник слушает короткими отрезками, распознаёт фразы локально и выполняет команды из списка.");
                }
                ui.horizontal(|ui| {
                    if ui.add_enabled(model_loaded && !recording && !transcribing && !passive_enabled,
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
                if recording { ui.label(if passive_enabled { "Пассивный режим: записываю короткий отрезок…" } else { "Идёт запись (максимум 12 секунд)…" }); }
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

        if let Some(enabled) = set_passive {
            self.voice_passive_enabled = enabled;
            if !enabled {
                self.voice_recorder = None;
                self.voice_recording_started = None;
                self.voice_status = "Пассивный помощник выключен; микрофон освобождён.".into();
            } else {
                self.voice_status = "Пассивный помощник включён.".into();
            }
        }

        if self.voice_passive_enabled && self.voice_recorder.is_none() && self.voice_transcription_pending.is_none() {
            match MicrophoneRecorder::start() {
                Ok(recorder) => {
                    self.voice_recorder = Some(recorder);
                    self.voice_recording_started = Some(Instant::now());
                    self.voice_status = "Пассивный помощник слушает…".into();
                }
                Err(error) => {
                    self.voice_passive_enabled = false;
                    self.voice_status = format!("Не удалось включить пассивный режим: {error:#}");
                }
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
        if !self.radio_open {
            let [width, height] = self.last_viewport_size;
            self.radio_window_pos = egui::pos2(((width - 720.0) * 0.5).max(12.0), ((height - 570.0) * 0.5).max(12.0));
        }
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

    fn ensure_radio_cover_textures(&mut self, ctx: &egui::Context) {
        if self.radio_cover_textures.len() == 24 { return; }
        let covers: [&[u8]; 24] = [
            include_bytes!("../assets/radio/01.png"), include_bytes!("../assets/radio/02.png"),
            include_bytes!("../assets/radio/03.png"), include_bytes!("../assets/radio/04.png"),
            include_bytes!("../assets/radio/05.png"), include_bytes!("../assets/radio/06.png"),
            include_bytes!("../assets/radio/07.png"), include_bytes!("../assets/radio/08.png"),
            include_bytes!("../assets/radio/09.png"), include_bytes!("../assets/radio/10.png"),
            include_bytes!("../assets/radio/11.png"), include_bytes!("../assets/radio/12.png"),
            include_bytes!("../assets/radio/13.png"), include_bytes!("../assets/radio/14.png"),
            include_bytes!("../assets/radio/15.png"), include_bytes!("../assets/radio/16.png"),
            include_bytes!("../assets/radio/17.png"), include_bytes!("../assets/radio/18.png"),
            include_bytes!("../assets/radio/19.png"), include_bytes!("../assets/radio/20.png"),
            include_bytes!("../assets/radio/21.png"), include_bytes!("../assets/radio/22.png"),
            include_bytes!("../assets/radio/23.png"), include_bytes!("../assets/radio/24.png"),
        ];
        self.radio_cover_textures = covers.iter().enumerate().filter_map(|(index, bytes)| {
            let decoded = image::load_from_memory(bytes).ok()?;
            let rgba = decoded.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let color = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
            Some(ctx.load_texture(format!("tschat-radio-cover-{index}"), color, egui::TextureOptions::LINEAR))
        }).collect();
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
        self.ensure_radio_cover_textures(ctx);

        let mut open = self.radio_open;
        let mut drag_delta = egui::Vec2::ZERO;
        let mut close_requested = false;
        let mut start_request: Option<(String, String)> = self.radio_autoplay_pending
            .take()
            .and_then(|index| self.radio_stations.get(index).map(|station| (station.url.trim().to_string(), station.name.clone())));
        let mut stop_requested = false;
        let mut pause_requested = false;
        let mut edit_url_requested = false;
        let mut toggle_favorite_requested = false;
        let paused = self.radio_player.as_ref().map(RadioPlayback::is_paused).unwrap_or(true);
        let player_present = self.radio_player.is_some();
        let playing = self.radio_player.is_some() && !paused;
        let connecting = self.radio_pending.is_some();
        let audio_level = self.radio_player.as_ref().map(RadioPlayback::level).unwrap_or(0.0);
        let current_volume = self.radio_volume;
        let stations = &mut self.radio_stations;
        let selected = &mut self.selected_radio_station;
        let volume = &mut self.radio_volume;
        let muted = self.radio_muted;

        let covers = self.radio_cover_textures.clone();
        let current_track = self.radio_player.as_ref().and_then(RadioPlayback::track_title);
        let time = ctx.input(|input| input.time) as f32;
        let reactive_level = if playing && !self.radio_muted { (audio_level * current_volume * 4.0).clamp(0.06, 1.0) } else { 0.0 };
        ctx.request_repaint_after(Duration::from_millis(33));

        egui::Window::new("Радио таверны")
            .id(egui::Id::new("legacy-radio-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([760.0, 620.0])
            .current_pos(self.radio_window_pos)
            .movable(false)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                let bg = egui::Color32::from_rgb(13, 24, 20);
                let panel = egui::Color32::from_rgb(22, 37, 31);
                let panel_soft = egui::Color32::from_rgb(29, 47, 39);
                let border = egui::Color32::from_rgb(48, 68, 56);
                let amber = egui::Color32::from_rgb(232, 170, 91);
                let cream = egui::Color32::from_rgb(235, 231, 219);
                let secondary = egui::Color32::from_rgb(159, 173, 163);
                let rect = ui.max_rect();
                let painter = ui.painter().clone();
                painter.rect_filled(rect, 18.0, bg);
                painter.rect_stroke(rect, 18.0, egui::Stroke::new(1.0, border));

                let header = egui::Rect::from_min_size(rect.min + egui::vec2(14.0, 7.0), egui::vec2(rect.width() - 28.0, 38.0));
                let drag = ui.interact(egui::Rect::from_min_max(header.min, egui::pos2(header.right() - 42.0, header.bottom())), egui::Id::new("radio-title-drag"), egui::Sense::click_and_drag());
                if drag.dragged() { drag_delta = ctx.input(|input| input.pointer.delta()); }
                painter.text(header.left_center() + egui::vec2(2.0, 0.0), egui::Align2::LEFT_CENTER, "♫", egui::FontId::proportional(22.0), amber);
                painter.text(header.left_center() + egui::vec2(32.0, 0.0), egui::Align2::LEFT_CENTER, "Радио", egui::FontId::proportional(19.0), cream);
                let close_rect = egui::Rect::from_min_size(egui::pos2(header.right() - 34.0, header.top() + 3.0), egui::vec2(30.0, 29.0));
                if ui.put(close_rect, egui::Button::new(egui::RichText::new("×").size(21.0).color(cream)).fill(panel_soft)).clicked() { close_requested = true; }
                painter.line_segment([egui::pos2(rect.left() + 16.0, rect.top() + 50.0), egui::pos2(rect.right() - 16.0, rect.top() + 50.0)], egui::Stroke::new(1.0, border));

                let body = egui::Rect::from_min_max(egui::pos2(rect.left() + 16.0, rect.top() + 62.0), rect.right_bottom() - egui::vec2(16.0, 16.0));
                let left = egui::Rect::from_min_max(body.min, egui::pos2(body.left() + 306.0, body.bottom()));
                let right = egui::Rect::from_min_max(egui::pos2(left.right() + 12.0, body.top()), body.max);
                painter.rect_filled(left, 14.0, panel);
                painter.rect_filled(right, 14.0, panel);
                painter.rect_stroke(left, 14.0, egui::Stroke::new(1.0, border));
                painter.rect_stroke(right, 14.0, egui::Stroke::new(1.0, border));

                let list_header = egui::Rect::from_min_size(left.min + egui::vec2(14.0, 10.0), egui::vec2(left.width() - 28.0, 38.0));
                painter.text(list_header.left_center(), egui::Align2::LEFT_CENTER, "◉   Радио", egui::FontId::proportional(16.0), cream);
                let add_rect = egui::Rect::from_min_size(egui::pos2(list_header.right() - 34.0, list_header.top() + 2.0), egui::vec2(32.0, 32.0));
                if ui.put(add_rect, egui::Button::new(egui::RichText::new("+").size(20.0).color(cream)).fill(panel_soft)).on_hover_text("Настроить URL выбранной станции").clicked() { edit_url_requested = true; }
                painter.line_segment([egui::pos2(left.left() + 12.0, left.top() + 54.0), egui::pos2(left.right() - 12.0, left.top() + 54.0)], egui::Stroke::new(1.0, border));

                let list_rect = egui::Rect::from_min_max(egui::pos2(left.left() + 8.0, left.top() + 62.0), egui::pos2(left.right() - 8.0, left.bottom() - 10.0));
                ui.allocate_ui_at_rect(list_rect, |list_ui| {
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(list_ui, |list_ui| {
                        for (index, station) in stations.iter().enumerate() {
                            let row = egui::Rect::from_min_size(list_ui.cursor().min, egui::vec2(list_ui.available_width(), 54.0));
                            let active = *selected == index;
                            let response = list_ui.put(row, egui::Button::new("").fill(if active { egui::Color32::from_rgb(91, 70, 44) } else { panel }));
                            if let Some(texture) = covers.get(index.min(covers.len().saturating_sub(1))) {
                                let thumb = egui::Rect::from_min_size(row.min + egui::vec2(8.0, 6.0), egui::vec2(42.0, 42.0));
                                list_ui.put(thumb, egui::Image::new((texture.id(), thumb.size())));
                                list_ui.painter().rect_stroke(thumb, 6.0, egui::Stroke::new(1.0, if active { amber } else { border }));
                            }
                            list_ui.put(egui::Rect::from_min_size(row.min + egui::vec2(60.0, 7.0), egui::vec2(row.width() - 68.0, 20.0)),
                                egui::Label::new(egui::RichText::new(&station.name).size(13.0).strong().color(cream)).truncate());
                            let subtitle = if active && playing { "Сейчас в эфире" } else if station.url.trim().is_empty() { "Настройте адрес потока" } else { "Радиостанция" };
                            list_ui.put(egui::Rect::from_min_size(row.min + egui::vec2(60.0, 29.0), egui::vec2(row.width() - 68.0, 17.0)),
                                egui::Label::new(egui::RichText::new(subtitle).size(11.0).color(secondary)));
                            if active && playing {
                                list_ui.painter().text(row.right_center() - egui::vec2(12.0, 0.0), egui::Align2::RIGHT_CENTER, "▮▮▮", egui::FontId::proportional(11.0), amber);
                            }
                            if response.clicked() {
                                *selected = index;
                                start_request = Some((station.url.trim().to_string(), station.name.clone()));
                            }
                            list_ui.add_space(4.0);
                        }
                    });
                });

                let station_name = stations.get(*selected).map(|station| station.name.as_str()).unwrap_or("Выберите станцию");
                let cover = covers.get((*selected).min(covers.len().saturating_sub(1)));
                let image_rect = egui::Rect::from_min_size(right.min + egui::vec2(18.0, 16.0), egui::vec2(right.width() - 36.0, 205.0));
                let glow = if playing && !muted { ((reactive_level + 0.08 * (time * 7.0).sin().abs()).clamp(0.0, 1.0) * 210.0) as u8 } else { 28 };
                for (extra, width, alpha) in [(6.0, 7.0, glow / 4), (3.0, 3.0, glow / 2), (0.0, 1.5, glow)] {
                    painter.rect_stroke(image_rect.expand(extra), 12.0, egui::Stroke::new(width, egui::Color32::from_rgba_unmultiplied(232, 170, 91, alpha)));
                }
                if let Some(texture) = cover {
                    ui.put(image_rect, egui::Image::new((texture.id(), image_rect.size())));
                }
                painter.rect_stroke(image_rect, 10.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(89, 92, 66)));
                painter.text(egui::pos2(right.left() + 20.0, image_rect.bottom() + 28.0), egui::Align2::LEFT_CENTER, station_name, egui::FontId::proportional(21.0), cream);
                painter.text(egui::pos2(right.left() + 20.0, image_rect.bottom() + 56.0), egui::Align2::LEFT_CENTER, "Сейчас в эфире:", egui::FontId::proportional(12.0), secondary);
                let track = current_track.as_deref().unwrap_or(if playing { "Прямой эфир" } else if connecting { "Подключение…" } else { "Остановлено" });
                painter.text(egui::pos2(right.left() + 20.0, image_rect.bottom() + 78.0), egui::Align2::LEFT_CENTER, track, egui::FontId::proportional(15.0), cream);

                let wave = egui::Rect::from_min_size(egui::pos2(right.left() + 18.0, image_rect.bottom() + 102.0), egui::vec2(right.width() - 36.0, 32.0));
                let wave_response = ui.interact(wave, egui::Id::new("radio-volume-waveform"), egui::Sense::hover()).on_hover_text("Визуализатор уровня звука · прокрутка меняет громкость");
                if wave_response.hovered() {
                    let scroll = ctx.input(|input| input.smooth_scroll_delta.y);
                    if scroll.abs() > 0.0 { *volume = (*volume + scroll.signum() * 0.04).clamp(0.0, 1.0); }
                }
                for index in 0..42 {
                    let x = wave.left() + 4.0 + index as f32 * (wave.width() - 8.0) / 42.0;
                    let music = (time * 8.0 + index as f32 * 0.57).sin().abs() * 0.32
                        + (time * 3.1 - index as f32 * 0.19).cos().abs() * 0.2;
                    let amplitude = if playing && !muted { (0.08 + reactive_level * (0.3 + music)).clamp(0.06, 1.0) } else { 0.12 };
                    let h = 3.0 + amplitude * (26.0 * (1.0 - ((index as f32 / 41.0) * 2.0 - 1.0).abs() * 0.28));
                    let line = [egui::pos2(x, wave.center().y - h / 2.0), egui::pos2(x, wave.center().y + h / 2.0)];
                    painter.line_segment(line, egui::Stroke::new(3.0, if playing && !muted { amber } else { egui::Color32::from_rgb(78, 94, 78) }));
                }

                let control_y = wave.bottom() + 28.0;
                let controls = egui::Rect::from_min_size(egui::pos2(right.center().x - 108.0, control_y), egui::vec2(216.0, 48.0));
                let previous = egui::Rect::from_min_size(controls.min, egui::vec2(48.0, 44.0));
                if ui.put(previous, egui::Button::new(egui::RichText::new("◀|").size(18.0).color(cream)).fill(panel_soft)).clicked() && !stations.is_empty() {
                    *selected = if *selected == 0 { stations.len() - 1 } else { *selected - 1 };
                    if let Some(station) = stations.get(*selected) { start_request = Some((station.url.trim().to_string(), station.name.clone())); }
                }
                let play = egui::Rect::from_min_size(egui::pos2(controls.center().x - 24.0, controls.top()), egui::vec2(52.0, 44.0));
                if ui.put(play, egui::Button::new(egui::RichText::new(if playing { "Ⅱ" } else { "▶" }).size(20.0).color(egui::Color32::from_rgb(28, 32, 29))).fill(amber)).clicked() {
                    if player_present { pause_requested = true; }
                    else if let Some(station) = stations.get(*selected) { start_request = Some((station.url.trim().to_string(), station.name.clone())); }
                }
                let stop = egui::Rect::from_min_size(egui::pos2(controls.right() - 48.0, controls.top()), egui::vec2(48.0, 44.0));
                if ui.put(stop, egui::Button::new(egui::RichText::new("■").size(18.0).color(cream)).fill(panel_soft)).clicked() { stop_requested = true; }

                let info = egui::Rect::from_min_size(egui::pos2(right.left() + 18.0, right.bottom() - 50.0), egui::vec2(right.width() - 68.0, 36.0));
                if ui.put(info, egui::Button::new(egui::RichText::new("ⓘ   О станции").size(12.0).color(cream)).fill(panel_soft)).clicked() {
                    self.radio_status = format!("{} · {}", station_name, if playing { track } else { "Станция выбрана" });
                }
                let favorite = egui::Rect::from_min_size(egui::pos2(right.right() - 48.0, info.top()), egui::vec2(36.0, 36.0));
                let favorite_mark = if self.radio_favorites.contains(&*selected) { "♥" } else { "♡" };
                if ui.put(favorite, egui::Button::new(egui::RichText::new(favorite_mark).size(18.0).color(amber)).fill(panel_soft)).clicked() { toggle_favorite_requested = true; }
            });        if close_requested { open = false; }
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
        if toggle_favorite_requested {
            if !self.radio_favorites.remove(&self.selected_radio_station) {
                self.radio_favorites.insert(self.selected_radio_station);
            }
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
    fn floating_window_position(&mut self, ctx: &egui::Context, key: &'static str, size: egui::Vec2) -> egui::Pos2 {
        *self.window_positions.entry(key).or_insert_with(|| ctx.screen_rect().center() - size * 0.5)
    }

    fn floating_window_drag_delta(ui: &mut egui::Ui, ctx: &egui::Context, key: &'static str) -> egui::Vec2 {
        let rect = ui.max_rect();
        let drag_rect = egui::Rect::from_min_size(rect.min, egui::vec2((rect.width() - 46.0).max(0.0), 40.0));
        let response = ui.interact(drag_rect, egui::Id::new(("tschat-floating-drag", key)), egui::Sense::click_and_drag());
        if response.dragged() { ctx.input(|input| input.pointer.delta()) } else { egui::Vec2::ZERO }
    }

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
        let rect = ui.max_rect();
        let bg = egui::Color32::from_rgb(13, 24, 20);
        let surface = egui::Color32::from_rgb(22, 37, 31);
        let surface_inner = egui::Color32::from_rgb(12, 24, 20);
        let amber = egui::Color32::from_rgb(232, 170, 91);
        let cream = egui::Color32::from_rgb(235, 231, 219);
        let secondary = egui::Color32::from_rgb(159, 173, 163);
        ui.painter().rect_filled(rect, 18.0, bg);

        let bar = egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), 44.0));
        let title_color = egui::Color32::from_rgb(11, 20, 17);
        ui.painter().rect_filled(bar, 18.0, title_color);
        ui.painter().rect_filled(egui::Rect::from_min_size(bar.min + egui::vec2(0.0, 18.0), egui::vec2(bar.width(), 26.0)), 0.0, title_color);
        ui.painter().line_segment([bar.left_bottom(), bar.right_bottom()], egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 66, 56)));
        let drag_rect = egui::Rect::from_min_size(bar.min, egui::vec2((bar.width() - 110.0).max(0.0), bar.height()));
        if ui.interact(drag_rect, egui::Id::new("auth-window-title-drag"), egui::Sense::click_and_drag()).drag_started() {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if let Some(animation) = &self.tavern_animation {
            if let Some(frame) = animation.frames.get(animation.current_frame) {
                let logo = egui::Rect::from_min_size(rect.min + egui::vec2(19.0, 9.0), egui::vec2(24.0, 24.0));
                ui.put(logo, egui::Image::new((frame.id(), logo.size())));
            }
        }
        ui.put(egui::Rect::from_min_size(rect.min + egui::vec2(52.0, 7.0), egui::vec2(56.0, 28.0)),
            egui::Label::new(egui::RichText::new("TSchat").size(18.0).strong().color(amber)));
        ui.painter().line_segment([egui::pos2(rect.left() + 111.0, bar.top() + 8.0), egui::pos2(rect.left() + 111.0, bar.bottom() - 8.0)], egui::Stroke::new(1.0, egui::Color32::from_rgb(66, 83, 70)));
        ui.put(egui::Rect::from_min_size(rect.min + egui::vec2(123.0, 7.0), egui::vec2(190.0, 28.0)),
            egui::Label::new(egui::RichText::new("Таверна странника").size(13.0).color(cream)));

        let controls = [("close", "×"), ("maximize", "□"), ("min", "—")];
        let mut x = rect.right() - 34.0;
        for (kind, glyph) in controls {
            let b = egui::Rect::from_min_size(egui::pos2(x, rect.top() + 8.0), egui::vec2(26.0, 25.0));
            if ui.put(b, egui::Button::new(glyph).fill(egui::Color32::from_rgb(35, 47, 41))).clicked() {
                match kind {
                    "min" => ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true)),
                    "maximize" => {
                        let maximized = ctx.input(|input| input.viewport().maximized.unwrap_or(false));
                        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
                    }
                    _ => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                }
            }
            x -= 31.0;
        }

        let card_width = rect.width();
        let card_height = (rect.height() - 44.0).max(0.0);
        let card = egui::Rect::from_min_size(egui::pos2(rect.left(), rect.top() + 44.0), egui::vec2(card_width, card_height));
        ui.painter().rect_filled(card, 18.0, surface);
        ui.painter().rect_stroke(card, 18.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(56, 77, 64)));

        let left_width = card.width() * 0.493;
        let left = egui::Rect::from_min_max(card.left_top(), egui::pos2(card.left() + left_width, card.bottom()));
        let right = egui::Rect::from_min_max(egui::pos2(left.right(), card.top()), card.right_bottom());
        ui.painter().line_segment([egui::pos2(left.right(), card.top() + 18.0), egui::pos2(left.right(), card.bottom() - 18.0)], egui::Stroke::new(1.0, egui::Color32::from_rgb(46, 68, 56)));

        let registering = self.screen == Screen::Register;
        let animation_rect = egui::Rect::from_center_size(egui::pos2(left.center().x, left.center().y - 78.0), egui::vec2(200.0, 160.0));
        legacy_animated_image(ui, self.tavern_animation.as_ref(), animation_rect);
        ui.put(egui::Rect::from_center_size(egui::pos2(left.center().x, left.center().y + 24.0), egui::vec2(left.width() - 34.0, 38.0)),
            egui::Label::new(egui::RichText::new("Место для своих").size(25.0).strong().color(amber)).halign(egui::Align::Center));
        ui.put(egui::Rect::from_center_size(egui::pos2(left.center().x, left.center().y + 54.0), egui::vec2(left.width() - 32.0, 54.0)),
            egui::Label::new(egui::RichText::new("Небольшой локальный чат,\nгде всегда можно собраться.").size(15.0).color(secondary)).halign(egui::Align::Center));

        let form_x = right.left() + 20.0;
        let form_width = right.width() - 40.0;
        let top = right.top() + (right.height() - if registering { 390.0 } else { 330.0 }) * 0.5;
        let heading = if registering { "Создать аккаунт" } else { "С возвращением" };
        let subtitle = if registering { "Присоединяйтесь к Таверне странника" } else { "Войдите, чтобы продолжить общение" };
        let compact_form = form_width < 285.0;
        let heading_size = if compact_form { 22.0 } else { 27.0 };
        let heading_height = if compact_form { 32.0 } else { 40.0 };
        ui.put(egui::Rect::from_min_size(egui::pos2(form_x, top), egui::vec2(form_width, heading_height)),
            egui::Label::new(egui::RichText::new(heading).size(heading_size).color(cream)).truncate());
        let subtitle_y = top + heading_height + 3.0;
        ui.put(egui::Rect::from_min_size(egui::pos2(form_x, subtitle_y), egui::vec2(form_width, 32.0)),
            egui::Label::new(egui::RichText::new(subtitle).size(if compact_form { 12.0 } else { 14.0 }).color(secondary)).wrap());
        let mut y = subtitle_y + 40.0;
        let field = |ui: &mut egui::Ui, y: f32, label: &str, text: &mut String, secret: bool| {
            ui.put(egui::Rect::from_min_size(egui::pos2(form_x, y), egui::vec2(form_width, 18.0)),
                egui::Label::new(egui::RichText::new(label).size(13.0).color(secondary)));
            ui.put(egui::Rect::from_min_size(egui::pos2(form_x, y + 21.0), egui::vec2(form_width, 32.0)),
                egui::TextEdit::singleline(text).password(secret).desired_width(form_width));
        };
        field(ui, y, "Имя пользователя", &mut self.username, false);
        y += 60.0;
        field(ui, y, if registering { "Придумайте пароль" } else { "Пароль" }, &mut self.password, true);
        y += 60.0;
        if registering {
            field(ui, y, "Электронная почта", &mut self.email, false);
            y += 60.0;
        }
        let primary = egui::Rect::from_min_size(egui::pos2(form_x, y + 4.0), egui::vec2(form_width, 36.0));
        if ui.put(primary, egui::Button::new(egui::RichText::new(if registering { "Зарегистрироваться" } else { "Войти" }).size(15.0).strong().color(egui::Color32::from_rgb(28, 32, 29))).fill(amber)).clicked() {
            if registering { self.register(); } else { self.login(); }
        }
        let switch = egui::Rect::from_min_size(egui::pos2(form_x, y + 52.0), egui::vec2(form_width, 30.0));
        if ui.put(switch, egui::Button::new(egui::RichText::new(if registering { "Уже есть аккаунт?  Войти" } else { "Создать аккаунт" }).size(13.0).color(cream)).fill(surface_inner)).clicked() {
            self.screen = if registering { Screen::Login } else { Screen::Register };
            self.status.clear();
            self.email.clear();
        }
        if !self.status.is_empty() {
            let color = if self.status.starts_with("Ошибка") || self.status.starts_with("Не удалось") || self.status.starts_with("Проверьте") { egui::Color32::from_rgb(239, 117, 105) } else { egui::Color32::from_rgb(133, 202, 149) };
            ui.put(egui::Rect::from_min_size(egui::pos2(form_x, y + 98.0), egui::vec2(form_width, 52.0)),
                egui::Label::new(egui::RichText::new(self.status.clone()).size(11.0).color(color)).wrap());
        }
        ui.painter().rect_stroke(rect, 18.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(56, 77, 64)));
    }

        fn draw_legacy_chat_screen(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let full = ui.max_rect();
        let bg = egui::Color32::from_rgb(13, 23, 20);
        let surface = egui::Color32::from_rgb(22, 37, 31);
        let inset = egui::Color32::from_rgb(14, 27, 22);
        let soft = egui::Color32::from_rgb(29, 47, 39);
        let amber = egui::Color32::from_rgb(232, 170, 91);
        let cream = egui::Color32::from_rgb(235, 231, 219);
        let secondary = egui::Color32::from_rgb(157, 172, 161);
        let green = egui::Color32::from_rgb(116, 198, 139);
        ui.painter().rect_filled(full, 18.0, bg);

        let title_h = 42.0;
        let title = egui::Rect::from_min_size(full.min, egui::vec2(full.width(), title_h));
        let title_color = egui::Color32::from_rgb(11, 20, 17);
        ui.painter().rect_filled(title, 18.0, title_color);
        ui.painter().rect_filled(egui::Rect::from_min_size(title.min + egui::vec2(0.0, 18.0), egui::vec2(title.width(), title_h - 18.0)), 0.0, title_color);
        ui.painter().line_segment([title.left_bottom(), title.right_bottom()], egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 67, 56)));
        let drag_rect = egui::Rect::from_min_size(title.min, egui::vec2((title.width() - 110.0).max(0.0), title.height()));
        if ui.interact(drag_rect, egui::Id::new("chat-window-title-drag"), egui::Sense::click_and_drag()).drag_started() {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if let Some(animation) = &self.tavern_animation {
            if let Some(frame) = animation.frames.get(animation.current_frame) {
                let logo = egui::Rect::from_min_size(full.min + egui::vec2(19.0, 8.0), egui::vec2(25.0, 25.0));
                ui.put(logo, egui::Image::new((frame.id(), logo.size())));
            }
        }
        ui.put(egui::Rect::from_min_size(full.min + egui::vec2(52.0, 6.0), egui::vec2(56.0, 28.0)),
            egui::Label::new(egui::RichText::new("TSchat").size(17.0).strong().color(amber)));
        ui.painter().line_segment([egui::pos2(full.left() + 111.0, title.top() + 8.0), egui::pos2(full.left() + 111.0, title.bottom() - 8.0)], egui::Stroke::new(1.0, egui::Color32::from_rgb(66, 83, 70)));
        ui.put(egui::Rect::from_min_size(full.min + egui::vec2(123.0, 6.0), egui::vec2(145.0, 28.0)),
            egui::Label::new(egui::RichText::new("Таверна странника").size(12.0).color(secondary)));
        let mut x = full.right() - 34.0;
        for (glyph, tooltip, action) in [("×", "Закрыть", "close"), ("□", "Развернуть", "maximize"), ("—", "Свернуть", "min")] {
            x -= 32.0;
            let r = egui::Rect::from_min_size(egui::pos2(x, title.top() + 7.0), egui::vec2(27.0, 27.0));
            if ui.put(r, egui::Button::new(egui::RichText::new(glyph).size(13.0).color(cream)).fill(egui::Color32::TRANSPARENT)).on_hover_text(tooltip).clicked() {
                match action {
                    "close" => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                    "min" => ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true)),
                    "maximize" => {
                        let maximized = ctx.input(|input| input.viewport().maximized.unwrap_or(false));
                        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
                    }
                    _ => {}
                }
            }
        }

        let margin = 0.0;
        let gap = 0.0;
        let nav_w = 60.0;
        let list_w = 184.0;
        let details_w = if self.profile_expanded { 170.0 } else { 0.0 };
        let body_top = full.top() + title_h;
        let body_h = (full.bottom() - body_top).max(0.0);
        let nav = egui::Rect::from_min_size(egui::pos2(full.left(), body_top), egui::vec2(nav_w, body_h));
        let chats = egui::Rect::from_min_size(egui::pos2(nav.right() + gap, body_top), egui::vec2(list_w, body_h));
        let details_x = full.right() - margin - details_w;
        let center_x = chats.right() + gap;
        let center_w = (details_x - gap - center_x).max(0.0);
        let main = egui::Rect::from_min_size(egui::pos2(center_x, body_top), egui::vec2(center_w, body_h));
        let details = egui::Rect::from_min_size(egui::pos2(details_x, body_top), egui::vec2(details_w, body_h));
        ui.painter().rect_filled(nav, 0.0, inset);
        ui.painter().rect_filled(chats, 0.0, surface);
        ui.painter().rect_filled(main, 0.0, egui::Color32::from_rgb(17, 32, 26));
        if self.profile_expanded { ui.painter().rect_filled(details, 0.0, surface); }
        for x in [nav.right(), chats.right(), main.right()] {
            ui.painter().line_segment([egui::pos2(x, body_top), egui::pos2(x, full.bottom())], egui::Stroke::new(1.0, egui::Color32::from_rgb(46, 66, 55)));
        }

        let nav_icons = [("chat", self.selected_friend.is_none()), ("users", self.profile_expanded), ("bell", self.show_inbox), ("settings", self.quick_settings_open)];
        for (i, (name, active)) in nav_icons.iter().enumerate() {
            let b = egui::Rect::from_min_size(egui::pos2(nav.left() + 9.0, nav.top() + 10.0 + i as f32 * 48.0), egui::vec2(42.0, 42.0));
            if ui.put(b, egui::Button::new("").fill(if *active { egui::Color32::from_rgb(91, 70, 44) } else { inset })).on_hover_text(match *name { "chat" => "Общий чат", "users" => "Участники", "bell" => "Входящие", _ => "Настройки" }).clicked() {
                match *name {
                    "chat" => self.selected_friend = None,
                    "users" => self.profile_expanded = !self.profile_expanded,
                    "bell" => self.load_inbox(),
                    _ => self.quick_settings_open = !self.quick_settings_open,
                }
            }
            paint_sketch_icon(ui.painter(), b.shrink(8.0), name, if *active { amber } else { cream });
        }
        if self.quick_settings_open {
            let menu = egui::Rect::from_min_size(egui::pos2(nav.right() + 6.0, nav.top() + 187.0), egui::vec2(164.0, 102.0));
            ui.painter().rect_filled(menu, 10.0, egui::Color32::from_rgb(22, 37, 31));
            ui.painter().rect_stroke(menu, 10.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(56, 77, 64)));
            let theme_label = if self.dark_theme { "Светлая тема" } else { "Тёмная тема" };
            for (index, (label, action)) in [("Радио", "radio"), ("Оформление", "style"), (theme_label, "theme")].iter().enumerate() {
                let row = egui::Rect::from_min_size(menu.min + egui::vec2(6.0, 5.0 + index as f32 * 30.0), egui::vec2(menu.width() - 12.0, 27.0));
                if ui.put(row, egui::Button::new(egui::RichText::new(*label).size(12.0).color(cream)).fill(egui::Color32::TRANSPARENT)).clicked() {
                    match *action {
                        "radio" => self.open_radio(),
                        "style" => self.chat_style_open = true,
                        "theme" => self.dark_theme = !self.dark_theme,
                        _ => {}
                    }
                    self.quick_settings_open = false;
                }
            }
        }
        let avatar_rect = egui::Rect::from_min_size(egui::pos2(nav.center().x - 18.0, nav.bottom() - 58.0), egui::vec2(36.0, 36.0));
        if let Some(texture) = &self.avatar_texture {
            ui.put(avatar_rect, egui::Image::new((texture.id(), avatar_rect.size())));
        } else {
            ui.painter().circle_filled(avatar_rect.center(), 16.0, surface);
            paint_sketch_icon(ui.painter(), avatar_rect.shrink(7.0), "profile", cream);
        }
        ui.painter().circle_filled(avatar_rect.right_bottom() - egui::vec2(3.0, 3.0), 5.0, green);

        let pad = 12.0;
        ui.put(egui::Rect::from_min_size(chats.min + egui::vec2(pad, 14.0), egui::vec2(130.0, 24.0)),
            egui::Label::new(egui::RichText::new("Чаты").size(14.0).strong().color(cream)));
        let plus = egui::Rect::from_min_size(egui::pos2(chats.right() - 34.0, chats.top() + 7.0), egui::vec2(26.0, 26.0));
        if ui.put(plus, egui::Button::new(egui::RichText::new("+").size(18.0).color(cream)).fill(soft)).clicked() {
            self.status = "Личные сообщения доступны через список собеседников.".into();
        }
        let selected = self.selected_friend.is_none();
        let chat_item = egui::Rect::from_min_size(chats.min + egui::vec2(6.0, 50.0), egui::vec2(chats.width() - 12.0, 52.0));
        if ui.put(chat_item, egui::Button::new("").fill(if selected { egui::Color32::from_rgb(122, 91, 53) } else { soft })).clicked() { self.selected_friend = None; }
        ui.painter().circle_filled(egui::pos2(chat_item.left() + 22.0, chat_item.center().y), 14.0, egui::Color32::from_rgb(116, 123, 105));
        paint_sketch_icon(ui.painter(), egui::Rect::from_center_size(egui::pos2(chat_item.left() + 22.0, chat_item.center().y), egui::vec2(22.0, 22.0)), "users", cream);
        ui.put(egui::Rect::from_min_size(egui::pos2(chat_item.left() + 42.0, chat_item.top() + 7.0), egui::vec2(chats.width() - 78.0, 19.0)),
            egui::Label::new(egui::RichText::new("Общий чат").size(12.0).strong().color(if selected { cream } else { cream })));
        let latest_preview = self.chat_text.lines().rev().find_map(|line| line.split_once(": ").map(|(_, body)| body.trim())).unwrap_or("Таверна: обсуждения и планы");
        ui.put(egui::Rect::from_min_size(egui::pos2(chat_item.left() + 42.0, chat_item.top() + 27.0), egui::vec2(chats.width() - 54.0, 17.0)),
            egui::Label::new(egui::RichText::new(latest_preview.chars().take(25).collect::<String>()).size(10.0).color(secondary)).truncate());
        let list_rect = egui::Rect::from_min_max(chats.min + egui::vec2(6.0, 108.0), chats.right_bottom() - egui::vec2(6.0, 8.0));
        ui.allocate_ui_at_rect(list_rect, |list_ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(list_ui, |list_ui| {
                for name in self.friends.clone() {
                    let active = self.selected_friend.as_deref() == Some(name.as_str());
                    let row_rect = egui::Rect::from_min_size(list_ui.cursor().min, egui::vec2(list_ui.available_width(), 48.0));
                    let row = list_ui.put(row_rect, egui::Button::new("").fill(if active { egui::Color32::from_rgb(76, 61, 41) } else { surface }));
                    list_ui.painter().circle_filled(egui::pos2(row_rect.left() + 18.0, row_rect.center().y), 12.0, egui::Color32::from_rgb(80, 105, 91));
                    let initials = name.chars().next().unwrap_or('?').to_uppercase().to_string();
                    list_ui.put(egui::Rect::from_center_size(egui::pos2(row_rect.left() + 18.0, row_rect.center().y), egui::vec2(20.0, 20.0)),
                        egui::Label::new(egui::RichText::new(initials).size(11.0).strong().color(cream)).halign(egui::Align::Center));
                    list_ui.put(egui::Rect::from_min_size(egui::pos2(row_rect.left() + 36.0, row_rect.top() + 5.0), egui::vec2(row_rect.width() - 42.0, 18.0)),
                        egui::Label::new(egui::RichText::new(&name).size(12.0).strong().color(if active { amber } else { cream })));
                    list_ui.put(egui::Rect::from_min_size(egui::pos2(row_rect.left() + 36.0, row_rect.top() + 24.0), egui::vec2(row_rect.width() - 42.0, 16.0)),
                        egui::Label::new(egui::RichText::new("Личные сообщения").size(10.0).color(secondary)));
                    if row.clicked() { self.selected_friend = Some(name.clone()); self.load_selected_profile(); }
                    list_ui.add_space(3.0);
                }
            });
        });

        let title_bar = egui::Rect::from_min_size(main.min + egui::vec2(14.0, 10.0), egui::vec2(main.width() - 28.0, 39.0));
        let room = egui::Rect::from_min_size(title_bar.min, egui::vec2(32.0, 32.0));
        ui.painter().circle_filled(room.center(), 16.0, soft);
        paint_sketch_icon(ui.painter(), room.shrink(6.0), "users", cream);
        let more_rect = egui::Rect::from_min_size(egui::pos2(title_bar.right() - 31.0, title_bar.top() + 1.0), egui::vec2(30.0, 30.0));
        let phone_rect = egui::Rect::from_min_size(egui::pos2(more_rect.left() - 38.0, more_rect.top()), egui::vec2(30.0, 30.0));
        let search_rect = egui::Rect::from_min_size(egui::pos2(phone_rect.left() - 38.0, phone_rect.top()), egui::vec2(30.0, 30.0));
        let title_text_width = (search_rect.left() - title_bar.left() - 54.0).max(54.0);
        ui.put(egui::Rect::from_min_size(title_bar.min + egui::vec2(48.0, 0.0), egui::vec2(title_text_width, 23.0)),
            egui::Label::new(egui::RichText::new("Общий чат").size(17.0).strong().color(cream)).truncate());
        ui.put(egui::Rect::from_min_size(title_bar.min + egui::vec2(48.0, 22.0), egui::vec2(title_text_width, 16.0)),
            egui::Label::new(egui::RichText::new(format!("{} участника", self.friends.len() + 1)).size(10.0).color(secondary)));
        if ui.put(more_rect, egui::Button::new("").fill(soft)).clicked() { self.show_clear_chat_confirmation = true; }
        paint_sketch_icon(ui.painter(), more_rect.shrink(7.0), "more", cream);
        let search_resp = ui.put(search_rect, egui::Button::new("").fill(soft)).on_hover_text("Поиск по чату");
        paint_sketch_icon(ui.painter(), search_rect.shrink(6.0), "search", cream);
        if search_resp.clicked() { self.status = "Поиск по истории скоро появится.".into(); }
        if ui.put(phone_rect, egui::Button::new("").fill(soft)).on_hover_text("Голосовой вызов").clicked() {
            #[cfg(feature = "voice")]
            { self.voice_open = true; }
            #[cfg(not(feature = "voice"))]
            { self.status = "Голосовые вызовы доступны в сборке с feature voice.".into(); }
        }
        paint_sketch_icon(ui.painter(), phone_rect.shrink(6.0), "phone", cream);
        ui.painter().line_segment([egui::pos2(main.left() + 14.0, main.top() + 56.0), egui::pos2(main.right() - 14.0, main.top() + 56.0)], egui::Stroke::new(1.0, egui::Color32::from_rgb(46, 66, 55)));

        let compose_h = 50.0;
        let messages = egui::Rect::from_min_max(egui::pos2(main.left() + 6.0, main.top() + 90.0), egui::pos2(main.right() - 6.0, main.bottom() - compose_h - 20.0));
        ui.painter().rect_filled(messages, 12.0, inset);
        let today = egui::Rect::from_center_size(egui::pos2(main.center().x, main.top() + 72.0), egui::vec2(62.0, 22.0));
        ui.painter().rect_filled(today, 11.0, surface);
        ui.put(today, egui::Label::new(egui::RichText::new("Сегодня").size(10.0).color(secondary)).halign(egui::Align::Center));
        let smiles = self.smiley_textures.clone();
        let styles = self.chat_styles.clone();
        ui.allocate_ui_at_rect(messages.shrink(8.0), |content| {
            egui::ScrollArea::vertical().stick_to_bottom(true).auto_shrink([false, false]).show(content, |content| {
                for line in self.chat_text.lines() {
                    if let Some((author, body)) = line.split_once(": ") {
                        let is_self = self.current_user.as_ref().map(|user| user.username == author).unwrap_or(false);
                        content.with_layout(if is_self { egui::Layout::right_to_left(egui::Align::Min) } else { egui::Layout::left_to_right(egui::Align::Min) }, |content| {
                            egui::Frame::none().fill(if is_self { egui::Color32::from_rgb(88, 68, 44) } else { soft }).rounding(egui::Rounding::same(8.0)).inner_margin(egui::Margin::symmetric(14.0, 9.0)).show(content, |bubble| {
                                bubble.set_max_width(messages.width() * 0.72);
                                bubble.vertical(|bubble| {
                                    bubble.label(egui::RichText::new(author).size(12.0).strong().color(if is_self { amber } else { egui::Color32::from_rgb(190, 198, 184) }));
                                    show_chat_line(bubble, body, &smiles, &styles, true);
                                });
                            });
                        });
                    } else {
                        content.horizontal(|content| {
                            content.add_space(content.available_width() * 0.3);
                            egui::Frame::none().fill(surface).rounding(egui::Rounding::same(8.0)).inner_margin(egui::Margin::symmetric(12.0, 6.0)).show(content, |system| {
                                system.label(egui::RichText::new(line).size(12.0).color(secondary));
                            });
                        });
                    }
                    content.add_space(9.0);
                }
            });
        });

        let compose = egui::Rect::from_min_max(egui::pos2(main.left() + 6.0, main.bottom() - compose_h - 10.0), egui::pos2(main.right() - 6.0, main.bottom() - 10.0));
        ui.painter().rect_filled(compose, 12.0, soft);
        let attach = egui::Rect::from_min_size(compose.min + egui::vec2(4.0, 5.0), egui::vec2(34.0, 38.0));
        if ui.put(attach, egui::Button::new("").fill(egui::Color32::TRANSPARENT)).on_hover_text("Прикрепление изображений в оригинальном клиенте отключено").clicked() {
            self.status = "Прикрепление изображений в оригинальном клиенте отключено.".into();
        }
        paint_sketch_icon(ui.painter(), attach.shrink(8.0), "attach", cream);
        let send = egui::Rect::from_min_size(egui::pos2(compose.right() - 42.0, compose.top() + 7.0), egui::vec2(34.0, 34.0));
        if ui.put(send, egui::Button::new("").fill(amber)).clicked() { self.send_chat(); }
        paint_sketch_icon(ui.painter(), send.shrink(5.0), "send", egui::Color32::from_rgb(24, 34, 28));
        let emoji = egui::Rect::from_min_size(egui::pos2(send.left() - 38.0, compose.top() + 5.0), egui::vec2(30.0, 38.0));
        if ui.put(emoji, egui::Button::new("").fill(egui::Color32::TRANSPARENT)).on_hover_text("Смайлики").clicked() { self.show_smileys = !self.show_smileys; }
        paint_sketch_icon(ui.painter(), emoji.shrink(6.0), "smile", cream);
        let input = egui::Rect::from_min_max(egui::pos2(attach.right() + 6.0, compose.top() + 5.0), egui::pos2(emoji.left() - 6.0, compose.bottom() - 5.0));
        ui.put(input, egui::TextEdit::multiline(&mut self.draft).desired_rows(2).hint_text("Сообщение…").desired_width(input.width()));
        if self.show_smileys {
            let picker = egui::Rect::from_min_size(egui::pos2(compose.left() + 6.0, (compose.top() - 176.0).max(main.top() + 48.0)), egui::vec2(204.0, 168.0));
            ui.painter().rect_filled(picker, 8.0, soft);
            ui.painter().rect_stroke(picker, 8.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(56, 77, 64)));
            ui.allocate_ui_at_rect(picker, |picker_ui| {
                picker_ui.horizontal(|picker_ui| {
                    for (pack, label) in [(0, "Барменша"), (1, "Трактирщик")] {
                        let selected = self.selected_sticker_pack == pack;
                        if picker_ui.add(egui::Button::new(egui::RichText::new(label).size(11.0).color(if selected { amber } else { cream }))
                            .fill(if selected { egui::Color32::from_rgb(84, 69, 45) } else { soft })
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 68, 56)))).clicked() {
                            self.selected_sticker_pack = pack;
                        }
                    }
                });
                egui::Grid::new("chat-smiley-picker").num_columns(4).spacing(egui::vec2(4.0, 4.0)).show(picker_ui, |picker_ui| {
                    let start = self.selected_sticker_pack * 16;
                    for (index, (code, texture)) in self.smiley_textures[start..start + 16].to_vec().into_iter().enumerate() {
                        if picker_ui.add(egui::Image::new((texture.id(), egui::vec2(28.0, 28.0))).sense(egui::Sense::click())).clicked() {
                            self.add_smiley(&code);
                            self.show_smileys = false;
                        }
                        if index % 4 == 3 { picker_ui.end_row(); }
                    }
                });
            });
        }
        if !self.status.is_empty() {
            let color = if self.status.starts_with("Ошибка") || self.status.starts_with("Не удалось") { egui::Color32::from_rgb(239, 117, 105) } else { secondary };
            ui.put(egui::Rect::from_min_size(egui::pos2(main.left() + 18.0, compose.top() - 21.0), egui::vec2(main.width() - 36.0, 18.0)),
                egui::Label::new(egui::RichText::new(self.status.clone()).size(9.0).color(color)).truncate())
                .on_hover_text(self.status.clone());
        }

        if self.profile_expanded {
            ui.put(egui::Rect::from_min_size(details.min + egui::vec2(14.0, 12.0), egui::vec2(details.width() - 28.0, 24.0)),
                egui::Label::new(egui::RichText::new("Участник").size(14.0).strong().color(cream)));
            let avatar = egui::Rect::from_min_size(egui::pos2(details.left() + 14.0, details.top() + 48.0), egui::vec2(52.0, 52.0));
            if let Some(texture) = &self.avatar_texture {
                ui.put(avatar, egui::Image::new((texture.id(), avatar.size())));
            } else {
                ui.painter().circle_filled(avatar.center(), 26.0, egui::Color32::from_rgb(77, 95, 78));
                if let Some(user) = &self.current_user {
                    let initial = user.username.chars().next().unwrap_or('?').to_uppercase().to_string();
                    ui.put(egui::Rect::from_center_size(avatar.center(), egui::vec2(28.0, 28.0)),
                        egui::Label::new(egui::RichText::new(initial).size(18.0).strong().color(cream)).halign(egui::Align::Center));
                }
            }
            if let Some(user) = &self.current_user {
                ui.put(egui::Rect::from_min_size(egui::pos2(avatar.right() + 9.0, avatar.top() + 5.0), egui::vec2(details.width() - 84.0, 22.0)),
                    egui::Label::new(egui::RichText::new(&user.username).size(12.0).strong().color(cream)).truncate());
                ui.painter().circle_filled(egui::pos2(avatar.right() + 13.0, avatar.top() + 37.0), 3.5, green);
                ui.put(egui::Rect::from_min_size(egui::pos2(avatar.right() + 21.0, avatar.top() + 28.0), egui::vec2(75.0, 18.0)),
                    egui::Label::new(egui::RichText::new(if self.connected { "В сети" } else { "Не в сети" }).size(10.0).color(secondary)));
            }
            ui.painter().line_segment([egui::pos2(details.left() + 12.0, details.top() + 118.0), egui::pos2(details.right() - 12.0, details.top() + 118.0)], egui::Stroke::new(1.0, egui::Color32::from_rgb(46, 66, 55)));
            for (index, (icon, label, action)) in [
                ("profile", "Профиль", "profile"),
                ("bell", "Уведомления", "inbox"),
                ("file", "Файлы", "files"),
                ("link", "Ссылки", "links"),
                ("users", "Общие группы", "groups"),
            ].iter().enumerate() {
                let row = egui::Rect::from_min_size(egui::pos2(details.left() + 8.0, details.top() + 126.0 + index as f32 * 34.0), egui::vec2(details.width() - 16.0, 30.0));
                if ui.interact(row, egui::Id::new(("profile-menu-row", index)), egui::Sense::click()).clicked() {
                    match *action {
                        "profile" => self.edit_profile_open = true,
                        "inbox" => self.load_inbox(),
                        "files" => self.status = "Вложения изображений пока недоступны.".into(),
                        "links" => self.status = "Сохранённые ссылки пока недоступны.".into(),
                        _ => self.status = "Доступен только общий чат.".into(),
                    }
                }
                paint_sketch_icon(ui.painter(), egui::Rect::from_min_size(row.min + egui::vec2(8.0, 4.0), egui::vec2(22.0, 22.0)), icon, cream);
                ui.painter().text(
                    egui::pos2(row.left() + 46.0, row.center().y),
                    egui::Align2::LEFT_CENTER,
                    *label,
                    egui::FontId::proportional(11.0),
                    cream,
                );
                if *action == "groups" {
                    let badge = egui::Rect::from_min_size(egui::pos2(row.right() - 23.0, row.top() + 4.0), egui::vec2(19.0, 21.0));
                    ui.painter().rect_filled(badge, 6.0, egui::Color32::from_rgb(122, 91, 53));
                    ui.put(badge, egui::Label::new(egui::RichText::new("1").size(10.0).color(cream)).halign(egui::Align::Center));
                }
            }
        }
        ui.painter().rect_stroke(full, 18.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(56, 77, 64)));
    }

    fn show_legacy_profile_window(&mut self, ctx: &egui::Context) {
        if !self.edit_profile_open {
            return;
        }
        let mut open = self.edit_profile_open;
        let mut close_requested = false;
        let mut drag_delta = egui::Vec2::ZERO;
        let window_pos = self.floating_window_position(ctx, "profile-editor", egui::vec2(720.0, 480.0));
        egui::Window::new("Редактирование профиля")
            .id(egui::Id::new("legacy-edit-profile-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([720.0, 480.0])
            .current_pos(window_pos)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                drag_delta = Self::floating_window_drag_delta(ui, ctx, "profile-editor");
                let rect = ui.max_rect();
                let origin = rect.min;
                let dark = self.dark_theme;
                let background = egui::Color32::from_rgb(13, 24, 20);
                let surface = egui::Color32::from_rgb(22, 37, 31);
                let inset = egui::Color32::from_rgb(12, 24, 20);
                let border = egui::Color32::from_rgb(56, 77, 64);
                let amber = egui::Color32::from_rgb(232, 170, 91);
                let cream = egui::Color32::from_rgb(235, 231, 219);
                let secondary = egui::Color32::from_rgb(159, 173, 163);
                let foreground = if dark { cream } else { egui::Color32::from_rgb(35, 46, 39) };
                let painter = ui.painter().clone();
                painter.rect_filled(rect, 16.0, background);
                let title = egui::Rect::from_min_size(origin, egui::vec2(rect.width(), 48.0));
                painter.rect_filled(title, 16.0, egui::Color32::from_rgb(11, 20, 17));
                painter.rect_filled(egui::Rect::from_min_size(title.min + egui::vec2(0.0, 18.0), egui::vec2(title.width(), 30.0)), 0.0, egui::Color32::from_rgb(11, 20, 17));
                painter.line_segment([title.left_bottom(), title.right_bottom()], egui::Stroke::new(1.0, border));
                painter.text(origin + egui::vec2(22.0, 24.0), egui::Align2::LEFT_CENTER, "Профиль", egui::FontId::proportional(18.0), cream);
                painter.text(origin + egui::vec2(103.0, 24.0), egui::Align2::LEFT_CENTER, "Личные данные и фотография", egui::FontId::proportional(12.0), secondary);
                let close = egui::Rect::from_min_size(egui::pos2(rect.right() - 42.0, rect.top() + 7.0), egui::vec2(32.0, 32.0));
                if ui.put(close, egui::Button::new(egui::RichText::new("×").size(19.0).color(cream)).fill(surface)).clicked() {
                    close_requested = true;
                }

                let card = legacy_rect(origin, 14.0, 62.0, 692.0, 346.0);
                painter.rect_filled(card, 12.0, surface);
                painter.rect_stroke(card, 12.0, egui::Stroke::new(1.0, border));
                let avatar_frame = egui::Rect::from_center_size(egui::pos2(origin.x + 106.0, origin.y + 183.0), egui::vec2(136.0, 136.0));
                painter.circle_filled(avatar_frame.center(), 68.0, inset);
                if let Some(texture) = &self.avatar_texture {
                    ui.put(avatar_frame, egui::Image::new((texture.id(), avatar_frame.size())));
                } else {
                    paint_sketch_icon(ui.painter(), avatar_frame.shrink(45.0), "profile", cream);
                }
                painter.circle_stroke(avatar_frame.center(), 68.0, egui::Stroke::new(2.0, amber));
                painter.text(egui::pos2(origin.x + 106.0, origin.y + 266.0), egui::Align2::CENTER_CENTER, "Фото профиля", egui::FontId::proportional(13.0), cream);
                if ui.put(legacy_rect(origin, 33.0, 291.0, 146.0, 36.0), egui::Button::new(egui::RichText::new("Сменить фото").size(12.0).color(cream)).fill(inset)).clicked() {
                    self.choose_avatar();
                }

                let field = |ui: &mut egui::Ui, x: f32, y: f32, label: &str, value: &mut String, secret: bool| {
                    legacy_text(ui, legacy_rect(origin, x, y, 220.0, 20.0), label, secondary, 12.0);
                    legacy_text_edit(ui, legacy_rect(origin, x, y + 22.0, 220.0, 34.0), value, "", secret, foreground);
                };
                field(ui, 210.0, 84.0, "Настоящее имя", &mut self.profile_name, false);
                field(ui, 456.0, 84.0, "Никнейм", &mut self.profile_username, false);
                field(ui, 210.0, 157.0, "Почта", &mut self.profile_email, false);
                field(ui, 456.0, 157.0, "Телефон", &mut self.profile_mobile, false);
                painter.text(origin + egui::vec2(210.0, 232.0), egui::Align2::LEFT_CENTER, "Сменить пароль (необязательно)", egui::FontId::proportional(12.0), secondary);
                field(ui, 210.0, 252.0, "Новый пароль", &mut self.new_password, true);
                field(ui, 456.0, 252.0, "Повторите пароль", &mut self.repeat_password, true);

                if legacy_button(
                    ui,
                    legacy_rect(origin, 452.0, 425.0, 240.0, 38.0),
                    "Сохранить изменения",
                    amber,
                    egui::Color32::from_rgb(28, 32, 29),
                    14.0,
                ).clicked() {
                    self.save_profile();
                    if !self.status.starts_with("Не удалось") && !self.status.starts_with("Имя,") {
                        self.edit_profile_open = false;
                    }
                }
                painter.rect_stroke(rect.shrink(0.5), 16.0, egui::Stroke::new(1.0, border));
            });
        if close_requested { open = false; }
        self.window_positions.insert("profile-editor", window_pos + drag_delta);
        self.edit_profile_open = open && self.edit_profile_open;
    }

    fn show_legacy_chat_style_window(&mut self, ctx: &egui::Context) {
        if !self.chat_style_open {
            return;
        }
        let mut open = self.chat_style_open;
        let mut close_requested = false;
        let mut save_requested = false;
        let mut drag_delta = egui::Vec2::ZERO;
        let window_pos = self.floating_window_position(ctx, "chat-style", egui::vec2(420.0, 292.0));

        egui::Window::new("Оформление сообщений")
            .id(egui::Id::new("legacy-chat-style-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([420.0, 292.0])
            .current_pos(window_pos)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                drag_delta = Self::floating_window_drag_delta(ui, ctx, "chat-style");
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
        self.window_positions.insert("chat-style", window_pos + drag_delta);
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
        let mut drag_delta = egui::Vec2::ZERO;
        let window_pos = self.floating_window_position(ctx, "friend-profile", egui::vec2(595.0, 356.0));

        egui::Window::new("Профиль друга")
            .id(egui::Id::new("legacy-friend-profile-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([595.0, 356.0])
            .current_pos(window_pos)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                drag_delta = Self::floating_window_drag_delta(ui, ctx, "friend-profile");
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
        self.window_positions.insert("friend-profile", window_pos + drag_delta);
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
        let mut drag_delta = egui::Vec2::ZERO;
        let window_pos = self.floating_window_position(ctx, "private-message", egui::vec2(643.0, 430.0));

        egui::Window::new(format!("Личное сообщение для: {recipient}"))
            .id(egui::Id::new("legacy-private-message-window"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size([643.0, 430.0])
            .current_pos(window_pos)
            .frame(egui::Frame::none())
            .open(&mut open)
            .show(ctx, |ui| {
                drag_delta = Self::floating_window_drag_delta(ui, ctx, "private-message");
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
        self.window_positions.insert("private-message", window_pos + drag_delta);
        self.private_message_open = open && self.private_message_open;
    }
}

impl eframe::App for TsChatApp {

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_chat_refresh(ctx);
        self.poll_inbox_load(ctx);

        if self.screen == Screen::Chat {
            // Poll less often so idle pool sessions can be released on shared
            // MySQL accounts with a low max_user_connections limit.
            ctx.request_repaint_after(Duration::from_secs(45));
            if self.last_refresh.elapsed() >= Duration::from_secs(45) {
                self.refresh_chat();
                self.last_refresh = Instant::now();
            }
        }

        let target_size = match self.screen {
            Screen::Login | Screen::Register => [640.0, 560.0],
            Screen::Chat if self.profile_expanded => [960.0, 620.0],
            Screen::Chat => [760.0, 620.0],
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

        let mut visuals = if self.dark_theme { egui::Visuals::dark() } else { egui::Visuals::light() };
        visuals.window_rounding = egui::Rounding::same(16.0);
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);
        visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);
        visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
        visuals.widgets.active.rounding = egui::Rounding::same(6.0);
        ctx.set_visuals(visuals);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT).inner_margin(0.0))
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
            let mut open = self.show_inbox;
            let mut drag_delta = egui::Vec2::ZERO;
            let window_pos = self.floating_window_position(ctx, "inbox", egui::vec2(540.0, 500.0));
            egui::Window::new("Входящие сообщения")
                .id(egui::Id::new("tschat-inbox-window"))
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .fixed_size([540.0, 500.0])
                .current_pos(window_pos)
                .frame(egui::Frame::none())
                .open(&mut open)
                .show(ctx, |ui| {
                    drag_delta = Self::floating_window_drag_delta(ui, ctx, "inbox");
                    let rect = ui.max_rect();
                    let origin = rect.min;
                    let background = egui::Color32::from_rgb(13, 24, 20);
                    let surface = egui::Color32::from_rgb(22, 37, 31);
                    let inset = egui::Color32::from_rgb(12, 24, 20);
                    let soft = egui::Color32::from_rgb(29, 47, 39);
                    let amber = egui::Color32::from_rgb(232, 170, 91);
                    let cream = egui::Color32::from_rgb(235, 231, 219);
                    let secondary = egui::Color32::from_rgb(159, 173, 163);
                    let border = egui::Color32::from_rgb(56, 77, 64);
                    let painter = ui.painter().clone();
                    painter.rect_filled(rect, 16.0, background);
                    let header = egui::Rect::from_min_size(origin, egui::vec2(rect.width(), 50.0));
                    painter.rect_filled(header, 16.0, egui::Color32::from_rgb(11, 20, 17));
                    painter.rect_filled(egui::Rect::from_min_size(header.min + egui::vec2(0.0, 18.0), egui::vec2(header.width(), 32.0)), 0.0, egui::Color32::from_rgb(11, 20, 17));
                    painter.line_segment([header.left_bottom(), header.right_bottom()], egui::Stroke::new(1.0, border));
                    painter.text(origin + egui::vec2(20.0, 25.0), egui::Align2::LEFT_CENTER, "Входящие", egui::FontId::proportional(18.0), cream);
                    painter.text(origin + egui::vec2(113.0, 25.0), egui::Align2::LEFT_CENTER, "Личные сообщения", egui::FontId::proportional(12.0), secondary);
                    if ui.put(egui::Rect::from_min_size(egui::pos2(rect.right() - 126.0, rect.top() + 9.0), egui::vec2(76.0, 32.0)),
                        egui::Button::new(egui::RichText::new("Обновить").size(11.0).color(cream)).fill(soft)).clicked() {
                        self.load_inbox();
                    }
                    if ui.put(egui::Rect::from_min_size(egui::pos2(rect.right() - 42.0, rect.top() + 9.0), egui::vec2(32.0, 32.0)),
                        egui::Button::new(egui::RichText::new("×").size(19.0).color(cream)).fill(soft)).clicked() {
                        close_inbox = true;
                    }

                    let content_rect = legacy_rect(origin, 14.0, 62.0, rect.width() - 28.0, rect.height() - 76.0);
                    painter.rect_filled(content_rect, 10.0, inset);
                    ui.allocate_ui_at_rect(content_rect.shrink(10.0), |content| {
                        egui::ScrollArea::vertical().auto_shrink([false, false]).show(content, |content| {
                            if self.inbox_loading && inbox_items.is_empty() {
                                content.vertical_centered(|content| {
                                    content.add_space(130.0);
                                    content.spinner();
                                    content.label(egui::RichText::new("Загружаем сообщения…").size(13.0).color(secondary));
                                });
                            } else if inbox_items.is_empty() {
                                let empty = egui::Frame::none().fill(surface).rounding(egui::Rounding::same(10.0)).inner_margin(egui::Margin::symmetric(18.0, 20.0));
                                empty.show(content, |empty| {
                                    empty.vertical_centered(|empty| {
                                        paint_sketch_icon(empty.painter(), egui::Rect::from_min_size(empty.cursor().min, egui::vec2(34.0, 34.0)), "bell", amber);
                                        empty.add_space(38.0);
                                        empty.label(egui::RichText::new("Пока тихо").size(15.0).strong().color(cream));
                                        empty.label(egui::RichText::new("Здесь появятся ваши личные сообщения.").size(12.0).color(secondary));
                                    });
                                });
                            }
                            for message in &inbox_items {
                                egui::Frame::none().fill(surface).rounding(egui::Rounding::same(10.0)).inner_margin(egui::Margin::symmetric(13.0, 11.0)).show(content, |card| {
                                    card.horizontal(|card| {
                                        card.label(egui::RichText::new(&message.from_user).size(13.0).strong().color(cream));
                                        if !message.is_read {
                                            card.label(egui::RichText::new("● новое").size(10.0).color(amber));
                                        }
                                    });
                                    card.label(egui::RichText::new(&message.subject).size(12.0).strong().color(amber));
                                    card.add_space(3.0);
                                    card.label(egui::RichText::new(&message.message).size(12.0).color(secondary));
                                    card.add_space(8.0);
                                    card.with_layout(egui::Layout::right_to_left(egui::Align::Center), |card| {
                                        if card.add(egui::Button::new(egui::RichText::new("Удалить").size(11.0).color(cream)).fill(soft)).clicked() {
                                            delete_requested = Some(message.id);
                                        }
                                        if card.add(egui::Button::new(egui::RichText::new("Ответить").size(11.0).color(egui::Color32::from_rgb(28, 32, 29))).fill(amber)).clicked() {
                                            reply_requested = Some((message.from_user.clone(), message.subject.clone()));
                                        }
                                        if !message.is_read && card.add(egui::Button::new(egui::RichText::new("Прочитано").size(11.0).color(cream)).fill(soft)).clicked() {
                                            self.mark_message_read(message.id);
                                        }
                                    });
                                });
                                content.add_space(8.0);
                            }
                        });
                    });
                    painter.rect_stroke(rect.shrink(0.5), 16.0, egui::Stroke::new(1.0, border));
                });
            self.window_positions.insert("inbox", window_pos + drag_delta);
            if !open { close_inbox = true; }
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
