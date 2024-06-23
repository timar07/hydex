use crate::md_ast::Node;
use crate::md_lex::Cursor;

use super::element::NormalTextParserEscaped;
use super::emphasis::EmphasisParser;
use super::parser::Parsable;

/// Parse span including text
///
/// ```bnf
/// span = link | emphasis;
/// ```
pub struct SpanParser<'src, 'a> {
    pub src: &'a mut Cursor<'src>
}

impl<'src, 'a> SpanParser<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }
}

impl<'src, 'a> Parsable for SpanParser<'src, 'a> {
    fn parse(&mut self) -> Node {
        match self.src.current().unwrap() {
            '[' => {
                // FIXME: Doesn't work well if it's `[<..>])(`
                if self.src.lookahead_inline("(") && self.src.lookahead_inline(")") {
                    self.parse_link()
                } else {
                    NormalTextParserEscaped::new(self.src).parse()
                }
            },
            _ => EmphasisParser::new(self.src).parse()
        }
    }
}
