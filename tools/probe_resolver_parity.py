#!/usr/bin/env python3
"""Synthetic-only actual Python DNS oracle; all network and file effects mocked."""
import json
import sys
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import backend  # noqa: E402


def main():
    try:
        raw = sys.stdin.buffer.read(65537)
        if len(raw) > 65536 or sys.argv[1:]:
            return 1
        data = json.loads(raw)
        if set(data) != {"cases"} or len(data["cases"]) != 4:
            return 1
        for case in data["cases"]:
            assert list(backend.dns_question(case["host"], 7, case["kind"])) == case["question"]
            assert backend.parse_dns_addresses(bytes(case["packet"]), 7) == case["addresses"]
        text = ("dns:\n  nameserver:\n    - https://1.1.1.1/dns-query#PROXY\n"
                "  proxy-server-nameserver:\n    - https://8.8.8.8/dns-query\n"
                "  direct-nameserver:\n    - 'https://9.9.9.9/dns-query'\n"
                "    - https://1.1.1.1/dns-query\n")
        paths = SimpleNamespace(config=Path("/synthetic/config"), template=Path("/synthetic/template"))
        with patch.object(Path, "exists", return_value=True), \
             patch.object(Path, "is_symlink", return_value=False), \
             patch.object(Path, "is_file", return_value=True), \
             patch.object(backend, "read_text_file", return_value=text):
            assert backend.configured_probe_resolvers(paths) == [
                "https://9.9.9.9/dns-query", "https://1.1.1.1/dns-query", "https://8.8.8.8/dns-query"]
        with patch.object(backend, "configured_probe_resolvers", return_value=["a", "b"]), \
             patch.object(backend, "probe_resolver_works", side_effect=lambda value: value == "b"):
            assert backend.configured_working_probe_resolvers(paths) == ["b"]
        with patch.object(backend, "configured_probe_resolvers", return_value=["a", "b"]), \
             patch.object(backend, "probe_resolver_works", return_value=False):
            assert backend.configured_working_probe_resolvers(paths) == ["a", "b"]
        calls = []

        def records(host, resolver, kind):
            calls.append((resolver, kind))
            return ("1.1.1.1",) if resolver == "b" and kind == 1 else ()

        with patch.object(backend, "resolve_probe_records", side_effect=records), \
             patch.object(backend, "system_probe_addresses", side_effect=AssertionError):
            assert backend.resolve_probe_addresses("example.com", ["a", "b"]) == ["1.1.1.1"]
        assert calls == [("a", 1), ("a", 28), ("b", 1), ("b", 28)]
        print("PASS 4 DNS cases; resolver order; health filter; address fallback")
        return 0
    except Exception:
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
