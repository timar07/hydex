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
            '>' => self.parse_blockquote(),
            '-' | '*' | '+' => {
                if self.src.check_next(' ') {
                    self.parse_unordered_list()
                } else {
                    self.parse_paragraph()
                }
            }
            '\n' => {
                if self.src.check_next('\n') {
                    self.src.match_curr("\n\n");
                    self.parse_paragraph()
                } else {
                    SpanParser::new(&mut self.src).parse()
                }
            },
            '0'..='9' => self.parse_ordered_list(),
            _ => {
                self.parse_paragraph()
            }
        }
    }
}
