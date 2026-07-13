# Coffer

Coffer is a cross-platform desktop application for protecting local files with a separate unlock key. Its version-one backend uses the documented authenticated container format and performs protection and restoration locally.

Version 1.0.0 is the first stable format release. Download archives for Windows, Linux, and macOS from [GitHub Releases](https://github.com/remypicciano/coffer/releases). Release archives include SHA-256 checksums; builds are currently unsigned, so verify the checksum and repository provenance before use.

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

The **macOS build** workflow publishes Intel and Apple Silicon archives. The local `dist/` directory is intentionally excluded from Git.

## Windows x64 build

An ARM Windows virtual machine may select the ARM64 target automatically. To build for conventional x64 Windows:

```sh
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

The repository's **Windows build** GitHub Actions workflow produces a downloadable `coffer-windows-x86_64` ZIP containing `coffer-windows-x86_64.exe` and a SHA-256 checksum.

## Linux x64 build

On Debian, Ubuntu, or Kali Linux:

```sh
sudo apt update
sudo apt install -y build-essential pkg-config libx11-dev libxkbcommon-dev
cargo build --locked --release
./target/release/coffer
```

The repository's **Linux build** GitHub Actions workflow produces downloadable `coffer-linux-x86_64` and `coffer-linux-aarch64` archives on native GitHub-hosted runners. It can be started manually from the repository's **Actions** tab.

## Project documentation

- [Container format proposal](docs/coffer-format-v1.md)
- [Complete feature catalog](docs/features.md)
- [Cross-platform recovery and decryption](docs/recovery.md)
- [Product roadmap](docs/roadmap.md)
- [Key-carrier safety](docs/key-carrier-safety.md)
- [Security audit notes](docs/security-audit.md)
- [Development and repository hygiene](docs/development.md)
- [Security policy](SECURITY.md)
- [Support policy](SUPPORT.md)
- [Contributing](CONTRIBUTING.md)
- [Public repository readiness](docs/public-readiness.md)

## Security and license

Report vulnerabilities privately through [GitHub Security Advisories](https://github.com/remypicciano/coffer/security/advisories/new). Never attach real keys, private containers, or confidential plaintext to an issue. Coffer is available under the [MIT License](LICENSE).
