use crate::md_ast::Node;
use crate::md_lex::Cursor;
use crate::md_parse::{
    block::BlockParser,
    inline::InlineParser,
    parser::Parsable
};

impl BlockParser<'_, '_> {
    pub fn parse_paragraph(&mut self) -> Node {
        let mut paragraph_content = vec![];

        while !self.src.is_eof() {
            paragraph_content.push(self.src.consume_line());
            self.src.match_curr("\n");

            if self.src.match_curr("\n") {
                break;
            }
        }

        Node::Paragraph(vec![
            InlineParser::new(
                &mut Cursor::from_string(dbg!(&paragraph_content.join(" ")))
            ).parse()
        ])
    }
}
