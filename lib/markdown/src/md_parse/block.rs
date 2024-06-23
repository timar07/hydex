use crate::md_lex::Cursor;
use crate::md_ast::Node;

use super::span::SpanParser;
use super::parser::Parsable;

pub struct BlockParser<'src, 'a> {
    pub src: &'a mut Cursor<'src>
}

impl<'src, 'a> BlockParser<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }
}

impl<'src, 'a> Parsable for BlockParser<'src, 'a> {
    fn parse(&mut self) -> Node {
        match self.src.current().unwrap() {
            '#' => self.parse_heading(),
            _ => SpanParser::new(&mut self.src).parse()
        }
    }
}
