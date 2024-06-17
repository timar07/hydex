use super::cursor::Cursor;
use super::node::Node;
use super::span::SpanParser;
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
        TreeOptimizer::optimize(
            self.parse_text_run()
        )
    }

    fn parse_text_run(&mut self) -> Node {
        let mut spans = vec![];

        while !self.src.is_eof() {
            spans.push(
                SpanParser::new(&mut self.src).parse()
            );
        }

        Node::TextRun(spans)
    }
}

pub trait Parsable {
    fn parse(&mut self) -> Node;
}
