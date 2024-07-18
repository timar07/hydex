use crate::md_ast::Node;
use crate::md_lex::Cursor;

use super::emphasis::EmphasisParser;
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
            let node = match self.src.current().unwrap() {
                ' ' => {
                    if self.src.match_curr("  \n") {
                        Node::Linebreak
                    } else {
                        EmphasisParser::new(self.src).parse()
                    }
                },
                _ => SpanParser::new(&mut self.src).parse()
            };

            nodes.push(node);
        }

        Node::TextRun(nodes)
    }
}
