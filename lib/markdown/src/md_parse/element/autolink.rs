use crate::md_ast::Node;
use crate::md_parse::span::SpanParser;

impl SpanParser<'_, '_> {
    /// ```bnf
    /// autolink = "<" STRING ">";
    /// ```
    pub fn parse_autolink(&mut self) -> Node {
        self.src.consume(); // <

        // TODO: Validate URL
        let url = self.src.consume_until(">");

        Node::Link {
            label: url.into(),
            url: url.into()
        }
    }
}