use std::ops::{Index, Range};

use crate::md_ast::Pos;

#[derive(Clone)]
pub struct Cursor<'src> {
    src: &'src str,
    len: usize,
    pub pos: Pos
}

impl<'a> Cursor<'a> {
    pub fn from_string(src: &'a str) -> Cursor<'a> {
        Cursor {
            src,
            len: src.encode_utf16().count(),
            pos: Pos::default()
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        &utf8_slice(self.src, start, end)
            .expect("Failed to take a slice of UTF8 string")
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Consume everything up to `\n` (not inclusive).
    pub fn consume_line(&mut self) -> &'a str {
        let start = self.pos.index;

        while !self.is_eof() && !self.check_curr("\n") {
            self.consume();
        }

        &self.slice(start, self.pos.index)
    }

    /// Consume characters until `enclosure` is stumbled.
    ///
    /// NOTE:
    /// it continues until `enclosure` is **actually** enclosures.
    /// For example:
    /// ```ignore
    /// let cursor = Cursor::new("some text***");
    /// cursor.consume_until("**");
    /// ```
    /// will consume `some text*`
    pub fn consume_enclosured(&mut self, enclosure: &'static str) {
        while !self.is_eof() {
            if self.match_curr(&enclosure[..1]) {
                if self.check_curr(enclosure) {
                    continue;
                }

                if enclosure.len() == 1 || self.match_curr(&enclosure[1..]) {
                    break;
                }

                continue;
            }

            self.consume();
        }
    }

    /// Consume until `seq` is stumbled.
    pub fn consume_until(&mut self, seq: &'static str) -> &'a str {
        let start = self.pos.index;

        while !self.is_eof() && !self.match_curr(seq) {
            self.consume();
        }

        &self.slice(start, self.pos.index)
    }

    /// Lookahead for `matcher`, bounded by line
    pub fn lookahead_inline(&self, matcher: &'static str) -> bool {
        let mut i = self.pos.index;
        let end = self.len() - matcher.len();

        while i <= end && self.char_at(i).is_some_and(|c| c != '\n') {
            if self.slice(i, i + matcher.len()) == matcher {
                return true;
            }

            i += 1
        }

        false
    }

    /// Lookahead for `matcher`
    pub fn lookahead(&self, matcher: &'static str) -> bool {
        let mut i = self.pos.index;
        let end = self.len() - matcher.len();

        while i <= end && self.char_at(i).is_some() {
            if self.slice(i, i + matcher.len()) == matcher {
                return true;
            }

            i += 1
        }

        false
    }

    // Consume n characters
    pub fn skip_n(&mut self, n: usize) {
        for _ in 0..n {
            if self.consume().is_none() {
                break;
            }
        }
    }

    // Get current char and move pointer one step further
    pub fn consume(&mut self) -> Option<char> {
        if let Some(ch) = self.current() {
            self.pos.index += 1;

            if ch == '\n' {
                self.pos.line += 1;
                self.pos.col = 0;
            } else {
                self.pos.col += 1;
            }

            return Some(ch);
        }

        None
    }

    // Check if the next char matches `ch`
    pub fn check_next(&self, ch: char) -> bool {
        self.next().is_some_and(|c| c == ch)
    }

    /// Check if the next sequence matches `matcher`.
    /// If so, consume it.
    pub fn match_curr(&mut self, matcher: &'static str) -> bool {
        if self.check_curr(matcher) {
            self.skip_n(matcher.len());
            return true;
        }

        false
    }

    /// Check if the next sequence matches `matcher`
    pub fn check_curr(&mut self, matcher: &'static str) -> bool {
        let start = self.pos.index;
        let end = start + matcher.len();

        end < self.len() && self[start..end].to_owned() == matcher
    }

    pub fn char_at(&self, i: usize) -> Option<char> {
        self.slice(i, self.len()).chars().next()
    }

    pub fn next(&self) -> Option<char> {
        self.slice(self.pos.index+1, self.len()).chars().next()
    }

    pub fn prev(&self) -> Option<char> {
        if self.pos.index > 0 {
            return Some(self.slice(self.pos.index-1, self.len()).chars().next().unwrap());
        }

        None
    }

    pub fn current(&self) -> Option<char> {
        if self.pos.index == self.len() {
            return None
        }

        self[self.pos.index..self.len()].chars().next()
    }

    pub fn is_start(&self) -> bool {
        self.pos.index == 0
    }

    pub fn is_eof(&self) -> bool {
        self.pos.index >= self.len()
    }

}

impl<'src> Index<Range<usize>> for Cursor<'src> {
    type Output = str;

    fn index(&self, range: Range<usize>) -> &'src Self::Output {
        if range.start >= self.len() || range.end > self.len() {
            panic!("Range out of bounds: {:?} for string length {}", range, self.len());
        }

        let start = range.start;
        let end = range.end;

        let slice = &utf8_slice(&self.src, start, end);

        #[cfg(debug_assertions)]
        if slice.is_none() {
            eprintln!("Slice taking failed for: \"{}\"", self.src)
        }

        slice.expect(
            &format!("Failed to take a slice of UTF8 string [{start}..{end}]")
        )
    }
}

/// Takes UTF8 slice from `s` string.
/// Credits: yolenoyer (https://stackoverflow.com/a/61711340/14156451)
fn utf8_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    let mut iter = s.char_indices()
        .map(|(pos, _)| pos)
        .chain(Some(s.len()))
        .skip(start)
        .peekable();
    let start_pos = *iter.peek()?;
    for _ in start..end { iter.next(); }
    Some(&s[start_pos..*iter.peek()?])
}
