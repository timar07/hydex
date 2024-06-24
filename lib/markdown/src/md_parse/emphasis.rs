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

    fn parse_nested_emphasis<F: FnOnce(Box<Node>) -> Node>(
        &mut self,
        enclosure: &'static str,
        result_constructor: F
    ) -> Node {
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
