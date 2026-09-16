#!/usr/bin/env python3
"""Fetch one bounded external editorial feed for the 3615 GCHO build pipeline."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import tempfile
from typing import Any
from urllib.parse import urlparse
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_OUTPUT = ROOT / "content" / "news.json"
MAX_BYTES = 64 * 1024
MAX_MESSAGES = 8
MAX_BODY_LINES = 5
MAX_TEXT_CHARS = 46
TIMEOUT_SECONDS = 10


def clean_text(value: object, *, field: str) -> str:
    if not isinstance(value, str):
        raise ValueError(f"{field} must be a string")
    cleaned = " ".join(value.replace("\x00", " ").split())
    if not cleaned:
        raise ValueError(f"{field} must not be empty")
    return cleaned[:MAX_TEXT_CHARS]


def normalize_payload(payload: object) -> dict[str, object]:
    if not isinstance(payload, dict):
        raise ValueError("feed root must be an object")

    generated_at = payload.get("generated_at")
    if not isinstance(generated_at, str) or not generated_at.strip():
        raise ValueError("generated_at must be a non-empty string")

    raw_messages = payload.get("messages")
    if not isinstance(raw_messages, list) or not raw_messages:
        raise ValueError("messages must be a non-empty list")

    messages: list[dict[str, object]] = []
    for index, raw in enumerate(raw_messages[:MAX_MESSAGES]):
        if not isinstance(raw, dict):
            raise ValueError(f"message {index} must be an object")
        body = raw.get("body")
        if not isinstance(body, list) or not body:
            raise ValueError(f"message {index} body must be a non-empty list")

        normalized_body = [
            clean_text(line, field=f"message {index} body line {line_index}")
            for line_index, line in enumerate(body[:MAX_BODY_LINES])
        ]
        if not normalized_body:
            raise ValueError(f"message {index} body must contain usable text")

        messages.append(
            {
                "id": clean_text(raw.get("id"), field=f"message {index} id"),
                "category": clean_text(
                    raw.get("category"), field=f"message {index} category"
                ),
                "title": clean_text(raw.get("title"), field=f"message {index} title"),
                "body": normalized_body,
            }
        )

    return {"generated_at": generated_at.strip(), "messages": messages}


def validate_url(url: str) -> None:
    parsed = urlparse(url)
    if parsed.scheme != "https":
        raise ValueError("external feed URL must use https")
    if not parsed.hostname:
        raise ValueError("external feed URL must include a host")
    if parsed.username is not None or parsed.password is not None:
        raise ValueError("external feed URL must not contain credentials")


def fetch_json(url: str) -> object:
    validate_url(url)
    request = Request(
        url,
        headers={
            "Accept": "application/json",
            "User-Agent": "3615-GCHO-content-fetcher/1",
        },
    )
    with urlopen(request, timeout=TIMEOUT_SECONDS) as response:
        final_url = response.geturl()
        validate_url(final_url)
        data = response.read(MAX_BYTES + 1)
        if len(data) > MAX_BYTES:
            raise ValueError(f"external feed exceeds {MAX_BYTES} bytes")
        charset = response.headers.get_content_charset() or "utf-8"
    return json.loads(data.decode(charset))


def load_json(path: Path) -> object:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def atomic_write(path: Path, payload: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    serialized = json.dumps(payload, indent=2, ensure_ascii=True) + "\n"
    fd, temporary_name = tempfile.mkstemp(prefix=f".{path.name}.", dir=path.parent, text=True)
    try:
        with os.fdopen(fd, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(serialized)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary_name, path)
    except Exception:
        try:
            os.unlink(temporary_name)
        except FileNotFoundError:
            pass
        raise


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Fetch and normalize one external 3615 GCHO editorial JSON feed"
    )
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--url", help="HTTPS URL of the configured JSON feed")
    source.add_argument("--source-file", type=Path, help="local JSON source for offline verification")
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    raw: Any
    if args.url:
        raw = fetch_json(args.url)
        source = args.url
    else:
        raw = load_json(args.source_file)
        source = str(args.source_file)

    normalized = normalize_payload(raw)
    atomic_write(args.output, normalized)
    print(f"external feed accepted from {source}; wrote {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
