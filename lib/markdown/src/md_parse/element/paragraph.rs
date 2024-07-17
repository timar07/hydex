use crate::md_ast::Node;
use crate::md_lex::Cursor;
use crate::md_parse::{
    block::BlockParser,
    inline::InlineParser,
    parser::Parsable
};

impl BlockParser<'_, '_> {
    pub fn parse_paragraph(&mut self) -> Node {
        let mut paragraph_content = String::new();

        while !self.src.is_eof() {
            paragraph_content.push_str(self.src.consume_line().trim_start());

            if self.src.check_curr("\n") {
                self.skip_blank();
                break;
            }
        }

        Node::Paragraph(vec![
            InlineParser::new(
                &mut Cursor::from_string(&paragraph_content.trim())
            ).parse()
        ])
    }

    fn skip_blank(&mut self) {
        let mut line_start = self.src.pos;

        while !self.src.is_eof() {
            match self.src.current().unwrap() {
                '\n' => {
                    self.src.consume();
                    line_start = self.src.pos;
                },
                c if c.is_whitespace() => {
                    self.src.consume();
                },
                _ => {
                    self.src.pos = line_start; // go to line start
                    break;
                }
            }
        }
    }
}
