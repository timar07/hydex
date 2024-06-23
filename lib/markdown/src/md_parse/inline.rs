use crate::md_ast::Node;
use crate::md_lex::Cursor;

use super::parser::Parsable;
use super::span::SpanParser;

/// Parses inline text
/// ```bnf
/// inline = emphasis | span | normal_text
/// ```
pub struct InlineParser<'src, 'a> {
    src: &'a mut Cursor<'src>
}

impl<'src, 'a> InlineParser<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }
}

impl<'src, 'a> Parsable for InlineParser<'src, 'a> {
    fn parse(&mut self) -> Node {
        let mut nodes = vec![];

        while !self.src.is_eof() {
            nodes.push(
                SpanParser::new(&mut self.src).parse()
            );
        }

        Node::TextRun(nodes)
    }
}
