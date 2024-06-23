use crate::md_ast::Node;
use crate::md_lex::Cursor;

use super::parser::Parsable;

pub struct NormalTextParserEscaped<'src, 'a> {
    src: &'a mut Cursor<'src>
}

impl<'src, 'a> NormalTextParserEscaped<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }
}

impl<'src, 'a> Parsable for NormalTextParserEscaped<'src, 'a> {
    fn parse(&mut self) -> Node {
        let start = self.src.pos.index;

        while !self.src.is_eof() {
            self.src.consume();
        }

        Node::Normal(self.src[start..self.src.pos.index].to_owned())
    }
}

pub struct NormalTextParserUnescaped<'src, 'a> {
    src: &'a mut Cursor<'src>
}

impl<'src, 'a> NormalTextParserUnescaped<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }

    fn is_normal_or_escaped(&self) -> bool {
        self.src.current().is_some_and(|c| Self::is_normal_char(c))
        || self.src.prev().is_some_and(|c| c == '\\')
    }


    fn is_normal_char(ch: char) -> bool {
        match ch {
            '`' | '*' | '_' | '{' |
            '}' | '[' | ']' | '<' |
            '>' | '(' | ')' | '+' |
            '|' | '\\' => false,
            _ => true
        }
    }
}

impl<'a, 'b> Parsable for NormalTextParserUnescaped<'a, 'b> {
    fn parse(&mut self) -> Node {
        let start = self.src.pos.index;

        while !self.src.is_eof() && self.is_normal_or_escaped() {
            self.src.consume();
        }

        Node::Normal(self.src[start..self.src.pos.index].to_owned())
    }
}
