use crate::md_ast::Node;
use super::cursor::Cursor;
use super::normal_text::NormalTextParserUnescaped;
use super::parser::Parsable;

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

        if !self.src.lookahead(self.rhs) {
            return NormalTextParserUnescaped::new(self.src).parse();
        }

        let start = self.src.pos.index;
        self.src.consume_until(self.rhs);

        (self.content_parser)(
            self.src.slice(
                start,
                self.src.pos.index-self.rhs.len()
            ).into()
        )
    }
}
