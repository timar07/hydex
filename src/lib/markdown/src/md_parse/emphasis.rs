use super::{cursor::Cursor, enclosured::Enclosured, normal_text::{NormalTextParserEscaped, NormalTextParserUnescaped}, parser::Parsable, Node, Parser};

pub struct EmphasisParser<'src, 'a> {
    src: &'a mut Cursor<'src>
}

impl<'src, 'a> EmphasisParser<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }

    fn parse_code(&mut self) -> Node {
        let enclosure = if self.src.check_next('`') { "``" } else { "`" };

        Enclosured::new(
            self.src,
            &enclosure,
            |content| {
                dbg!(content);
                NormalTextParserEscaped::new(
                    &mut Cursor::from_string(content)
                ).parse()
            }
        ).parse()
    }

    fn parse_enclosured_nested<T>(
        &mut self,
        enclosure: &'static str,
        result_constructor: T
    ) -> Node
    where
        T: Fn(Box<Node>) -> Node
    {
        result_constructor(
            Box::new(
                Enclosured::new(
                    self.src,
                    enclosure,
                    |content| {
                        Parser::from_string(content).parse()
                    }
                ).parse()
            )
        )
    }
}

impl<'src, 'a> Parsable for EmphasisParser<'src, 'a> {
    fn parse(&mut self) -> Node {
        match self.src.current() {
            '*' => {
                if self.src.check_next('*') {
                    self.parse_enclosured_nested(
                        "**",
                        |inner| {
                            Node::Bold(inner)
                        }
                    )
                } else {
                    self.parse_enclosured_nested(
                        "*",
                        |inner| {
                            Node::Italic(inner)
                        }
                    )
                }
            },
            '_' => {
                if self.src.check_next('_') {
                    self.parse_enclosured_nested(
                        "__",
                        |inner| {
                            Node::Bold(inner)
                        }
                    )
                } else {
                    self.parse_enclosured_nested(
                        "_",
                        |inner| {
                            Node::Italic(inner)
                        }
                    )
                }
            },
            '=' => self.parse_enclosured_nested(
                "=",
                |inner| {
                    Node::Highlight(inner)
                }
            ),
            '`' => {
                Node::Code(Box::new(self.parse_code()))
            },
            '\\' => {
                self.src.consume();
                NormalTextParserEscaped::new(self.src).parse()
            },
            _ => NormalTextParserUnescaped::new(self.src).parse()
        }
    }
}
