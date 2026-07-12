# Changelog

All notable changes to Coffer are recorded here. The project follows semantic versioning once tagged releases begin.

## Unreleased

### Added

- Production AES-256-GCM implementation of the version-one `.coffer` container.
- Strict parser for authenticated filenames, declared sizes, versions, algorithms, and trailing data.
- Versioned 44-byte `.cofferkey` encoding with owner-only Unix permissions.
- Same-directory temporary writes, no-clobber commits, cancellation cleanup, and rollback for paired outputs.
- Background Protect and Open workers connected to the desktop workflow.
- Zeroization for plaintext working buffers, key-file bytes, and in-memory key material.
- Generic lifecycle and operation logging that excludes paths, filenames, keys, and file contents.
- Deterministic compatibility fixture and negative tests for malformed, truncated, changed, and mismatched inputs.
- Plain-language attachment warnings for the planned v2 key-carrier feature.
- Documented dependency-advisory review and narrowly scoped build-time exceptions.

### Changed

- Generated key files now use the `.cofferkey` extension.
- Updated egui/eframe and the native file dialog to their current compatible releases.
- Replaced settings that had no behavioral effect with non-interactive safeguard descriptions.
- Random keys, nonces, and temporary identifiers now come directly from the operating system.

### Removed

- Simulated timer-based protection and restoration results.
- Prototype messages claiming that no files were read or written.
