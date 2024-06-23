use crate::md_ast::Node;
use crate::md_lex::Cursor;
use crate::md_parse::{
    block::BlockParser,
    inline::InlineParser,
    parser::Parsable
};

impl BlockParser<'_, '_> {
    /// ```bnf
    /// heading = ( "#" )* inline;
    /// ```
    pub fn parse_heading(&mut self) -> Node {
        let mut level = 0;

        while self.src.match_curr("#") {
            level += 1;
        }

        self.src.match_curr(" "); // TODO: Add pedantic warning

        Node::Heading(
            level,
            Box::new(
                InlineParser::new(
                    &mut Cursor::from_string(
                        self.src.consume_line()
                    )
                ).parse()
            )
        )
    }
}

