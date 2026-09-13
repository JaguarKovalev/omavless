# SPDX-License-Identifier: MIT
"""Deterministic guard tests; never use a real terminal, host or auth agent."""
import importlib.util
import io
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location("human_auth", Path(__file__).with_name("human_authorization.py"))
subject = importlib.util.module_from_spec(spec)
spec.loader.exec_module(subject)


class Terminal(io.StringIO):
    def isatty(self):
        return True


class HumanAuthorizationTests(unittest.TestCase):
    def guard(self, text):
        return subject.HumanAuthorization(Terminal(text), Terminal())

    def test_nonterminal_never_executes(self):
        for input_stream, output_stream in ((io.StringIO("ready\nsettled\n"), Terminal()),
                                            (Terminal("ready\nsettled\n"), io.StringIO())):
            guard = subject.HumanAuthorization(input_stream, output_stream)
            with self.assertRaises(subject.AuthorizationUnsettled):
                guard.step("connect", lambda: self.fail("host effect"))
            self.assertTrue(guard.blocked)

    def test_each_action_requires_two_distinct_acknowledgements(self):
        guard = self.guard("ready\nsettled\nready\nsettled\n")
        calls = []
        for phase in ("connect", "disconnect"):
            self.assertEqual(guard.step(phase, lambda: calls.append(phase) or 7), 7)
        self.assertEqual(calls, ["connect", "disconnect"])
        self.assertEqual(guard.output.getvalue().count("Do NOT enter a password"), 4)

    def test_pre_action_stop_latches_without_effect_or_retry(self):
        for answer in ("", "stop\n", "settled\n", "READY\n", "ready", "x" * 100):
            guard = self.guard(answer)
            with self.assertRaises(subject.AuthorizationUnsettled):
                guard.step("connect", lambda: self.fail("host effect"))
            with self.assertRaises(subject.AuthorizationUnsettled):
                guard.step("disconnect", lambda: self.fail("automatic cleanup"))

    def test_success_does_not_advance_without_post_action_ack(self):
        guard = self.guard("ready\n")
        calls = []
        with self.assertRaises(subject.AuthorizationUnsettled):
            guard.step("connect", lambda: calls.append("connect"))
        with self.assertRaises(subject.AuthorizationUnsettled):
            guard.step("disconnect", lambda: calls.append("disconnect"))
        self.assertEqual(calls, ["connect"])

    def test_failed_effect_still_requires_settlement_and_preserves_error(self):
        for error in (ValueError("synthetic-private"), TimeoutError(), KeyboardInterrupt()):
            guard = self.guard("ready\nsettled\n")
            def fail():
                raise error
            with self.assertRaises(type(error)) as raised:
                guard.step("connect", fail)
            self.assertIs(raised.exception, error)
            self.assertFalse(guard.blocked)
            self.assertNotIn("synthetic-private", guard.output.getvalue())

    def test_failed_effect_plus_missing_ack_blocks_compensation(self):
        guard = self.guard("ready\nstop\n")
        def fail():
            raise TimeoutError()
        with self.assertRaises(subject.AuthorizationUnsettled):
            guard.step("connect", fail)
        with self.assertRaises(subject.AuthorizationUnsettled):
            guard.step("disconnect", lambda: self.fail("cleanup effect"))

    def test_invalid_phase_rejected_without_echo(self):
        guard = self.guard("ready\nsettled\n")
        with self.assertRaises(subject.AuthorizationUnsettled) as raised:
            guard.step("private://invalid", lambda: self.fail("host effect"))
        self.assertEqual(str(raised.exception), "human_authorization_unsettled")
        self.assertEqual(guard.output.getvalue(), "")

    def test_input_not_echoed_or_retained_in_error(self):
        guard = self.guard("synthetic-private-input\n")
        with self.assertRaises(subject.AuthorizationUnsettled) as raised:
            guard.step("connect", lambda: self.fail("host effect"))
        self.assertNotIn("synthetic-private-input", str(raised.exception) + guard.output.getvalue())

    def test_reentrant_effect_is_blocked(self):
        guard = self.guard("ready\nsettled\n")
        with self.assertRaises(subject.AuthorizationUnsettled):
            guard.step("connect", lambda: guard.step("disconnect", lambda: self.fail("nested effect")))
        self.assertTrue(guard.blocked)

    def test_terminal_failure_and_interruption_latch(self):
        class Broken(Terminal):
            def readline(self, _limit):
                raise KeyboardInterrupt()
        guard = subject.HumanAuthorization(Broken(), Terminal())
        with self.assertRaises(subject.AuthorizationUnsettled):
            guard.step("connect", lambda: self.fail("host effect"))
        self.assertTrue(guard.blocked)


if __name__ == "__main__":
    unittest.main()
