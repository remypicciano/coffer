# Public repository readiness

Last reviewed: July 12, 2026

## Completed

- Full reachable Git history scanned with Gitleaks 8.30.1: 23 commits, no leaks found.
- Tracked-path audit found no local AI-assistant configuration, instruction files, keys, containers, environment files, or private-key files.
- Rust formatting, all tests, strict Clippy, and RustSec audit pass.
- Linux aarch64 and x86_64 builds run natively in GitHub Actions.
- GitHub Actions receive read-only repository contents permission and cannot approve pull requests.
- Third-party Actions are pinned to reviewed full commit SHAs.
- Dependabot monitors Cargo and GitHub Actions dependencies weekly.
- Security, support, contribution, ownership, and issue-reporting policies are present.
- Public bug forms explicitly prohibit keys and confidential data.

## Must be completed before visibility changes

- Select and add an explicit software license.
- Merge the reviewed release-readiness pull request into `main`.
- Review and intentionally remove or retain every non-default remote branch.
- Enable private vulnerability reporting after the repository becomes eligible.
- Enable branch protection or a ruleset for `main`, requiring pull requests and passing Linux, Windows, and security checks.
- Enable repository secret scanning and push protection; public repositories receive GitHub secret scanning automatically.
- Confirm the public repository description, topics, homepage, and support destinations.

## Required before official production releases

- Add coverage-guided fuzzing for container, payload, and key parsers.
- Complete Kali runtime tracing, hardening, resource-pressure, and memory-residue tests.
- Sign Windows and macOS releases with publisher identities and notarize macOS builds.
- Sign Linux release checksums or provenance with a separately protected identity.
- Define release support windows and a vulnerability-response process.
- Obtain independent review of cryptographic construction, parsing, and filesystem behavior.

Changing visibility is not a release. Source may become public while downloadable builds remain explicitly marked prerelease and unsigned.
