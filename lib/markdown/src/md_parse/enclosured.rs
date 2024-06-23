use crate::md_ast::Node;
use crate::md_lex::Cursor;

use crate::md_parse::element::NormalTextParserUnescaped;
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
    content_parser: fn(&'src str) -> Node
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
}

impl<'src, 'a> Parsable for Enclosured<'src, 'a> {
    fn parse(&mut self) -> Node {
        self.src.match_curr(self.lhs);

        if !self.src.lookahead_inline(self.rhs) {
            return NormalTextParserUnescaped::new(self.src).parse();
        }

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
