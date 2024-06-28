import unittest
import urllib.request
import json
import ctypes
import os

lib_path = os.path.join(os.path.dirname(__file__), "./test.so")
lib = ctypes.CDLL(lib_path)

lib.md_to_html_generate.argtypes = [ctypes.c_char_p]
lib.md_to_html_generate.restype = ctypes.c_char_p

lib.md_to_html_free.argtypes = [ctypes.c_char_p]
lib.md_to_html_free.restype = None

def md_to_html(markdown_text):
    """Converts Markdown text to HTML using the C library.

    Args:
        markdown_text (str): The Markdown text to convert.

    Returns:
        str: The generated HTML, or None on error.
    """
    global lib
    c_markdown = ctypes.c_char_p(markdown_text.encode("utf-8"))
    c_html = lib.md_to_html_generate(c_markdown)

    if c_html is not None:
        html = ctypes.cast(c_html, ctypes.c_char_p).value.decode("utf-8")
        return html
    else:
        return None

class CommonMarkTest(unittest.TestCase):
    def __init__(self, methodName: str = "runTest") -> None:
        super().__init__(methodName)
        self.f = urllib.request.urlopen("https://spec.commonmark.org/0.31.2/spec.json")
        self.test_suite = json.loads(self.f.read())

    def test_commonmark(self):
        for test in self.test_suite:
            with self.subTest(i=test['example']):
                self.assertEqual(test['html'], md_to_html(test['markdown']))

if __name__ == '__main__':
    unittest.main()
