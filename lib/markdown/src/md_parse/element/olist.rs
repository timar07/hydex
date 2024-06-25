use crate::md_ast::Node;
use crate::md_parse::block::BlockParser;
use crate::md_parse::Parser;

impl BlockParser<'_, '_> {
    /// ```ebnf
    /// olist = ( NUMBER "." WHITESPACE block NEWLINE )*;
    /// ```
    pub fn parse_ordered_list(&mut self) -> Node {
        if !self.match_ol_start() {
            return self.parse_paragraph();
        }

        let mut nodes = vec![];

        loop {
            nodes.push(
                Parser::from_string(self.src.consume_line()).parse()
            );

            self.src.consume(); // \n

            while self.src.current().is_some_and(|c| c.is_whitespace() || c == '\n') {
                self.src.consume();
            }

            if !self.match_ol_start() {
                break;
            }
        }

        Node::OrderedList(nodes)
    }

    fn match_ol_start(&mut self) -> bool {
        let is_ol_start = self.match_number() && self.src.match_curr(".");

        if is_ol_start {
            self.src.match_curr(" ");
            return true;
        }

        false
    }

    fn match_number(&mut self) -> bool {
        if !self.match_digit() {
            return false;
        }

        while self.match_digit() {}

        true
    }

    fn match_digit(&mut self) -> bool {
        match self.src.current() {
            Some('0'..='9') => {
                self.src.consume();
                true
            },
            _ => false
        }
    }
}

