use crate::md_ast::Node;
use crate::md_lex::Cursor;
use crate::md_parse::parser::Parsable;
use crate::md_parse::span::SpanParser;
use crate::md_parse::fenced::Fenced;

use super::normal_text::NormalTextParserEscaped;

impl SpanParser<'_, '_> {
    /// ```bnf
    /// link = "[" label "](" url ( STRING )? ")";
    /// ```
    pub fn parse_link(&mut self) -> Node {
        let label = Fenced::new(
            self.src,
            "[",
            "]",
            |inner| {
                NormalTextParserEscaped::new(
                    &mut Cursor::from_string(inner)
                ).parse()
            }
        ).parse();

        let url = Fenced::new(
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