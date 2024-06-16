use std::borrow::BorrowMut;

use super::cursor::Cursor;
use super::enclosured::Enclosured;
use super::node::Node;
use super::normal_text::{NormalTextParserEscaped, NormalTextParserUnescaped};
use super::tree_optimizer::TreeOptimizer;

pub struct Parser<'src> {
    src: Cursor<'src>,
}

impl<'a> Parser<'a> {
    pub fn from_string(s: &'a str) -> Parser<'a> {
        return Self {
            src: Cursor::from_string(s)
        }
    }

    pub fn parse(&mut self) -> Node {
        TreeOptimizer::optimize(
            self.parse_text_run()
        )
    }

    fn parse_text_run(&mut self) -> Node {
        let mut text = vec![];

        while !self.src.is_eof() {
            let token = match self.src.current() {
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
                    NormalTextParserEscaped::new(self.src.borrow_mut()).parse()
                }
                _ => NormalTextParserUnescaped::new(self.src.borrow_mut()).parse()
            };

            text.push(token);
        }

        Node::TextRun(text)
    }

    fn parse_code(&mut self) -> Node {
        let enclosure = if self.src.check_next('`') { "``" } else { "`" };

        Enclosured::new(
            self.src.borrow_mut(),
            &enclosure,
            |content| {
                NormalTextParserEscaped::new(
                    Cursor::from_string(content).borrow_mut()
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
                    self.src.borrow_mut(),
                    enclosure,
                    |content| {
                        Parser::from_string(content).parse()
                    }
                ).parse()
            )
        )
    }
}

pub trait Parsable {
    fn parse(&mut self) -> Node;
}
