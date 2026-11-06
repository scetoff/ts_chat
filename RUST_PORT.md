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
- Insert the original 16 smiley shortcodes into the message composer.
- Send private messages, view the inbox, and mark received messages as read.
- Toggle light and dark themes.

## Build and run

Install stable Rust and Cargo, then:

```sh
cp config.example.toml config.toml
# Set database credentials in config.toml.
cargo check
cargo run
```

The configured database must already exist and the supplied MySQL user must have
permissions to create the three TSchat tables. Do not commit `config.toml`.

## Compatibility and remaining work

The current code is a functional starting point, not a complete recreation of the
legacy client. The next migration stages are:

1. Native rendering of the original 16 emoticon images and rich-text message formatting.
2. Image attachments (the legacy attachment button was commented out in the VB.NET source).
3. Complete profile preferences and settings persistence for chat colors/fonts.
3. Notes/drawing.
4. Radio streaming and media controls.
5. Speech commands.
6. Automated tests and a Windows release build.

The VB.NET client uses .NET Framework 4.5, Windows Forms, MySQL Connector/NET,
Windows Media Player COM components, and Microsoft Speech SDK. The GUI and platform
integrations therefore need idiomatic Rust equivalents rather than line-by-line
translation.

## Password compatibility warning

The login digest temporarily reproduces the legacy MD5 + `ZUBI` transformation so
existing accounts can be tested. This is cryptographically weak and must not be used
for new production accounts. Before a public release, migrate to Argon2id using a
password-reset or one-time rehash flow. This compatibility digest is not a security
upgrade.

All SQL statements in the Rust client use bound parameters. Test using a disposable
copy of the original database before connecting to production data.
