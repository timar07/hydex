use crate::md_lex::Cursor;
use crate::md_ast::Node;
use crate::md_parse::Parser;

use super::inline::InlineParser;
use super::span::SpanParser;
use super::parser::Parsable;

pub struct BlockParser<'src, 'a> {
    pub src: &'a mut Cursor<'src>
}

impl<'src, 'a> BlockParser<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }

    fn parse_paragraph(&mut self) -> Node {
        let mut paragraph_content = vec![];

        while !self.src.is_eof() {
            paragraph_content.push(self.src.consume_line());
            self.src.match_curr("\n");

            if self.src.match_curr("\n") {
                break;
            }
        }

        Node::Paragraph(vec![
            InlineParser::new(
                &mut Cursor::from_string(dbg!(&paragraph_content.join(" ")))
            ).parse()
        ])
    }

    fn parse_blockquote(&mut self) -> Node {
        let mut content = String::new();

        while self.src.match_curr(">") {
            self.src.match_curr(" ");
            content.push_str(dbg!(&self.src.consume_until("\n")));
            self.src.match_curr("\n");
        }

        dbg!(&content);

        Node::Blockquote(
            Box::new(
                Parser::from_string(
                    &content
                ).parse()
            )
        )
    }
}

impl<'src, 'a> Parsable for BlockParser<'src, 'a> {
    fn parse(&mut self) -> Node {
        match self.src.current().unwrap() {
            '#' => self.parse_heading(),
            '>' => self.parse_blockquote(),
            '\n' => {
                dbg!(self.src.slice(self.src.pos.index, self.src.len()));
                if self.src.check_next('\n') {
                    self.src.match_curr("\n\n");
                    self.parse_paragraph()
                } else {
                    SpanParser::new(&mut self.src).parse()
                }
            },
            _ => {
                self.parse_paragraph()
            }
        }
    }
}
