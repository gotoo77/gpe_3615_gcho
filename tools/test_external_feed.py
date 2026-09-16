from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

import fetch_external_feed as feed


class ExternalFeedContractTests(unittest.TestCase):
    def test_normalizes_valid_payload_for_videotex(self) -> None:
        payload = {
            "generated_at": "2026-09-16T06:00:00Z",
            "messages": [
                {
                    "id": "wire-1",
                    "category": "breve",
                    "title": "Une actualite exterieure beaucoup trop longue pour un terminal quarante colonnes",
                    "body": [
                        "Premiere ligne exterieure avec suffisamment de texte pour devoir etre tronquee proprement.",
                        "Deuxieme ligne.",
                    ],
                }
            ],
        }

        normalized = feed.normalize_payload(payload)

        self.assertEqual(normalized["generated_at"], payload["generated_at"])
        self.assertEqual(len(normalized["messages"]), 1)
        message = normalized["messages"][0]
        self.assertEqual(message["id"], "wire-1")
        self.assertEqual(message["category"], "breve")
        self.assertLessEqual(len(message["title"]), feed.MAX_TEXT_CHARS)
        self.assertTrue(all(len(line) <= feed.MAX_TEXT_CHARS for line in message["body"]))

    def test_rejects_empty_or_malformed_messages(self) -> None:
        bad_payloads = [
            {},
            {"generated_at": "x", "messages": []},
            {
                "generated_at": "x",
                "messages": [{"id": "", "category": "breve", "title": "T", "body": ["B"]}],
            },
            {
                "generated_at": "x",
                "messages": [{"id": "1", "category": "breve", "title": "T", "body": []}],
            },
        ]

        for payload in bad_payloads:
            with self.subTest(payload=payload):
                with self.assertRaises(ValueError):
                    feed.normalize_payload(payload)

    def test_caps_message_count_and_body_lines(self) -> None:
        payload = {
            "generated_at": "2026-09-16T06:00:00Z",
            "messages": [
                {
                    "id": f"m-{index}",
                    "category": "breve",
                    "title": f"MESSAGE {index}",
                    "body": [f"ligne {line}" for line in range(feed.MAX_BODY_LINES + 3)],
                }
                for index in range(feed.MAX_MESSAGES + 3)
            ],
        }

        normalized = feed.normalize_payload(payload)
        self.assertEqual(len(normalized["messages"]), feed.MAX_MESSAGES)
        self.assertTrue(
            all(len(message["body"]) == feed.MAX_BODY_LINES for message in normalized["messages"])
        )

    def test_atomic_write_replaces_snapshot_with_valid_json(self) -> None:
        payload = {
            "generated_at": "2026-09-16T06:00:00Z",
            "messages": [
                {"id": "m-1", "category": "breve", "title": "TEST", "body": ["OK"]}
            ],
        }
        with tempfile.TemporaryDirectory() as directory:
            target = Path(directory) / "news.json"
            target.write_text("old", encoding="utf-8")
            feed.atomic_write(target, feed.normalize_payload(payload))
            loaded = json.loads(target.read_text(encoding="utf-8"))
            self.assertEqual(loaded["messages"][0]["title"], "TEST")

    def test_url_policy_accepts_https_only(self) -> None:
        feed.validate_url("https://example.org/gcho/news.json")
        for url in [
            "http://example.org/feed.json",
            "file:///tmp/feed.json",
            "https://user:pass@example.org/feed.json",
            "https:///missing-host.json",
        ]:
            with self.subTest(url=url):
                with self.assertRaises(ValueError):
                    feed.validate_url(url)


if __name__ == "__main__":
    unittest.main()
