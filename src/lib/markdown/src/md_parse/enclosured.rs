use super::{
    cursor::Cursor, node::Node, normal_text::NormalTextParserUnescaped, parser::Parsable
};

pub struct Enclosured<'src, 'a> {
    src: &'a mut Cursor<'src>,
    enclosure: &'static str,
    content_parser: fn(&'src str) -> Node
}

impl<'src, 'a> Enclosured<'src, 'a> {
    pub fn new(
        src: &'a mut Cursor<'src>,
        enclosure: &'static str,
        content_parser: fn(&'src str) -> Node
    ) -> Self {
        Self { src, enclosure, content_parser }
    }

    fn consume_until(&mut self, enclosure: &'static str) {
        while !self.src.is_eof() {
            if self.src.match_curr(&enclosure[..1]) {
                if self.src.check_curr(enclosure) {
                    continue;
                }

                if enclosure.len() == 1 || self.src.match_curr(&enclosure[1..]) {
                    break;
                }

                continue;
            }

            self.src.consume();
        }
    }

    fn lookahead(&self, matcher: &'static str) -> bool {
        let mut i = self.src.pos.index;

        while i <= self.src.len() - matcher.len() && self.src.char_at(i) != '\n' {
            if &self.src[i..i + matcher.len()] == matcher {
                return true;
            }

            i += 1
        }

        false
    }
}

impl<'src, 'a> Parsable for Enclosured<'src, 'a> {
    fn parse(&mut self) -> Node
    {
        self.src.match_curr(self.enclosure);

        if !self.lookahead(self.enclosure) {
            return NormalTextParserUnescaped::new(self.src).parse();
        }

        let start = self.src.pos.index;
        self.consume_until(self.enclosure);

        (self.content_parser)(
            self.src.slice(
                start,
                self.src.pos.index-self.enclosure.len()
            ).into()
        )
    }
}
