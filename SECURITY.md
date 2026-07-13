# Security policy

## Supported versions

Coffer has not published a stable release yet. Security fixes currently target the latest commit on the default branch. This policy will be updated with explicit support windows when versioned releases begin.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability. Use GitHub's private vulnerability reporting page:

<https://github.com/remypicciano/coffer/security/advisories/new>

Include the affected commit or version, operating system and architecture, a minimal reproduction using synthetic data, expected and observed behavior, and your assessment of impact. Do not send real `.cofferkey` files, passphrases, carrier files, confidential plaintext, or private containers.

You should receive an initial acknowledgement within five business days. Resolution timing depends on reproducibility, severity, and the need to preserve container compatibility. Please allow a reasonable remediation window before public disclosure.

## Security boundary

Coffer protects file contents and authenticated filename metadata while the container and its matching key remain separated. It does not protect secrets from malware or an administrator already controlling the user session, a compromised build or operating system, screen capture, active process-memory inspection, or disclosure of both the container and matching key.

No project representative will ask for an unlock key, passphrase, carrier file, or confidential plaintext as part of support or vulnerability triage.
