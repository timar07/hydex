use crate::md_parse::inline::InlineParser;

use super::span::SpanParser;
use super::Node;
use super::cursor::Cursor;
use super::parser::Parsable;

pub struct BlockParser<'src, 'a> {
    src: &'a mut Cursor<'src>
}

impl<'src, 'a> BlockParser<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }

    fn parse_heading(&mut self) -> Node {
        let mut level = 0;

        while self.src.match_curr("#") {
            level += 1;
        }

        self.src.match_curr(" ");

        Node::Heading(
            level,
            Box::new(
                InlineParser::new(
                    &mut Cursor::from_string(
                        self.src.consume_line()
                    )
                ).parse()
            )
        )
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
