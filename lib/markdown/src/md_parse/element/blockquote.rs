use crate::md_ast::Node;
use crate::md_parse::Parser;
use crate::md_parse::block::BlockParser;

impl BlockParser<'_, '_> {
    /// ```ebnf
    /// blockquote = ( ">" ( WHITESPACE )? block NEWLINE )*;
    /// ```
    pub fn parse_blockquote(&mut self) -> Node {
        let mut content = String::new();

        while self.src.match_curr(">") {
            self.src.match_curr(" ");
            content.push_str(&self.src.consume_until("\n"));

            if self.src.match_curr("\n") {
                content.push_str("\n");
            }
        }

        Node::Blockquote(
            Box::new(
                Parser::from_string(
                    &content
                ).parse()
            )
        )
    }
}

