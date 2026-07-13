#!/usr/bin/env python3
"""Validate the dependency-free GitHub Pages site before deployment."""

from __future__ import annotations

import json
import sys
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import urlparse


ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / "site"


class PageParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.ids: set[str] = set()
        self.links: list[tuple[str, str]] = []
        self.title_seen = False
        self.description_seen = False

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = dict(attrs)
        if identifier := values.get("id"):
            if identifier in self.ids:
                raise ValueError(f"duplicate id: {identifier}")
            self.ids.add(identifier)
        if tag == "a" and (href := values.get("href")):
            self.links.append(("href", href))
        if tag in {"link", "script", "img"}:
            attribute = "href" if tag == "link" else "src"
            if value := values.get(attribute):
                self.links.append((attribute, value))
        if tag == "title":
            self.title_seen = True
        if tag == "meta" and values.get("name") == "description" and values.get("content"):
            self.description_seen = True


def validate_html(path: Path) -> list[str]:
    parser = PageParser()
    parser.feed(path.read_text(encoding="utf-8"))
    errors: list[str] = []
    if path.name == "index.html":
        if not parser.title_seen:
            errors.append(f"{path}: missing title")
        if not parser.description_seen:
            errors.append(f"{path}: missing description")

    for attribute, target in parser.links:
        parsed = urlparse(target)
        if parsed.scheme in {"http", "https", "mailto"} or target.startswith("//"):
            continue
        if target.startswith("#"):
            if target[1:] not in parser.ids:
                errors.append(f"{path}: missing fragment target {target}")
            continue
        clean_target = target.split("#", 1)[0].split("?", 1)[0]
        if not clean_target:
            continue
        candidate = (path.parent / clean_target).resolve()
        if candidate.is_dir():
            candidate /= "index.html"
        if SITE.resolve() not in candidate.parents and candidate != SITE.resolve():
            errors.append(f"{path}: local {attribute} escapes site root: {target}")
        elif not candidate.exists():
            errors.append(f"{path}: missing local {attribute}: {target}")
    return errors


def main() -> int:
    required = ["index.html", "404.html", "styles.css", "site.webmanifest", "robots.txt", "sitemap.xml", ".nojekyll"]
    errors = [f"missing site file: {name}" for name in required if not (SITE / name).exists()]
    for path in SITE.glob("*.html"):
        try:
            errors.extend(validate_html(path))
        except (OSError, UnicodeError, ValueError) as error:
            errors.append(f"{path}: {error}")

    try:
        json.loads((SITE / "site.webmanifest").read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        errors.append(f"invalid site.webmanifest: {error}")

    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print("GitHub Pages site validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
