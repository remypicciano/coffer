# Coffer

Coffer is a cross-platform desktop application for protecting local files with a separate unlock key. Its version-one backend uses the documented authenticated container format and performs protection and restoration locally.

> **Carrier-file warning for the planned v2 feature:** an image or other file used as a key carrier must remain byte-for-byte identical. Send it with **Attach file** or **Send as document**. Do not paste it inline and do not use a chat application's normal photo-sharing button; those options commonly compress or rewrite the file, and the received copy will not unlock the container.

## Development

Requirements:

- Rust toolchain with edition 2024 support
- Platform dependencies required by `eframe` and `rfd`

Run the application:

```sh
cargo run
```

Verify changes:

```sh
cargo fmt -- --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo audit
```

## macOS release build

```sh
cargo build --release
cp target/release/coffer dist/macos/Coffer.app/Contents/MacOS/coffer
open dist/macos/Coffer.app
```

The local `dist/` directory is intentionally excluded from Git.

## Windows x64 build

An ARM Windows virtual machine may select the ARM64 target automatically. To build for conventional x64 Windows:

```sh
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

## Project documentation

- [Container format proposal](docs/coffer-format-v1.md)
- [Product roadmap](docs/roadmap.md)
- [Key-carrier safety](docs/key-carrier-safety.md)
- [Security audit notes](docs/security-audit.md)
- [Development and repository hygiene](docs/development.md)
