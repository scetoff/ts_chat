# TSchat — Rust migration

This branch contains the Rust rewrite work. The original VB.NET/Windows Forms application
is preserved in the default branch and in the repository root.

## Current implementation

The initial desktop client is built with Rust, egui/eframe, Tokio and SQLx. Implemented:

- Connect to an existing MySQL database using local `config.toml`.
- Create the original TSchat tables when they do not exist.
- Authenticate against the existing `tb_users` table.
- Register users using the original user table columns.
- Read and send messages using the legacy shared `tb_chat` record.
- Poll the shared chat every two seconds while the chat screen is open.
- List registered users, view friend profiles, and show their avatars.
- Edit own nickname, display name, email and phone; change the password.
- Choose and upload a PNG/JPEG/GIF avatar (normalized to PNG, max 640×480).
- Import and render all 16 original PNG smileys in the composer and message history; shortcode compatibility is retained.
- Send private messages, view the inbox, and mark received messages as read.
- Toggle light and dark themes.
- Draw in a Paint-style notes window with adjustable brush size/color, clear canvas, and export to PNG.
- Radio station panel with the original station list, configurable stream URLs, native playback, pause/resume, stop and volume control.

## Build and run

Install stable Rust and Cargo, then:

```sh
cp config.example.toml config.toml
# Set database credentials in config.toml.
cargo check
cargo test
cargo run
```

The configured database must already exist and the supplied MySQL user must have
permissions to create the three TSchat tables. Do not commit `config.toml`.

Radio station URLs can be entered in the Radio window and saved to the local
`config.toml`. The original repository did not include its `База/URL.txt` stream
list, so the station names are restored but stream URLs are intentionally blank.
The native player supports HTTP/HTTPS live audio streams using Rodio and
stream-download. It uses a bounded adaptive cache for live streams and performs
the network connection in the background so the UI remains responsive. Playback
depends on the URL being a compatible audio stream and the Windows audio output
device being available.

## Compatibility and remaining work

The current code is a functional starting point, not a complete recreation of the
legacy client. The next migration stages are:

1. Russian speech recognition from a microphone and continuous voice-command activation; the legacy app used Microsoft Speech SDK.
2. Rich-text per-user message formatting and persistent chat font/color preferences.
3. Image attachments (the legacy attachment button was commented out in the VB.NET source).
4. Automated tests against a disposable MySQL database and a Windows release build.

The command parser has unit tests for legacy station aliases, radio/notes actions, and unsupported phrases. It is separate from speech-to-text so an offline recognizer can feed the same dispatcher later. The current command field is manual input; the microphone is not activated.

The VB.NET client uses .NET Framework 4.5, Windows Forms, MySQL Connector/NET,
Windows Media Player COM components, and Microsoft Speech SDK. The GUI and platform
integrations therefore need idiomatic Rust equivalents rather than line-by-line
translation.

## Password compatibility

New registrations and password changes use Argon2id. On successful login, a legacy MD5
+ `ZUBI` password hash is transparently upgraded to Argon2id. The old digest remains
only for compatibility with existing accounts. Test this migration with a disposable
copy of the database before using the client with real accounts.

All SQL statements in the Rust client use bound parameters. Test using a disposable
copy of the original database before connecting to production data.
