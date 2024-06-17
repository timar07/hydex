use super::cursor::Cursor;
use super::emphasis::EmphasisParser;
use super::enclosured::Enclosured;
use super::normal_text::NormalTextParserEscaped;
use super::parser::Parsable;
use super::Node;

pub struct SpanParser<'src, 'a> {
    src: &'a mut Cursor<'src>
}

impl<'src, 'a> SpanParser<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }

    fn parse_link(&mut self) -> Node {
        let label = Enclosured::new(
            self.src,
            "[",
            "]",
            |inner| {
                NormalTextParserEscaped::new(
                    &mut Cursor::from_string(inner)
                ).parse()
            }
        ).parse();

        let url = Enclosured::new(
            self.src,
            "(",
            ")",
            |inner| {
                NormalTextParserEscaped::new(
                    &mut Cursor::from_string(inner)
                ).parse()
            }
        ).parse();

        Node::Link {
            label: label.into(),
            url: url.into()
        }
    }
}

impl<'src, 'a> Parsable for SpanParser<'src, 'a> {
    fn parse(&mut self) -> Node {
        match self.src.current().unwrap() {
            '[' => {
                // FIXME: Doesn't work well if it's `[<..>])(`
                if self.src.lookahead("(") && self.src.lookahead(")") {
                    self.parse_link()
                } else {
                    NormalTextParserEscaped::new(self.src).parse()
                }
            },
            _ => EmphasisParser::new(self.src).parse()
        }
    }
}
