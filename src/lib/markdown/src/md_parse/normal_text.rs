use super::{cursor::Cursor, parser::Parsable, Node};

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
        Self::is_normal_char(self.src.current())
        || self.src.prev().is_some_and(|c| c == '\\')
    }


    fn is_normal_char(ch: char) -> bool {
        match ch {
            '!' | '`' | '*' | '_' |
            '{' | '}' | '[' | ']' |
            '<' | '>' | '(' | ')' |
            '#' | '+' | '|' | '\\' => false,
            _ => true
        }
    }
}

impl<'a, 'b> Parsable for NormalTextParserUnescaped<'a, 'b> {
    fn parse(&mut self) -> Node {
        let start = self.src.pos.index;

        while !self.src.is_eof() && self.is_normal_or_escaped() {
            dbg!(self.src.current());
            self.src.consume();
        }

        Node::Normal(self.src[start..self.src.pos.index].to_owned())
    }
}
