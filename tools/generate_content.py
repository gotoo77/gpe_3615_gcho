#!/usr/bin/env python3
"""Deterministically refresh and validate 3615 GCHO editorial snapshots."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
from pathlib import Path
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CONTENT = ROOT / "content"
TEMPLATES = CONTENT / "templates" / "service_public_messages.json"
OUTPUT = CONTENT / "service_public.json"
SNAPSHOTS = [
    CONTENT / "service_public.json",
    CONTENT / "news.json",
    CONTENT / "messages.json",
    CONTENT / "secrets.json",
    CONTENT / "rencontres.json",
]


def validate_payload(payload: object, source: Path) -> None:
    if not isinstance(payload, dict):
        raise ValueError(f"{source}: root must be an object")
    generated_at = payload.get("generated_at")
    messages = payload.get("messages")
    if not isinstance(generated_at, str) or not generated_at.strip():
        raise ValueError(f"{source}: generated_at must be a non-empty string")
    if not isinstance(messages, list) or not messages:
        raise ValueError(f"{source}: messages must be a non-empty list")
    for index, message in enumerate(messages):
        if not isinstance(message, dict):
            raise ValueError(f"{source}: message {index} must be an object")
        for key in ("id", "category", "title"):
            value = message.get(key)
            if not isinstance(value, str) or not value.strip():
                raise ValueError(f"{source}: message {index} has invalid {key}")
        body = message.get("body")
        if not isinstance(body, list) or not body or not all(isinstance(line, str) for line in body):
            raise ValueError(f"{source}: message {index} has invalid body")


def load_json(path: Path) -> object:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def validate_all() -> None:
    for path in SNAPSHOTS:
        validate_payload(load_json(path), path)


def build_payload(day: dt.date) -> dict[str, object]:
    templates = load_json(TEMPLATES)
    if not isinstance(templates, list) or len(templates) < 3:
        raise ValueError(f"{TEMPLATES}: at least three templates are required")

    start = day.toordinal() % len(templates)
    selected = [templates[(start + offset) % len(templates)] for offset in range(3)]
    messages = []
    for offset, template in enumerate(selected, start=1):
        if not isinstance(template, dict):
            raise ValueError(f"{TEMPLATES}: template must be an object")
        messages.append(
            {
                "id": f"sp-{day:%Y%m%d}-{offset:02d}",
                "category": template["category"],
                "title": template["title"],
                "body": template["body"],
            }
        )

    payload: dict[str, object] = {
        "generated_at": f"{day.isoformat()}T00:00:00Z",
        "messages": messages,
    }
    validate_payload(payload, OUTPUT)
    return payload


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
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="validate snapshots without modifying them")
    parser.add_argument("--date", help="UTC date override in YYYY-MM-DD form")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.check:
        validate_all()
        print("content snapshots: OK")
        return 0

    day = dt.date.fromisoformat(args.date) if args.date else dt.datetime.now(dt.timezone.utc).date()
    atomic_write(OUTPUT, build_payload(day))
    validate_all()
    print(f"refreshed {OUTPUT.relative_to(ROOT)} for {day.isoformat()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
