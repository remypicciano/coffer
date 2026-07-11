# Coffer container and key format v1

Status: design specification. The current UI still simulates file operations; this document defines the format that the real implementation must follow.

All integers are unsigned and stored in big-endian byte order. Parsers must reject truncated input, trailing data that is inconsistent with declared lengths, unknown required flags, and values above the limits below.

## Security model

Coffer v1 uses AES-256-GCM with a new random 256-bit key and a new random 96-bit nonce for every encryption. The entire serialized container header is passed to AES-GCM as associated authenticated data. The ciphertext includes the 128-bit GCM authentication tag produced by the `aes-gcm` crate.

The key file must be stored and transmitted separately from the `.coffer` container. Coffer has no recovery key or backdoor.

## `.coffer` container

| Field | Size | Value or limit |
| --- | ---: | --- |
| Magic | 8 bytes | ASCII `COFFER\0\x01` |
| Format version | 1 byte | `1` |
| Algorithm | 1 byte | `1` = AES-256-GCM |
| Flags | 2 bytes | `0` for v1 |
| Header length | 4 bytes | Total authenticated header length; maximum 65,536 bytes |
| Original file size | 8 bytes | Maximum implementation-supported file size |
| Creation time | 8 bytes | Unix timestamp in seconds; informational |
| Key identifier | 16 bytes | First 16 bytes of SHA-256 over the encoded key file payload |
| Nonce | 12 bytes | Cryptographically random; never reused with the same key |
| Filename length | 2 bytes | UTF-8 byte length; 1–1,024 bytes |
| Original filename | variable | UTF-8 filename only, never a path |
| Metadata length | 4 bytes | `0` in v1; maximum 32,768 bytes for future optional metadata |
| Metadata | variable | Empty in v1 |
| Ciphertext and tag | remaining | Exact source bytes encrypted with AES-256-GCM |

The authenticated header starts at Magic and ends after Metadata. No embedded directory is permitted. On restore, the filename must be treated as untrusted input: remove path components, reject `.` and `..`, reject NUL and platform separators, and use the chosen destination directory. Existing output files must not be overwritten without an explicit user decision.

Authentication failures use one public error regardless of whether the likely cause is a wrong key, modified metadata, or modified ciphertext.

## `.key` file

| Field | Size | Value |
| --- | ---: | --- |
| Magic | 8 bytes | ASCII `COFKEY\0\x01` |
| Format version | 1 byte | `1` |
| Algorithm | 1 byte | `1` = AES-256-GCM |
| Reserved | 2 bytes | `0` |
| Key material | 32 bytes | Cryptographically random AES key |

The encoded key file is exactly 44 bytes. Other lengths, magic values, versions, algorithms, or nonzero reserved bytes are invalid. Writers should request owner-only permissions on Unix-like systems (`0600`) and avoid replacing an existing key file.

The key identifier stored in a container is a locator and early mismatch check, not a secret and not an authentication substitute. Decryption must still perform AES-GCM authentication.

## Parser requirements

- Validate lengths before slicing or allocating.
- Cap the header at 65,536 bytes and optional metadata at 32,768 bytes.
- Reject unsupported versions and algorithms before decryption.
- Never derive filesystem paths directly from embedded names.
- Never write plaintext before authentication succeeds.
- Write through a temporary file in the destination directory and commit only after the complete write succeeds.
- Remove temporary output after failure or cancellation.
- Do not log keys, plaintext, ciphertext contents, or sensitive filenames.

## Compatibility

Version 1 readers must reject unknown versions rather than guessing. Future formats may add optional metadata or new algorithms under new version or algorithm identifiers. Test fixtures should be shared across macOS, Windows, and Linux to guarantee byte-for-byte compatibility.
