use std::ops::{Index, Range};

use super::pos::Pos;

#[derive(Clone)]
pub struct Cursor<'src> {
    src: &'src str,
    pub pos: Pos
}

impl<'a> Cursor<'a> {
    pub fn from_string(src: &'a str) -> Cursor<'a> {
        Cursor {
            src,
            pos: Pos::default()
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.src[start..end]
    }

    pub fn len(&self) -> usize {
        self.src.len()
    }

    // Consume n characters
    pub fn skip_n(&mut self, n: usize) {
        for _ in 0..n {
            self.consume();
        }
    }

    // Get current char and move pointer one step further
    pub fn consume(&mut self) -> char {
        let ch = self.current();
        self.pos.index += 1;

        if ch == '\n' {
            self.pos.line += 1;
            self.pos.col = 0;
        } else {
            self.pos.col += 1;
        }

        ch
    }

    // Check if the next char matches `ch`
    pub fn check_next(&self, ch: char) -> bool {
        self.pos.index + 1 < self.src.len() && self.next() == ch
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
        let end = self.pos.index + matcher.len();

        end < self.src.len() && self.src[start..end].to_owned() == matcher
    }

    pub fn char_at(&self, i: usize) -> char {
        self.src.chars().nth(i).unwrap().clone()
    }

    pub fn next(&self) -> char {
        self.src[self.pos.index+1..].chars().next().unwrap()
    }

    pub fn prev(&self) -> char {
        self.src[self.pos.index-1..].chars().next().unwrap()
    }

    pub fn current(&self) -> char {
        self.src[self.pos.index..].chars().next().unwrap()
    }

    pub fn is_eof(&self) -> bool {
        self.pos.index >= self.src.len()
    }

}

impl<'src> Index<Range<usize>> for Cursor<'src> {
    type Output = str;

    fn index(&self, range: Range<usize>) -> &'src Self::Output {
        // Check if the range is within bounds
        if range.start >= self.src.len() || range.end > self.src.len() {
            panic!("Range out of bounds: {:?} for string length {}", range, self.src.len());
        }

        let start = range.start;
        let end = range.end;

        // Extract the slice from the source string
        &self.src[start..end]
    }
}
