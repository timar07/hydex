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

    pub fn len(&self) -> usize {
        self.src.len()
    }

    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.src[start..end]
    }

    pub fn skip_n(&mut self, n: usize) {
        for _ in 0..n {
            self.consume();
        }
    }

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

    pub fn check_next(&mut self, ch: char) -> bool {
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
