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

    // Lookahead for `matcher`, bounded by line
    pub fn lookahead(&self, matcher: &'static str) -> bool {
        for i in self.pos.index..self.len() - matcher.len() {
            if self.char_at(i).is_some_and(|c| c != '\n') {
                break;
            }

            if &self.src[i..i + matcher.len()] == matcher {
                return true;
            }
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

        end < self.len() && self.src[start..end].to_owned() == matcher
    }

    pub fn char_at(&self, i: usize) -> Option<char> {
        self.src.chars().nth(i)
    }

    pub fn next(&self) -> Option<char> {
        self.src[self.pos.index+1..].chars().next()
    }

    pub fn prev(&self) -> Option<char> {
        if self.pos.index > 0 {
            return Some(self.src[self.pos.index-1..].chars().next().unwrap());
        }

        None
    }

    pub fn current(&self) -> Option<char> {
        self.src[self.pos.index..].chars().next()
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
