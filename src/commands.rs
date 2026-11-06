#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatCommand {
    OpenRadio,
    CloseRadio,
    PlayRadio,
    PauseRadio,
    StopRadio,
    OpenNotes,
    CloseNotes,
    RequestClearChat,
    SelectStation(usize),
    Unknown,
}

/// Parse the Russian phrases used by the legacy TSchat voice-command grammar.
/// This module is independent from speech recognition so it can be tested and
/// later connected to a microphone/ASR backend without changing command actions.
pub fn parse_command(input: &str) -> ChatCommand {
    let cleaned = input.trim().trim_matches(|character: char| {
        character.is_whitespace()
            || matches!(character, '.' | ',' | '!' | '?' | ':' | ';' | '…' | '"' | '\'' | '«' | '»')
    });
    let normalized = cleaned
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();

    match normalized.as_str() {
        "открой радио" | "запусти радио" => return ChatCommand::OpenRadio,
        "закрой радио" => return ChatCommand::CloseRadio,
        "выключи радио" | "отключи радио" => return ChatCommand::StopRadio,
        "включи радио" => return ChatCommand::PlayRadio,
        "пауза радио" | "приостанови радио" => return ChatCommand::PauseRadio,
        "открой заметки" => return ChatCommand::OpenNotes,
        "закрой заметки" => return ChatCommand::CloseNotes,
        "очисти чат" | "очисти весь чат" => return ChatCommand::RequestClearChat,
        _ => {}
    }

    let station_phrases: &[(&[&str], usize)] = &[
        (&["включи первую станцию"], 0),
        (&["включи русский рок", "включи рок"], 1),
        (&["включи европа плюс", "включи европу"], 2),
        (&["включи радио кафе", "включи кафе"], 3),
        (&["включи дорожное радио"], 4),
        (&["включи старое радио", "включи детское радио"], 5),
        (&["включи вип микс радио", "включи вип микс"], 6),
        (&["включи рекорд дабстеп", "включи дабстеп"], 7),
        (&["включи чилаут радио", "включи чилаут"], 8),
        (&["включи радио джаз", "включи джаз"], 9),
        (&["включи радио книга", "включи книгу", "включи аудиокнигу"], 10),
        (&["включи дискотеку девяностых", "включи дискотеку"], 11),
        (&["включи амбиэнт", "включи слипинг пил"], 12),
        (&["включи драм энд бэйс", "включи драм"], 13),
        (&["включи тектоник"], 14),
    ];

    for (phrases, station_index) in station_phrases {
        if phrases.iter().any(|phrase| normalized == *phrase) {
            return ChatCommand::SelectStation(*station_index);
        }
    }

    ChatCommand::Unknown
}

#[cfg(test)]
mod tests {
    use super::{parse_command, ChatCommand};

    #[test]
    fn parses_legacy_radio_station_aliases() {
        assert_eq!(parse_command("Включи Европу"), ChatCommand::SelectStation(2));
        assert_eq!(parse_command("Включи Европу."), ChatCommand::SelectStation(2));
        assert_eq!(parse_command("  Включи Европу!  "), ChatCommand::SelectStation(2));
        assert_eq!(parse_command("включи   русский рок"), ChatCommand::SelectStation(1));
        assert_eq!(parse_command("Включи аудиокнигу"), ChatCommand::SelectStation(10));
        assert_eq!(parse_command("Включи Тектоник"), ChatCommand::SelectStation(14));
    }

    #[test]
    fn parses_radio_and_notes_controls() {
        assert_eq!(parse_command("Открой радио"), ChatCommand::OpenRadio);
        assert_eq!(parse_command("Выключи радио"), ChatCommand::StopRadio);
        assert_eq!(parse_command("Пауза радио"), ChatCommand::PauseRadio);
        assert_eq!(parse_command("Открой заметки"), ChatCommand::OpenNotes);
        assert_eq!(parse_command("Закрой заметки"), ChatCommand::CloseNotes);
        assert_eq!(parse_command("Очисти чат"), ChatCommand::RequestClearChat);
        assert_eq!(parse_command("Очисти весь чат."), ChatCommand::RequestClearChat);
    }

    #[test]
    fn unknown_commands_are_not_executed() {
        assert_eq!(parse_command("удали все аккаунты"), ChatCommand::Unknown);
        assert_eq!(parse_command(""), ChatCommand::Unknown);
    }
}
