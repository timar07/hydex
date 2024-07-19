use crate::md_ast::Node;
use crate::md_lex::Cursor;

use super::enclosured::Enclosured;
use super::inline::InlineParser;
use super::parser::Parsable;
use super::element::{
    NormalTextParserEscaped,
    NormalTextParserUnescaped
};

/// Parses emphasis
///
/// ```bnf
/// emphasis = bold
///            | italic
///            | highlight
///            | strikethrough
///            | code
///            | normal_text
/// ```
pub struct EmphasisParser<'src, 'a> {
    src: &'a mut Cursor<'src>
}

impl<'src, 'a> EmphasisParser<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }

    fn parse_code(&mut self) -> Node {
        let enclosure = if self.src.check_next('`') { "``" } else { "`" };

        self.parse_emphasis(
            enclosure,
            |content| {
                Node::Normal(
                    content
                        .trim_start_matches(' ')
                        .trim_end_matches(' ')
                        .clone()
                        .chars()
                        .map(|c| match c {
                            '\n' => ' ',
                            _ => c
                        })
                        .collect::<String>()
                )
            },
            Node::Code
        )
    }

    fn parse_nested_emphasis<F: FnOnce(Box<Node>) -> Node>(
        &mut self,
        enclosure: &'static str,
        result_constructor: F
    ) -> Node {
        self.parse_emphasis(
            enclosure,
            |content| {
                InlineParser::new(
                    &mut Cursor::from_string(content)
                ).parse()
            },
            result_constructor
        )
    }

    fn parse_emphasis<F: FnOnce(Box<Node>) -> Node>(
        &mut self,
        enclosure: &'static str,
        content_parser: fn(&str) -> Node,
        result_constructor: F
    ) -> Node {
        let mut parser = Enclosured::new(
            self.src,
            enclosure,
            enclosure,
            content_parser
        );

        if parser.is_enclosured() {
            result_constructor(Box::new(parser.parse()))
        } else {
            self.src.match_curr(enclosure);
            Node::Normal(enclosure.to_string())
        }
    }
}

impl<'src, 'a> Parsable for EmphasisParser<'src, 'a> {
    fn parse(&mut self) -> Node {
        match self.src.current().unwrap() {
            '*' => {
                if self.src.check_next('*') {
                    self.parse_nested_emphasis("**", Node::Bold)
                } else {
                    self.parse_nested_emphasis("*", Node::Italic)
                }
            },
            '_' => {
                if self.src.check_next('_') {
                    self.parse_nested_emphasis("__", Node::Bold)
                } else {
                    self.parse_nested_emphasis("_", Node::Italic)
                }
            },
            '=' => self.parse_nested_emphasis("=", Node::Highlight),
            '~' => {
                if self.src.check_next('~') {
                    self.parse_nested_emphasis("~~", Node::Strikethrough)
                } else {
                    NormalTextParserEscaped::new(self.src).parse()
                }
            }
            '`' => {
                self.parse_code()
            },
            _ => NormalTextParserUnescaped::new(self.src).parse()
        }
    }
}
