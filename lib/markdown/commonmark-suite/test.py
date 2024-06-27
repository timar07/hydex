import urllib.request
import json
import ctypes
import os
import sys

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

link = "https://spec.commonmark.org/0.31.2/spec.json"
f = urllib.request.urlopen(link)
test_suite = json.loads(f.read())

for test in test_suite:
    got = md_to_html(test['markdown'])
    expected = test['html']

    if got != expected:
        print(f"Fail: expected\n{expected}\ngot:\n{got}\n\n")

# print(md_to_html("# Hello from C!\nThis is *Markdown*."))