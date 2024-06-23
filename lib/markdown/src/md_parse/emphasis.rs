use crate::md_ast::Node;
use crate::md_lex::Cursor;

use super::enclosured::Enclosured;
use super::inline::InlineParser;
use super::parser::Parsable;
use super::normal_text::{
    NormalTextParserEscaped,
    NormalTextParserUnescaped
};

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
            &enclosure,
            |content| {
                dbg!(content);
                NormalTextParserEscaped::new(
                    &mut Cursor::from_string(content)
                ).parse()
            }
        ).parse()
    }

    fn parse_nested_emphasis<T>(
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
                    enclosure,
                    |content| {
                        InlineParser::new(
                            &mut Cursor::from_string(content)
                        ).parse()
                    }
                ).parse()
            )
        )
    }
}

impl<'src, 'a> Parsable for EmphasisParser<'src, 'a> {
    fn parse(&mut self) -> Node {
        match self.src.current().unwrap() {
            '*' => {
                if self.src.check_next('*') {
                    self.parse_nested_emphasis(
                        "**",
                        |inner| {
                            Node::Bold(inner)
                        }
                    )
                } else {
                    self.parse_nested_emphasis(
                        "*",
                        |inner| {
                            Node::Italic(inner)
                        }
                    )
                }
            },
            '_' => {
                if self.src.check_next('_') {
                    self.parse_nested_emphasis(
                        "__",
                        |inner| {
                            Node::Bold(inner)
                        }
                    )
                } else {
                    self.parse_nested_emphasis(
                        "_",
                        |inner| {
                            Node::Italic(inner)
                        }
                    )
                }
            },
            '=' => self.parse_nested_emphasis(
                "=",
                |inner| {
                    Node::Highlight(inner)
                }
            ),
            '~' => {
                if self.src.check_next('~') {
                    self.parse_nested_emphasis(
                        "~~",
                        |inner| {
                            Node::Strikethrough(inner)
                        }
                    )
                } else {
                    NormalTextParserUnescaped::new(self.src).parse()
                }
            }
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
