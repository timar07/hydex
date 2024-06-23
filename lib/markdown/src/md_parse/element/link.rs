use crate::md_ast::Node;
use crate::md_lex::Cursor;
use crate::md_parse::parser::Parsable;
use crate::md_parse::span::SpanParser;
use crate::md_parse::enclosured::Enclosured;

use super::normal_text::NormalTextParserEscaped;

impl SpanParser<'_, '_> {
    /// ```bnf
    /// link = "[" label "](" url ( STRING )? ")";
    /// ```
    pub fn parse_link(&mut self) -> Node {
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