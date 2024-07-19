use crate::md_ast::Node;
use crate::md_lex::Cursor;

use super::parser::Parsable;

/// Parses fenced syntax.
///
/// ```bnf
/// fenced = lhs content rhs;
/// ```
pub struct Fenced<'src, 'a> {
    src: &'a mut Cursor<'src>,
    lhs: &'static str,
    rhs: &'static str,
    content_parser: fn(&'src str) -> Node,
}

impl<'src, 'a> Fenced<'src, 'a> {
    pub fn new(
        src: &'a mut Cursor<'src>,
        lhs: &'static str,
        rhs: &'static str,
        content_parser: fn(&'src str) -> Node
    ) -> Self {
        Self { src, lhs, rhs, content_parser }
    }

    pub fn is_enclosured(&self) -> bool {
        let start = self.src.pos.index + self.lhs.len();
        let end = self.src.len() - self.rhs.len();

        for i in start..=end {
            if self.src.slice(i, i + self.rhs.len()) == self.rhs {
                return true;
            }
        }

        false
    }
}

impl<'src, 'a> Parsable for Fenced<'src, 'a> {
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
