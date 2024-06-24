use crate::md_ast::Node;
use crate::md_parse::block::BlockParser;
use crate::md_parse::Parser;

impl BlockParser<'_, '_> {
    /// ```ebnf
    /// ulist = ( ( "-" | "*" ) WHITESPACE block NEWLINE )*;
    /// ```
    pub fn parse_unordered_list(&mut self) -> Node {
        let mut nodes = vec![];

        while self.src.match_curr("- ") || self.src.match_curr("* ") {
            nodes.push(
                Parser::from_string(self.src.consume_line()).parse()
            );

            self.src.consume(); // \n

            while self.src.current().is_some_and(|c| c.is_whitespace() || c == '\n') {
                self.src.consume();
            }
        }

        Node::UnorderedList(nodes)
    }
}

