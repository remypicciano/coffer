# Development notes

## Repository hygiene

Local AI-assistant configuration, instructions, prompts, and session metadata are intentionally excluded from this repository. The root `.gitignore` covers the known local tool directories and instruction files.

Before committing, stage the project normally:

```sh
git add .
git status --short
git diff --cached --name-only
```

Confirm a specific local file is ignored:

```sh
git check-ignore -v .codex/config.toml
```

Abort the commit if any staged path is associated with local AI tooling:

```sh
if git diff --cached --name-only | grep -E '(^|/)(\.codex|\.agents|\.ai|\.claude|\.cursor|\.continue|\.gemini|\.windsurf|\.aider|\.skills|skills|skills-lock\.json|AGENTS\.md|CLAUDE\.md|GEMINI\.md|SKILL\.md|copilot-instructions\.md)(/|$)'; then
  echo "Remove the listed local AI files from the commit" >&2
  false
fi
```

If a local-only file was accidentally tracked in an earlier commit, ignoring it is not enough. Remove it from Git's index without deleting the local copy:

```sh
git ls-files | grep -E '(^|/)(\.codex|\.agents|\.ai|\.claude|\.cursor|\.continue|\.gemini|\.windsurf|\.aider|\.skills|skills|skills-lock\.json|AGENTS\.md|CLAUDE\.md|GEMINI\.md|SKILL\.md|copilot-instructions\.md)(/|$)'
git rm --cached -- <tracked-path>
git commit -m "Stop tracking local assistant configuration"
```

Repeat the removal command for each path printed by `git ls-files`. This affects Git's index without deleting the local file.

## Preparing a GitHub change

Feature work should be committed on a focused branch rather than directly on `main`. The current UI and workflow work belongs on:

```text
ui/responsive-key-workflows
```

Before publishing, review both the file list and the complete staged patch. Do not commit generated `target/` or `dist/` output, local assistant configuration, private test fixtures, real keys, plaintext samples, or `.coffer` containers.

Recommended conventional commit message:

```text
feat(ui): refine responsive workflows and document key carriers
```

The exact preparation and publication commands are listed in the handoff report. They are intentionally not automated by this repository.
