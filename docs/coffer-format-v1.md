# Coffer container and key format v1

Status: design specification. The current UI still simulates file operations; this document defines the minimal format that the real implementation must follow.

All integers are unsigned and stored in big-endian byte order. Parsers must reject truncated input, inconsistent lengths, unsupported versions, and unsupported algorithms.

## Design principles

- Store only what is necessary to decrypt and restore a file safely.
- Keep sensitive metadata inside the encrypted payload.
- Use a new random 256-bit key and a new random 96-bit nonce for every encryption.
- Authenticate the visible prefix and encrypted payload with AES-256-GCM.
- Never write plaintext until authentication succeeds.
- Introduce new behavior through a new format version instead of speculative v1 flags.

The separate key file must be stored and transmitted separately from the `.coffer` container. Coffer has no recovery key or backdoor.

## `.coffer` container

Only the minimum information required to recognize and decrypt the container is visible.

| Field | Size | Value or limit |
| --- | ---: | --- |
| Magic | 8 bytes | ASCII `COFFER\0\x01` |
| Format version | 1 byte | `1` |
| Algorithm | 1 byte | `1` = AES-256-GCM |
| Nonce | 12 bytes | Cryptographically random; never reused with the same key |
| Ciphertext length | 8 bytes | Length of encrypted payload including the GCM tag |
| Ciphertext and tag | declared length | Encrypted payload produced by AES-256-GCM |

The fixed 30-byte visible prefix is passed to AES-GCM as associated authenticated data. It is visible but cannot be modified without authentication failing. The container must end exactly after the declared ciphertext; trailing or missing bytes are invalid.

The container's total size still reveals an approximation of the source size. Hiding size accurately would require padding, which is outside v1.

## Encrypted payload

The payload below is serialized first and then encrypted as one authenticated unit.

| Field | Size | Value or limit |
| --- | ---: | --- |
| Filename length | 2 bytes | UTF-8 byte length; 1–1,024 bytes |
| Original filename | variable | UTF-8 filename only, never a path |
| Original file size | 8 bytes | Exact plaintext byte length |
| Original file bytes | declared size | Exact binary contents of the source file |

The original filename and exact size are therefore not readable without the correct key. No creation timestamp, generic metadata, key fingerprint, or optional flags are stored in v1.

On restore, the filename remains untrusted even though it was authenticated. Coffer must reject `.` and `..`, NUL, directory components, and platform path separators, then write only inside the destination selected by the user. Existing files must not be overwritten silently.

Authentication failures use one public error regardless of whether the likely cause is a wrong key, modified prefix, modified ciphertext, or corruption.

## `.key` file

| Field | Size | Value |
| --- | ---: | --- |
| Magic | 8 bytes | ASCII `COFKEY\0\x01` |
| Format version | 1 byte | `1` |
| Algorithm | 1 byte | `1` = AES-256-GCM |
| Reserved | 2 bytes | Must be `0` in v1 |
| Key material | 32 bytes | Cryptographically random AES key |

The encoded key file is exactly 44 bytes. Other lengths, magic values, versions, algorithms, or nonzero reserved bytes are invalid. Writers should request owner-only permissions on Unix-like systems (`0600`) and must not silently replace an existing key file.

The key format marker identifies a valid Coffer key; it does not reveal which container it opens. AES-GCM authentication determines whether the selected key matches.

## Parser requirements

- Validate every length before slicing or allocating.
- Limit filenames to 1,024 UTF-8 bytes.
- Reject unsupported versions and algorithms before decryption.
- Require the decrypted file byte count to equal the embedded original size.
- Never derive a filesystem path directly from an embedded name.
- Never write plaintext before authentication succeeds.
- Write through a temporary file in the destination directory and commit only after the complete write succeeds.
- Remove temporary output after failure or cancellation.
- Do not log keys, plaintext, ciphertext contents, or sensitive filenames.

## Compatibility

Version 1 readers must reject unknown versions rather than guessing. Future metadata, chunked large-file encryption, secret sharing, or algorithm changes require a deliberately specified new format version. Cross-platform test fixtures must guarantee byte-for-byte compatibility on macOS, Windows, and Linux.
