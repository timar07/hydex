use super::{
    cursor::Cursor, node::Node, normal_text::NormalTextParserUnescaped, parser::Parsable
};

pub struct Enclosured<'src, 'a> {
    src: &'a mut Cursor<'src>,
    lhs: &'static str,
    rhs: &'static str,
    content_parser: fn(&'src str) -> Node
}

impl<'src, 'a> Enclosured<'src, 'a> {
    pub fn new(
        src: &'a mut Cursor<'src>,
        lhs: &'static str,
        rhs: &'static str,
        content_parser: fn(&'src str) -> Node
    ) -> Self {
        Self { src, lhs, rhs, content_parser }
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
}

impl<'src, 'a> Parsable for Enclosured<'src, 'a> {
    fn parse(&mut self) -> Node
    {
        self.src.match_curr(self.lhs);

        if !self.src.lookahead(self.rhs) {
            return NormalTextParserUnescaped::new(self.src).parse();
        }

        let start = self.src.pos.index;
        self.consume_until(self.rhs);

        (self.content_parser)(
            self.src.slice(
                start,
                self.src.pos.index-self.rhs.len()
            ).into()
        )
    }
}
