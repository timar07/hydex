use crate::md_parse::inline::InlineParser;
use crate::md_ast::Node;

use super::span::SpanParser;
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

    fn parse_paragraph(&mut self) -> Node {
        let mut nodes = vec![];

        while !self.src.is_eof() {
            self.src.match_curr("\n\n");

            nodes.push(SpanParser::new(
                &mut Cursor::from_string(self.src.consume_line())
            ).parse());

            if self.src.check_curr("\n\n") {
                dbg!(self.src.slice(self.src.pos.index, self.src.len()));
                break;
            }
        }

        Node::Paragraph(nodes)
    }
}

impl<'src, 'a> Parsable for BlockParser<'src, 'a> {
    fn parse(&mut self) -> Node {
        match self.src.current().unwrap() {
            '#' => self.parse_heading(),
            '\n' => {
                if self.src.check_next('\n') {
                    self.parse_paragraph()
                } else {
                    SpanParser::new(&mut self.src).parse()
                }
            }
            _ if self.src.is_start() => {
                Node::Paragraph(vec![
                    InlineParser::new(
                        &mut Cursor::from_string(self.src.consume_line())
                    ).parse()
                ])
            },
            _ => {
                if self.src.lookahead("\n\n") {
                    self.parse_paragraph()
                } else {
                    SpanParser::new(&mut self.src).parse()
                }
            }
        }
    }
}
