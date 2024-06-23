use crate::md_ast::Node;
use crate::md_lex::Cursor;

use super::block::BlockParser;
use super::tree_optimizer::TreeOptimizer;

pub struct Parser<'src> {
    src: Cursor<'src>,
}

impl<'a> Parser<'a> {
    pub fn from_string(s: &'a str) -> Parser<'a> {
        return Self {
            src: Cursor::from_string(s)
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut nodes = vec![];

        while !self.src.is_eof() {
            nodes.push(
                BlockParser::new(&mut self.src).parse()
            );
        }

        TreeOptimizer::optimize(Node::TextRun(nodes))
    }
}

pub trait Parsable {
    fn parse(&mut self) -> Node;
}
