#!/usr/bin/env python3
"""Check local Markdown links and basic repository documentation hygiene."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlparse


ROOT = Path(__file__).resolve().parents[1]
LINK = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")


def main() -> int:
    errors: list[str] = []
    markdown = sorted(ROOT.glob("*.md")) + sorted((ROOT / "docs").glob("*.md"))
    for document in markdown:
        text = document.read_text(encoding="utf-8")
        if not text.startswith("# "):
            errors.append(f"{document.relative_to(ROOT)}: missing level-one heading")
        for raw_target in LINK.findall(text):
            target = raw_target.strip().split(maxsplit=1)[0].strip("<>")
            parsed = urlparse(target)
            if parsed.scheme or target.startswith(("#", "mailto:")):
                continue
            relative = unquote(parsed.path)
            candidate = (document.parent / relative).resolve()
            if ROOT.resolve() not in candidate.parents and candidate != ROOT.resolve():
                errors.append(f"{document.relative_to(ROOT)}: link escapes repository: {target}")
            elif not candidate.exists():
                errors.append(f"{document.relative_to(ROOT)}: missing link target: {target}")

    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print(f"Documentation validation passed ({len(markdown)} Markdown files)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
