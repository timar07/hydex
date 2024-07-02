use crate::md_ast::Node;
use crate::md_lex::Cursor;

use super::parser::Parsable;

/// Parses enclosured syntax.
///
/// ```bnf
/// enclosured = lhs content rhs;
/// ```
pub struct Enclosured<'src, 'a> {
    src: &'a mut Cursor<'src>,
    lhs: &'static str,
    rhs: &'static str,
    content_parser: fn(&'src str) -> Node,
}

impl<'src, 'a> Enclosured<'src, 'a> {
    pub fn new(
        src: &'a mut Cursor<'src>,
        lhs: &'static str,
        rhs: &'static str,
        content_parser: fn(&'src str) -> Node
    ) -> Self {
        Self { src, lhs, rhs, content_parser }
    }

    pub fn is_enclosured(&self) -> bool {
        let mut i = self.src.pos.index + self.lhs.len();
        let end = self.src.len() - self.rhs.len();

        while i <= end && self.src.char_at(i).is_some_and(|c| c != '\n') {
            if self.src.slice(i, i + self.rhs.len()) == self.rhs {
                return true;
            }

            i += 1
        }

        false
    }
}

impl<'src, 'a> Parsable for Enclosured<'src, 'a> {
    fn parse(&mut self) -> Node {
        self.src.match_curr(self.lhs);

        let start = self.src.pos.index;
        self.src.consume_enclosured(self.rhs);

        (self.content_parser)(
            self.src.slice(
                start,
                self.src.pos.index-self.rhs.len()
            ).into()
        )
    }
}
