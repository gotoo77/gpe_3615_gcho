from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[1]
INDEX = (ROOT / "web" / "index.html").read_text(encoding="utf-8")


class WebShellContractTests(unittest.TestCase):
    def test_canvas_is_bounded_by_both_viewport_axes(self):
        self.assertIn("100dvh", INDEX)
        self.assertIn("min(100vw, calc(100dvh * 4 / 3), 960px)", INDEX)

    def test_winit_control_flow_exception_is_not_reported_as_terminal_failure(self):
        self.assertIn("Using exceptions for control flow", INDEX)
        self.assertIn("startsWith", INDEX)
        self.assertIn("if (!isControlFlowException)", INDEX)


if __name__ == "__main__":
    unittest.main()
