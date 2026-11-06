# TSchat — Rust migration

This branch contains the Rust rewrite work. The original VB.NET/Windows Forms application
is preserved in the default branch and in the repository root.

## Current implementation

The initial desktop client is built with Rust, egui/eframe, Tokio and SQLx. Implemented:

- Connect to an existing MySQL database using `DATABASE_URL`, individual `DB_*` environment variables, local `.env`, or `config.toml`.
- Verify the existing TSchat tables in read-only mode on connection; normal startup does not create tables or insert data.
- Check that `tb_users.password` can hold Argon2id hashes, the `tb_users` and `tb_pm` IDs are auto-incrementing, and the shared-chat row `tb_chat.id=1` exists.
- Authenticate against the existing `tb_users` table.
- Register users using the original user table columns.
- Read and send messages using the legacy shared `tb_chat` record.
- Poll the shared chat every two seconds while the chat screen is open.
- List registered users, view friend profiles, and show their avatars.
- Read and save per-user message color, font family preference, size, and style in the legacy `tb_users` columns; render each author's chat lines accordingly.
- Edit own nickname, display name, email and phone; change the password.
- Choose and upload a PNG/JPEG/GIF avatar (normalized to PNG, max 640×480).
- Import and render all 16 original PNG smileys in the composer and message history; shortcode compatibility is retained.
- Send private messages, view the inbox, and mark received messages as read.
- Toggle light and dark themes.
- Draw in a Paint-style notes window with adjustable brush size/color, clear canvas, and export to PNG.
- Radio station panel with the original station list, configurable stream URLs, native playback, pause/resume, stop and volume control.
- Optional offline Russian voice commands (`voice` Cargo feature): user-selected multilingual Whisper model, push-to-talk recording via CPAL, 12-second automatic recording limit, and command dispatch after local transcription. Microphone is never opened until the user starts a recording. The legacy `Очисти чат` / `Очисти весь чат` command opens a confirmation dialog before replacing the shared history.

## Build and run

Install stable Rust and Cargo, then:

```sh
cp config.example.toml config.toml
# Optional: copy .env.example to .env and fill in DB settings.
cargo check --all-targets
cargo test
cargo build --release --bin tschat
cargo run
# Optional local speech recognition (requires CMake, LLVM/Clang and a C++ toolchain):
cargo run --features voice
```

Connection settings can come from `.env`, process environment variables, or
`config.toml`. Environment variables take precedence. Supported names:

- `DATABASE_URL` — full MySQL connection URL, if supplied.
- `DB_CONNECTION` — `mysql` (or `mariadb`, using MySQL protocol).
- `DB_HOST`, `DB_PORT`, `DB_DATABASE`, `DB_USERNAME`, `DB_PASSWORD`.

The application reads `.env` locally via dotenvy but `.env` is ignored by Git.
Never commit real credentials.

The configured database and the original `tb_users`, `tb_chat`, and `tb_pm` tables
must already exist. Normal startup only verifies the schema and does not require CREATE
or INSERT permissions. The optional bootstrap method is not called during normal login.
Do not commit `config.toml`.

### Verify an existing test database safely

A read-only smoke checker is available:

```powershell
$env:DB_CONNECTION = "mysql"
$env:DB_HOST = "127.0.0.1"
$env:DB_PORT = "3306"
$env:DB_DATABASE = "db_TSchat_test"
$env:DB_USERNAME = "tschat_test"
$env:DB_PASSWORD = "replace-locally"
cargo run --bin db_smoke
```

It verifies the expected columns, password-hash capacity, auto-increment IDs, and
shared-chat seed row, then reads only the registered-user count and the shared-chat
row. It also prints table collations (the original VB.NET DDL used `latin1`). It does
not create tables or modify data. For GitHub Actions, add either a `DATABASE_URL` repository secret or the separate
`DB_HOST`, `DB_PORT`, `DB_DATABASE`, `DB_USERNAME`, and `DB_PASSWORD` secrets
(optionally `DB_CONNECTION`). The smoke test then runs automatically on this
repository's pull requests or can be started by manually dispatching the workflow.
It is skipped when no connection details are present. The legacy
`TSCHAT_TEST_DATABASE_URL` secret is still accepted. Use only the disposable test database, not production.

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

1. Continuous hands-free voice-command activation; the current Rust version uses push-to-talk so the microphone is only opened by an explicit user action.
2. Image attachments (the legacy attachment button was commented out in the VB.NET source).
3. Integration tests for mutating database flows and final Windows installation packaging.

The command parser has unit tests for all 15 legacy station aliases, radio/notes actions, the clear-chat confirmation request, punctuation emitted by speech recognition, and unsupported phrases. It is separate from the recognizer so manual text and Whisper transcription use the same command dispatcher.

### Optional Russian voice control

The voice feature is built with CPAL and whisper-rs. On Windows, building
whisper-rs requires Visual Studio C++ build tools, CMake, and LLVM/Clang; the
GitHub Actions workflow installs CMake and LLVM and builds a separate
voice-enabled Windows artifact.

1. Download the multilingual Whisper model [ggml-base.bin](https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin) (about 148 MB) from the official whisper.cpp model repository.
2. Build/run with `--features voice`.
3. Sign in, open **Голос**, choose `ggml-base.bin`, then click **Начать запись**. Say one command and click **Остановить и распознать**.
4. The recognized Russian text is passed to the existing command dispatcher. Recording stops after 12 seconds even if the stop button is not pressed; closing the voice window also releases the microphone.

Use the multilingual model file without the `.en` suffix. The model remains on the user's computer and is not committed to Git. The audio is processed locally by Whisper; no audio upload is performed.

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
